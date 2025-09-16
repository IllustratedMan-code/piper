use daggy::petgraph::adj::OutgoingEdgeReferences;
use regex::Regex;
use std::collections::VecDeque;
use std::iter::Peekable;
use std::vec::Vec;
use std::{collections::HashMap, hash::DefaultHasher, hash::Hash, hash::Hasher};
use steel::SteelVal;
use steel_derive::Steel;

#[derive(Clone, Steel)]
pub struct Derivation {
    attributes: HashMap<SteelVal, SteelVal>,
    pub hash: String,
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

        let hash = calculate_hash(name.to_string(), script.to_string());

        let d = Derivation {
            attributes: attributes,
            hash: hash,
        };

        Ok(d)
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

#[derive(Clone)]
pub struct ScriptString {
    string_fragments: VecDeque<String>,
    execution_queue: Vec<String>,
}

impl ScriptString {
    pub fn new(script: String) -> ScriptString {
        let interpolation_regex = Regex::new(r"${(.*)}").unwrap();
        let matches: Vec<String> = interpolation_regex
            .captures_iter(script.as_str())
            .map(|captures| captures[1].to_string())
            .collect();

        let split: Vec<String> = interpolation_regex
            .split(script.as_str())
            .map(|s| s.to_string())
            .collect();

        ScriptString {
            string_fragments: VecDeque::from(split),
            execution_queue: matches,
        }
    }
}

pub fn indent_string(s: String) -> Result<String, String> {
    let mut strings = s.split("\n").peekable();
    strings.next(); // consumes first element of iterator (will be needed to add script annotations like 'bash')
    let whitespace_regex = Regex::new(r"^(\s*)").unwrap();
    let first_elem = match strings.peek() {
        Some(v) => v,
        None => return Err("String is empty!!".to_string()),
    };
    let indents = match whitespace_regex.captures(first_elem) {
        Some(v) => v.get(1).unwrap().as_str(),
        None => "",
    };

    let s: String = strings.map(|i| match i.strip_prefix(indents) {
        Some(v) => v,
        None => i,
    }).map(|i| i.to_string()).collect::<std::vec::Vec<String>>().join("\n");

    return Ok(s)

}

pub trait InterpolateScript {
    fn interpolate(&self, script: ScriptString) -> Result<String, String>;
}

impl InterpolateScript for steel::steel_vm::engine::Engine {
    fn interpolate(&self, script: ScriptString) -> Result<String, String> {
        let mut script_fragments = VecDeque::from(script.string_fragments);
        let mut s = match script_fragments.pop_front() {
            Some(v) => v,
            None => return Err("Script is empty!!".to_string()),
        };
        for (i, frag) in std::iter::zip(script.execution_queue.iter(), script_fragments.iter()) {
            s = s + i + frag;
        }
        s = indent_string(s).unwrap();
        Ok(s)
    }
}
