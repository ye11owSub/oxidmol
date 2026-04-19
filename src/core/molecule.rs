use crate::core::element::Element;
use glam::Vec3;

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

#[derive(Debug, Clone, Copy)]
pub struct FlatAtom {
    pub position: Vec3,
    pub radius: f32,
    pub color: [f32; 3],
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
}
