use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL;
use comfy_table::{ContentArrangement, Table};
use std::{
    collections::HashMap, hash::DefaultHasher, hash::Hash, hash::Hasher,
};
use steel::{
    SteelVal,
    rvals::{Custom, FromSteelVal},
};
mod scriptstring;
use scriptstring::ScriptString;
mod evaluator;

#[derive(Clone)]
pub struct Derivation {
    attributes: HashMap<SteelVal, SteelVal>,
    pub script: ScriptString,
    pub name: String,
    pub hash: Option<String>,
    pub inward_edges: Option<Vec<String>>,
    pub container: Option<String>,
    pub time: Option<usize>,
    pub memory: Option<usize>,
    pub shell: Option<String>
    
}

fn calculate_hash(name: String, script: String) -> String {
    let mut s = DefaultHasher::new();
    let combined = format!("{}{}", name, script);
    combined.hash(&mut s);
    let hash = s.finish().to_string();
    format!("{}-{}", hash, name)
}

impl Derivation {
    pub fn new(
        attributes: HashMap<SteelVal, SteelVal>,
    ) -> Result<Derivation, String> {
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
            script: ScriptString::new(
                String::from_steelval(script)
                    .expect("Couldn't interpret script as a string"),
            ),
            name: String::from_steelval(name)
                .expect("Couldn't interpret name as a string"), // need to handle this error
            inward_edges: None,
            container: None,
            time: None,
            memory: None,
            shell: None
        };

        Ok(d)
    }
    pub fn attr(&self, key: SteelVal) -> Option<SteelVal> {
        self.attributes.get(&key).cloned()
    }
    pub fn script(&self) -> String {
        if let Some(v) = self.hash.clone(){
            self.script.to_string().replace(super::OUT_PLACEHOLDER, "../out")
        } else {
            self.script.to_string()
        }
    }

    pub fn hash(&self) -> String {
        self.hash.clone().expect("Hash doesn't exist yet")
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
                .flat_map(|i| extract_derivation_hashes(i.clone()))
                .collect(),
        )
    }

    pub fn write_hash(&mut self) {
        let hash = calculate_hash(self.name.clone(), self.script.to_string());
        
        self.hash = Some(hash);
    }

    pub fn display(&self) -> DisplayTable {
        let mut table = Table::new();
        let hash = self.hash.clone().unwrap_or("None".to_string());
        table
            .load_preset(UTF8_FULL)
            .apply_modifier(UTF8_ROUND_CORNERS)
            .set_content_arrangement(ContentArrangement::Dynamic)
            //.set_width(40)
            .add_row(vec!["hash".to_string(), hash])
            .add_row(vec!["name".to_string(), self.name.clone()])
            .add_row(vec![
                "script".to_string(),
                self.script(),
            ]);

        DisplayTable { table}
    }
    pub fn run(&self) -> Result<(), ()> {
        
        todo!();
        Ok(())
    }
}


// have to make custom type because can't implement external type for type from different crate
pub struct DisplayTable { table: Table }

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

impl Custom for Derivation{
    fn fmt(&self) -> Option<std::result::Result<String, std::fmt::Error>> {
        Some(Ok(format!("{:?}", self.hash.clone().expect("no hash available"))))
    }
}

impl std::fmt::Display for Derivation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.display())
    }
}

impl std::fmt::Debug for Derivation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self.attributes)
    }
}


fn extract_derivation_hashes(val: SteelVal) -> Vec<String> {
    let mut vec = Vec::<String>::new();
    extract_derivation_hashes_recursive(val, &mut vec);
    vec
}

fn extract_derivation_hashes_recursive(val: SteelVal, vec: &mut Vec<String>) {
    if let Ok(derivation) = Derivation::from_steelval(&val) {
        vec.push(derivation.hash.expect("Hash doesn't exist"));
        return;
    }

    if let Ok(vector) = Vec::<SteelVal>::from_steelval(&val) {
        for i in vector {
            extract_derivation_hashes_recursive(i, vec);
        }
        return;
    }

    if let Ok(hashmap) = HashMap::<SteelVal, SteelVal>::from_steelval(&val) {
        for (_, v) in hashmap {
            extract_derivation_hashes_recursive(v, vec)
        }

    }
}
