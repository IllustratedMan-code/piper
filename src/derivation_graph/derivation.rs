use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL;
use comfy_table::{ContentArrangement, Table};
use process::scriptstring::ScriptString;
use regex::bytes::Regex;
use std::fmt::Display;
use std::path::PathBuf;
use std::{
    collections::HashMap, hash::Hash,
};
use steel::steel_vm::builtin::BuiltInModule;
use steel::steel_vm::register_fn::RegisterFn;
use steel::{
    SteelVal,
    rvals::{Custom, FromSteelVal, IntoSteelVal},
};

use crate::config::{Config, ParamValue};
pub mod evaluator;
use sha2::Digest;
pub mod dataframe;
pub mod file;
pub mod output;
pub mod process;
pub mod iterator;
pub mod test;
use steel_derive::Steel;

// Derivation needs to be an enum with possible derivation types
// File derivation, metadata derivation, output derivation

#[derive(Debug, Clone)]
pub enum Derivation {
    Process(Process),
    File(File),
    Output(Output),
    Dataframe(Dataframe),
    Iterator(Iterator),
    Test(Test)
}

impl Derivation {
    pub fn hash(&self) -> DerivationHash {
        match self {
            Derivation::Process(v) => v.hash.clone(),
            Derivation::File(v) => v.hash.clone(),
            Derivation::Output(v) => v.hash.clone(),
            Derivation::Dataframe(v) => v.hash.clone(),
            Derivation::Iterator(v) => v.hash.clone(),
            Derivation::Test(v) => v.hash.clone()
        }
    }
    pub fn inputs(&self) -> Option<Vec<DerivationHash>> {
        match self {
            Derivation::Process(v) => Some(v.inward_edges.clone()),
            Derivation::File(_) => None,
            Derivation::Output(v) => Some(v.inward_edges.clone()),
            Derivation::Dataframe(v) => Some(v.derivations.clone()),
            Derivation::Iterator(v) => Some(v.inward_edges.clone()),
            Derivation::Test(v) => Some(v.inward_edges.clone()),
        }
    }
    pub fn outputs(&self) -> Vec<DerivationHash> {
        match self {
            Derivation::Dataframe(v) => {
                [vec![v.hash.clone()], v.derivations.clone()].concat()
            }
            _ => vec![self.hash().clone()],
        }
    }

    pub fn display(&self) -> DisplayTable {
        match self {
            Derivation::Process(v) => v.display(),
            Derivation::File(v) => v.display(),
            Derivation::Output(v) => v.display(),
            Derivation::Dataframe(v) => v.display(),
            Derivation::Iterator(v) => v.display(),
            Derivation::Test(v) => v.display()
        }
    }
}



pub fn register_steel_functions(module: &mut BuiltInModule) {
    module.register_type::<Derivation>("Derivation?");
    module.register_fn("Derivation::hash", Derivation::hash);
    module.register_fn("Derivation::display", Derivation::display);
}

impl steel::rvals::Custom for Derivation {
    fn fmt(&self) -> Option<std::result::Result<String, std::fmt::Error>> {
        match self {
            Derivation::Process(v) => <DerivationHash as Custom>::fmt(&v.hash),
            Derivation::File(v) => <DerivationHash as Custom>::fmt(&v.hash),
            Derivation::Output(v) => <DerivationHash as Custom>::fmt(&v.hash),
            Derivation::Dataframe(v) => {
                <DerivationHash as Custom>::fmt(&v.hash)
            },
            Derivation::Iterator(v) => {
                <DerivationHash as Custom>::fmt(&v.hash)
            }
            Derivation::Test(v) => {
                <DerivationHash as Custom>::fmt(&v.hash)
            }
        }
    }
}




#[derive(Debug, Clone, Steel)]
pub struct File {
    pub path: PathBuf,
    pub hash: DerivationHash,
}

#[derive(Debug, Clone, Steel)]
pub struct Output {
    pub hash: DerivationHash,
    pub inward_edges: Vec<DerivationHash>,
}

#[derive(Debug, Clone, Steel)]
pub struct Dataframe {
    pub hash: DerivationHash,
    pub derivations: Vec<DerivationHash>,
}

#[derive(Debug, Clone, Steel)]
pub struct Iterator {
    pub hash: DerivationHash,
    pub inward_edges: Vec<DerivationHash>,
}

#[derive(Debug, Clone, Steel)]
pub struct Test {
    pub hash: DerivationHash,
    pub inward_edges: Vec<DerivationHash>,
}

/// Process Derivation
#[derive(Clone)] // Debug and Steel are custom implemented
pub struct Process {
    attributes: HashMap<String, SteelVal>,
    pub script: ScriptString,
    pub name: String,
    pub hash: DerivationHash,
    pub inward_edges: Vec<DerivationHash>,
    pub container: Option<String>,
    pub time: Option<usize>,
    pub memory: Option<usize>,
    pub shell: String,
    pub hpc_runtime: Option<String>,
    pub container_runtime: Option<String>,
    pub work_dir: String,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct DerivationHash(String);

impl std::fmt::Display for DerivationHash {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl Custom for DerivationHash {
    fn fmt(&self) -> Option<std::result::Result<String, std::fmt::Error>> {
        Some(Ok(self.0.clone()))
    }
}

pub struct DisplayTable {
    table: Table,
}

impl Custom for DisplayTable {
    fn fmt(&self) -> Option<std::result::Result<String, std::fmt::Error>> {
        Some(Ok(format!("\n{}", self.table)))
    }
}

impl std::fmt::Display for DisplayTable {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "\n{}", self.table)
    }
}


