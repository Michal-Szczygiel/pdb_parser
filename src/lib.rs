use std::fmt::Write;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use pyo3::prelude::*;
use serde::Serialize;
use serde_json;

#[pyclass]
#[derive(Clone, Serialize)]
struct Atom {
    #[pyo3(set, get)]
    name: String,

    #[pyo3(set, get)]
    pos_x: f32,

    #[pyo3(set, get)]
    pos_y: f32,

    #[pyo3(set, get)]
    pos_z: f32,
}

#[pymethods]
impl Atom {
    #[new]
    fn new(name: String, pos_x: f32, pos_y: f32, pos_z: f32) -> Atom {
        return Atom {
            name,
            pos_x,
            pos_y,
            pos_z,
        };
    }

    fn __repr__(&self) -> String {
        return format!(
            "Atom: \"{}\", {{{}, {}, {}}}",
            self.name, self.pos_x, self.pos_y, self.pos_z
        );
    }
}

#[pyclass]
#[derive(Clone, Serialize)]
struct Residue {
    #[pyo3(set, get)]
    name: String,

    #[pyo3(set, get)]
    sequence_number: u64,

    #[pyo3(set, get)]
    atoms: Vec<Atom>,
}

#[pymethods]
impl Residue {
    #[new]
    fn new(name: String, sequence_number: u64, atoms: Vec<Atom>) -> Residue {
        return Residue {
            name,
            sequence_number,
            atoms,
        };
    }

    fn __repr__(&self) -> String {
        let mut buffer = format!("Residue: \"{}\"\n", self.name);

        for atom in self.atoms.iter() {
            write!(buffer, "\t{}\n", atom.__repr__()).unwrap();
        }

        return buffer;
    }
}

#[pyclass]
#[derive(Clone, Serialize)]
struct Chain {
    #[pyo3(set, get)]
    name: String,

    #[pyo3(set, get)]
    residues: Vec<Residue>,
}

#[pymethods]
impl Chain {
    #[new]
    fn new(name: String, residues: Vec<Residue>) -> Chain {
        return Chain { name, residues };
    }

    fn __repr__(&self) -> String {
        let mut buffer = format!("Chain: \"{}\"\n", self.name);

        for residue in self.residues.iter() {
            write!(buffer, "\t{}\n", residue.name).unwrap();
        }

        return buffer;
    }

    fn __len__(&self) -> usize {
        return self.residues.len();
    }

    fn get_sequence(&self) -> PyResult<Vec<String>> {
        return Ok(self
            .residues
            .iter()
            .map(|residue| residue.name.clone())
            .collect());
    }
}

#[pyclass]
#[derive(Clone, Serialize)]
struct Structure {
    #[pyo3(set, get)]
    name: String,

    #[pyo3(set, get)]
    chains: Vec<Chain>,
}

#[pymethods]
impl Structure {
    #[new]
    fn new(file_path: String) -> PyResult<Structure> {
        let mut structure = Structure {
            name: String::new(),
            chains: vec![],
        };

        let pdb_file = File::open(Path::new(&file_path))?;
        let pdb_file_reader = BufReader::new(pdb_file);

        let mut prev_chain_name = String::new();
        let mut current_chain_name: String;
        let mut prev_residue_number: u64 = 0;
        let mut current_resiude_number: u64;
        let mut current_residue_name: String;

        let mut atom_name: String;
        let mut atom_pos_x: f32;
        let mut atom_pos_y: f32;
        let mut atom_pos_z: f32;

        for line in pdb_file_reader.lines() {
            let line = line?;

            if &line[0..4] == "ATOM" {
                current_chain_name = String::from(&line[21..22]);
                current_resiude_number = line[23..26].parse()?;
                current_residue_name = String::from(line[17..20].trim());

                atom_name = String::from(line[12..16].trim());
                atom_pos_x = line[30..38].trim().parse()?;
                atom_pos_y = line[38..46].trim().parse()?;
                atom_pos_z = line[46..54].trim().parse()?;

                if prev_chain_name != current_chain_name {
                    structure
                        .chains
                        .push(Chain::new(current_chain_name.clone(), vec![]));

                    prev_chain_name = current_chain_name;
                }

                if prev_residue_number != current_resiude_number {
                    structure
                        .chains
                        .last_mut()
                        .unwrap()
                        .residues
                        .push(Residue::new(
                            current_residue_name,
                            current_resiude_number,
                            vec![],
                        ));

                    prev_residue_number = current_resiude_number;
                }

                structure
                    .chains
                    .last_mut()
                    .unwrap()
                    .residues
                    .last_mut()
                    .unwrap()
                    .atoms
                    .push(Atom::new(atom_name, atom_pos_x, atom_pos_y, atom_pos_z));
            } else if &line[0..5] == "TITLE" {
                structure.name = String::from(line[10..80].trim());
            }
        }

        return Ok(structure);
    }

    fn __repr__(&self) -> String {
        let mut residues_number = 0;
        let mut atoms_number = 0;

        for chain in self.chains.iter() {
            residues_number += chain.residues.len();

            for residue in chain.residues.iter() {
                atoms_number += residue.atoms.len();
            }
        }

        return format!(
            "Structure: \"{}\"\n\tChains number: {}\n\tResidues number: {}\n\tAtoms number: {}",
            self.name,
            self.chains.len(),
            residues_number,
            atoms_number
        );
    }

    fn dump_to_json(&self) -> PyResult<String> {
        let serialized_structure = serde_json::to_string(self).unwrap();

        return Ok(serialized_structure);
    }
}

#[pymodule]
fn pdb_parser(_py: Python, module: &PyModule) -> PyResult<()> {
    module.add_class::<Atom>()?;
    module.add_class::<Residue>()?;
    module.add_class::<Chain>()?;
    module.add_class::<Structure>()?;

    return Ok(());
}
