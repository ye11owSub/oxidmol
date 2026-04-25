use crate::core::element::Element;
use crate::utils::LoadError;
use glam::Vec3;
use pdbtbx::StrictnessLevel;

// Needed for FlatAtom::get_instance_layout
use wgpu;

#[derive(Debug, Clone)]
pub struct Atom {
    pub serial: u32,
    pub name: String,
    pub element: Element,
    pub position: Vec3,
    pub b_factor: f32,
    pub occupancy: f32,
    pub het: bool,
}

/// Compact GPU-ready atom representation.
/// Memory layout: position=0 (12 B), radius=12 (4 B), color=16 (12 B) — stride 28 B.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FlatAtom {
    pub position: Vec3,  // offset  0
    pub radius: f32,     // offset 12
    pub color: [f32; 3], // offset 16
}

impl FlatAtom {
    pub fn get_instance_layout() -> wgpu::VertexBufferLayout<'static> {
        const ATTRIBUTES: [wgpu::VertexAttribute; 3] = wgpu::vertex_attr_array![
            1 => Float32x3, // center
            2 => Float32,   // radius
            3 => Float32x3, // color
        ];
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<FlatAtom>() as u64,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &ATTRIBUTES,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ResidueKind {
    AminoAcid,
    Nucleotide,
    Ligand,
    Water,
    Unknown,
}

#[derive(Debug, Clone)]
pub struct Residue {
    pub seq_num: i32,
    pub name: String,
    pub kind: ResidueKind,
    pub atoms: Vec<Atom>,
}

impl Residue {
    pub fn ca(&self) -> Option<&Atom> {
        self.atoms.iter().find(|a| a.name == "CA")
    }
}

#[derive(Debug, Clone)]
pub struct Chain {
    pub id: char,
    pub residues: Vec<Residue>,
}

impl Chain {
    pub fn ca_trace(&self) -> impl Iterator<Item = &Atom> {
        self.residues
            .iter()
            .filter(|r| r.kind == ResidueKind::AminoAcid)
            .filter_map(|r| r.ca())
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Bond {
    pub a: u32,
    pub b: u32,
}

#[derive(Debug)]
pub struct Molecule {
    pub name: Option<String>,
    pub chains: Vec<Chain>,
    pub bonds: Vec<Bond>,
    pub atoms_flat: Vec<FlatAtom>,
}

impl Molecule {
    pub fn new(name: Option<String>, chains: Vec<Chain>, bonds: Vec<Bond>) -> Self {
        let atoms_flat = Self::build_flat(&chains);
        Self {
            name,
            chains,
            bonds,
            atoms_flat,
        }
    }

    pub fn build_flat(chains: &[Chain]) -> Vec<FlatAtom> {
        chains
            .iter()
            .flat_map(|c| &c.residues)
            .flat_map(|r| &r.atoms)
            .map(|a| FlatAtom {
                position: a.position,
                radius: a.element.vdw_radius(),
                color: a.element.cpk_color(),
            })
            .collect()
    }

    pub fn classify(name: &str, has_het: bool) -> ResidueKind {
        if name == "HOH" || name == "WAT" {
            return ResidueKind::Water;
        }
        if has_het {
            return ResidueKind::Ligand;
        }
        const AA: &[&str] = &[
            "ALA", "ARG", "ASN", "ASP", "CYS", "GLN", "GLU", "GLY", "HIS", "ILE", "LEU", "LYS",
            "MET", "PHE", "PRO", "SER", "THR", "TRP", "TYR", "VAL",
        ];
        const NA: &[&str] = &["DA", "DC", "DG", "DT", "A", "C", "G", "U"];

        if AA.contains(&name) {
            ResidueKind::AminoAcid
        } else if NA.contains(&name) {
            ResidueKind::Nucleotide
        } else {
            ResidueKind::Unknown
        }
    }

    pub fn load(path: &str) -> Result<Molecule, LoadError> {
        let (pdb, _errors) = pdbtbx::open(path, StrictnessLevel::Loose)
            .map_err(|e| LoadError::Pdb(format!("{e:?}")))?;
        Self::from_pdb(pdb)
    }

    pub fn load_str(content: &str) -> Result<Molecule, LoadError> {
        use std::io::{BufReader, Cursor};
        let reader = BufReader::new(Cursor::new(content.as_bytes().to_vec()));
        let (pdb, _errors) = pdbtbx::open_raw(reader, StrictnessLevel::Loose)
            .map_err(|e| LoadError::Pdb(format!("{e:?}")))?;
        Self::from_pdb(pdb)
    }

    fn from_pdb(pdb: pdbtbx::PDB) -> Result<Molecule, LoadError> {
        let chains = pdb
            .chains()
            .map(|c| {
                let id = c.id().chars().next().unwrap_or('A');

                let residues = c
                    .residues()
                    .map(|r| {
                        let name = r.name().unwrap_or("UNK").to_string();

                        let kind = Self::classify(&name, r.atoms().any(|a| a.hetero()));

                        let atoms = r
                            .atoms()
                            .map(|a| {
                                let (x, y, z) = a.pos();
                                Atom {
                                    serial: a.serial_number() as u32,
                                    name: a.name().to_string(),
                                    element: a
                                        .element()
                                        .and_then(|e| e.symbol().parse::<Element>().ok())
                                        .unwrap_or(Element::Unknown),
                                    position: glam::Vec3::new(x as f32, y as f32, z as f32),
                                    b_factor: a.b_factor() as f32,
                                    occupancy: a.occupancy() as f32,
                                    het: a.hetero(),
                                }
                            })
                            .collect();

                        Residue {
                            seq_num: r.serial_number() as i32,
                            name,
                            kind,
                            atoms,
                        }
                    })
                    .collect();

                Chain { id, residues }
            })
            .collect::<Vec<_>>();

        let bonds = pdb
            .bonds()
            .map(|(a, b, _)| Bond {
                a: a.serial_number() as u32,
                b: b.serial_number() as u32,
            })
            .collect();

        let name = pdb
            .identifier
            .filter(|s| !s.trim().is_empty())
            .map(|s| s.trim().to_string());

        Ok(Molecule::new(name, chains, bonds))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const MINIMAL_PDB: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/minimal.pdb");
    const MINIMAL_CIF: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/minimal.cif");

    #[test]
    fn test_load_pdb_ok() {
        let mol = Molecule::load(MINIMAL_PDB).unwrap();

        assert_eq!(mol.name.clone().unwrap(), "MINI".to_string());

        assert_eq!(mol.chains.len(), 1);
        assert_eq!(mol.chains[0].id, 'A');

        // ALA + HOH
        assert_eq!(mol.chains[0].residues.len(), 2);

        let residues = &mol.chains[0].residues;
        assert_eq!(residues[0].kind, ResidueKind::AminoAcid);
        assert_eq!(residues[1].kind, ResidueKind::Water);

        assert_eq!(mol.atoms_flat.len(), 5);
    }

    #[test]
    fn test_load_crambin() {
        const CRN_PDB: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/1crn.pdb");
        let mol = Molecule::load(CRN_PDB).unwrap();

        assert_eq!(mol.atoms_flat.len(), 327);
        assert_eq!(mol.bonds.len(), 3);
        assert!(!mol.chains.is_empty());

        assert_eq!(mol.chains[0].id, 'A');

        let ca_count = mol.chains[0].ca_trace().count();
        assert_eq!(ca_count, 46);
    }

    #[test]
    fn test_classify() {
        assert_eq!(Molecule::classify("ALA", false), ResidueKind::AminoAcid);
        assert_eq!(Molecule::classify("GLY", false), ResidueKind::AminoAcid);

        assert_eq!(Molecule::classify("HOH", false), ResidueKind::Water);
        assert_eq!(Molecule::classify("WAT", false), ResidueKind::Water);

        assert_eq!(Molecule::classify("LIG", true), ResidueKind::Ligand);

        assert_eq!(Molecule::classify("DA", false), ResidueKind::Nucleotide);
    }

    #[test]
    fn test_load_mmcif_ok() {
        let mol = Molecule::load(MINIMAL_CIF).unwrap();

        assert_eq!(mol.chains.len(), 1);
        assert_eq!(mol.chains[0].id, 'A');

        assert_eq!(mol.chains[0].residues.len(), 2);

        let residues = &mol.chains[0].residues;
        assert_eq!(residues[0].kind, ResidueKind::AminoAcid);
        assert_eq!(residues[1].kind, ResidueKind::Water);

        assert_eq!(mol.atoms_flat.len(), 5);
    }

    #[test]
    fn test_load_pdb_nonexistent() {
        let err = Molecule::load("nonexistent.pdb").unwrap_err();
        assert!(matches!(err, LoadError::Pdb(_)));
    }

    #[test]
    fn test_load_str_cif() {
        let cif = std::fs::read_to_string(MINIMAL_CIF).unwrap();
        let mol = Molecule::load_str(&cif).unwrap();

        assert_eq!(mol.chains.len(), 1);
        assert_eq!(mol.chains[0].id, 'A');
        assert_eq!(mol.chains[0].residues.len(), 2);

        let residues = &mol.chains[0].residues;
        assert_eq!(residues[0].kind, ResidueKind::AminoAcid);
        assert_eq!(residues[1].kind, ResidueKind::Water);
        assert_eq!(mol.atoms_flat.len(), 5);
    }

    #[test]
    fn test_load_str_invalid() {
        let err = Molecule::load_str("not a valid molecule format").unwrap_err();
        assert!(matches!(err, LoadError::Pdb(_)));
    }
}
