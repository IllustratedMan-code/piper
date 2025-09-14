use std::{collections::HashMap, hash::DefaultHasher, hash::Hash, hash::Hasher};
use steel::SteelVal;
use steel_derive::Steel;

#[derive(Clone, Steel)]
pub struct Derivation {
    attributes: HashMap<SteelVal, SteelVal>,
    pub hash: String
}


fn calculate_hash(name: String, script: String) -> String {
    let mut s = DefaultHasher::new();
    let combined = format!("{}{}", name, script);
    combined.hash(&mut s);
    let hash = s.finish().to_string();
    return format!("{}-{}", hash, name);
}


impl Derivation{
    pub fn new(attributes: HashMap<SteelVal, SteelVal>) -> Result<Derivation, String>{

        let name = match attributes.get(&SteelVal::SymbolV("name".into())) {
            Some(v) => v,
            None => {
                return Err("Name attribute does not exist".to_string())
            }
        };
        let script = match attributes.get(&SteelVal::SymbolV("script".into())) {
            Some(v) => v,
            None => {
                return Err("Script attribute does not exist!".to_string())
            }
        };

        let hash = calculate_hash(name.to_string(), script.to_string());

        let d = Derivation { attributes: attributes, hash: hash};

        Ok(d)
    }
}


impl std::fmt::Display for Derivation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self.attributes)
    }
}
