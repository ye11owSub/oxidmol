use crate::core::element::Element;
use crate::core::molecule::*;
use crate::utils::LoadError;
use pdbtbx::StrictnessLevel;

fn classify(name: &str, has_het: bool) -> ResidueKind {
    if name == "HOH" || name == "WAT" {
        return ResidueKind::Water;
    }
    if has_het {
        return ResidueKind::Ligand;
    }
    const AA: &[&str] = &[
        "ALA", "ARG", "ASN", "ASP", "CYS", "GLN", "GLU", "GLY", "HIS", "ILE", "LEU", "LYS", "MET",
        "PHE", "PRO", "SER", "THR", "TRP", "TYR", "VAL",
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

pub fn load_pdb(path: &str) -> Result<Molecule, LoadError> {
    let (pdb, _errors) =
        pdbtbx::open(path, StrictnessLevel::Loose).map_err(|e| LoadError::Pdb(format!("{e:?}")))?;

    let chains = pdb
        .chains()
        .map(|c| {
            let id = c.id().chars().next().unwrap_or('A');

            let residues = c
                .residues()
                .map(|r| {
                    let name = r.name().unwrap_or("UNK").to_string();

                    let kind = classify(&name, r.atoms().any(|a| a.hetero()));

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

#[cfg(test)]
mod tests {
    use super::*;

    const MINIMAL_PDB: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/minimal.pdb");

    #[test]
    fn test_load_pdb_ok() {
        let mol = load_pdb(MINIMAL_PDB).unwrap();

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
        let mol = load_pdb(CRN_PDB).unwrap();

        assert_eq!(mol.atoms_flat.len(), 327);
        assert_eq!(mol.bonds.len(), 3);
        assert!(!mol.chains.is_empty());

        assert_eq!(mol.chains[0].id, 'A');

        let ca_count = mol.chains[0].ca_trace().count();
        assert_eq!(ca_count, 46);
    }

    #[test]
    fn test_classify() {
        assert_eq!(classify("ALA", false), ResidueKind::AminoAcid);
        assert_eq!(classify("GLY", false), ResidueKind::AminoAcid);

        assert_eq!(classify("HOH", false), ResidueKind::Water);
        assert_eq!(classify("WAT", false), ResidueKind::Water);

        assert_eq!(classify("LIG", true), ResidueKind::Ligand);

        assert_eq!(classify("DA", false), ResidueKind::Nucleotide);
    }

    #[test]
    fn test_load_pdb_nonexistent() {
        let err = load_pdb("nonexistent.pdb").unwrap_err();
        assert!(matches!(err, LoadError::Pdb(_)));
    }
}
