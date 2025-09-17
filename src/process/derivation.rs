use std::{collections::HashMap, hash::DefaultHasher, hash::Hash, hash::Hasher};
use steel::{rvals::{FromSteelVal}, SteelVal};
use steel_derive::Steel;
mod scriptstring;
use scriptstring::{ScriptString};

#[derive(Clone, Steel)]
pub struct Derivation {
    attributes: HashMap<SteelVal, SteelVal>,
    pub script: ScriptString,
    pub name: String,
    pub hash: Option<String>,
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
        };

        Ok(d)
    }
    pub fn attr(&self, key: SteelVal) -> Option<SteelVal>{
        self.attributes.get(&key).cloned()
    }
    pub fn script(&self) -> String{
        self.script.to_string()
    }

    pub fn hash(&self) -> String{
        self.hash.clone().unwrap()
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
            .map(|x| self.vm.run(x.clone()).unwrap()[0].to_string())
            .collect();

        let hash = calculate_hash(derivation.name.clone(), derivation.script.to_string());
        println!("{:?}", derivation.script.to_string());
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

trait SafeInsert<K, V> {
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
