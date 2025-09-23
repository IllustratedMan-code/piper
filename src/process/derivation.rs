use std::{collections::HashMap, hash::DefaultHasher, hash::Hash, hash::Hasher};
use steel::{
    SteelVal,
    rvals::{Custom, FromSteelVal, IntoSteelVal},
};
use steel_derive::Steel;
mod scriptstring;
use scriptstring::ScriptString;

#[derive(Clone)]
pub struct Derivation {
    attributes: HashMap<SteelVal, SteelVal>,
    pub script: ScriptString,
    pub name: String,
    pub hash: Option<String>,
    pub inward_edges: Option<Vec<String>>,
}

fn calculate_hash(name: String, script: String) -> String {
    let mut s = DefaultHasher::new();
    let combined = format!("{}{}", name, script);
    combined.hash(&mut s);
    let hash = s.finish().to_string();
    return format!("{}-{}", hash, name);
}

impl Derivation {
    pub fn new(attributes: HashMap<SteelVal, SteelVal>) -> Result<Derivation, String> {
        let name = match attributes.get(&SteelVal::SymbolV("name".into())) {
            Some(v) => v,
            None => return Err("Name attribute does not exist".to_string()),
        };
        let script = match attributes.get(&SteelVal::SymbolV("script".into())) {
            Some(v) => v,
            None => return Err("Script attribute does not exist!".to_string()),
        };

        let d = Derivation {
            attributes: attributes.clone(),
            hash: None,
            script: ScriptString::new(String::from_steelval(script).unwrap()), // TODO error handling
            name: String::from_steelval(name).unwrap(), // need to handle this error
            inward_edges: None,
        };

        Ok(d)
    }
    pub fn attr(&self, key: SteelVal) -> Option<SteelVal> {
        self.attributes.get(&key).cloned()
    }
    pub fn script(&self) -> String {
        self.script.to_string()
    }

    pub fn hash(&self) -> String {
        self.hash.clone().unwrap()
    }

    pub fn interpolations(&self) -> Vec<String> {
        self.script.interpolations.clone()
    }

    pub fn inward_hashes(&self) -> Option<Vec<String>> {
        self.inward_edges.clone()
    }

    pub fn set_interpolations(&mut self, interpolations: Vec<SteelVal>) {
        let stringinterpolations: Vec<String> =
            interpolations.iter().map(|i| i.to_string()).collect();
        self.script.interpolations = stringinterpolations;
        self.inward_edges = Some(
            interpolations
                .iter()
                .map(|i| extract_derivation_hashes(i.clone()))
                .flatten()
                .collect(),
        )
    }

    pub fn write_hash(&mut self) {
        let hash = calculate_hash(self.name.clone(), self.script.to_string());
        self.hash = Some(hash);
    }
}

impl std::fmt::Display for Derivation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self.attributes)
    }
}

impl std::fmt::Debug for Derivation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self.attributes)
    }
}

pub trait InterpolateDerivationScript {
    fn interpolate(&mut self, derivation: Derivation) -> Result<Derivation, String>;
}

impl InterpolateDerivationScript for super::DAG {
    fn interpolate(&mut self, derivation: Derivation) -> Result<Derivation, String> {
        let mut derivation = derivation.clone();

        derivation.script.interpolations = derivation
            .script
            .interpolations
            .iter()
            .map(|x| {
                (match self.vm.run(x.clone()) {
                    Ok(v) => v,
                    Err(e) => vec![format!("{:?}", e).into_steelval().unwrap()],
                })[0]
                    .to_string()
            })
            .collect();

        let hash = calculate_hash(derivation.name.clone(), derivation.script.to_string());
        derivation.hash = Some(hash.clone());
        match self.nodes.safe_insert(hash.clone(), derivation.clone()) {
            Ok(_) => {
                self.dag.add_node(hash.clone());
                Ok(derivation)
            }
            Err(_) => return Ok(derivation),
        }
    }
}

pub trait SafeInsert<K, V> {
    fn safe_insert(&mut self, key: K, value: V) -> Result<(), ()>;
}

impl<K: Eq + Hash, V> SafeInsert<K, V> for HashMap<K, V> {
    fn safe_insert(&mut self, key: K, value: V) -> Result<(), ()> {
        match self.get(&key) {
            Some(_) => Err(()),
            None => {
                self.insert(key, value);
                Ok(())
            }
        }
    }
}

impl Custom for Derivation {
    fn fmt(&self) -> Option<std::result::Result<String, std::fmt::Error>> {
        Some(Ok(match self.hash.clone() {
            Some(v) => format!("{:?}", v),
            None => "The hash has not been generated yet!!".to_string(),
        }))
    }
}

fn extract_derivation_hashes(val: SteelVal) -> Vec<String> {
    let mut vec = Vec::<String>::new();
    extract_derivation_hashes_recursive(val, &mut vec);
    vec
}

fn extract_derivation_hashes_recursive(val: SteelVal, vec: &mut Vec<String>) {
    if let Ok(derivation) = Derivation::from_steelval(&val) {
        vec.push(derivation.hash.unwrap());
        return;
    }

    if let Ok(vector) = Vec::<SteelVal>::from_steelval(&val) {
        for i in vector {
            extract_derivation_hashes_recursive(i, vec);
        }
        return;
    }

    if let Ok(hashmap) = HashMap::<SteelVal, SteelVal>::from_steelval(&val) {
        for (k, v) in hashmap {
            extract_derivation_hashes_recursive(v, vec)
        }

        return;
    }
}
