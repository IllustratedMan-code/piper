use regex::Regex;
use std::vec::Vec;
use std::collections::VecDeque;



#[derive(Clone)]
pub struct ScriptString{
    pub string_fragments: VecDeque<String>,
    pub interpolations: Vec<String>,
}

impl ScriptString {
    pub fn new(script: String) -> ScriptString {
        let interpolation_regex = Regex::new(r"\$\{(.*)\}").unwrap();
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
            interpolations: matches,
        }
    }
}

impl ToString for ScriptString {
    fn to_string(&self) -> String{
        
        let mut script_fragments = VecDeque::from(self.string_fragments.clone());
        let mut s = script_fragments.pop_front().unwrap();
        for (i, frag) in std::iter::zip(self.interpolations.iter(), script_fragments.iter()) {

            s = s + i + frag;
        }
        indent_string(s).unwrap()
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

    let s: String = strings
        .map(|i| match i.strip_prefix(indents) {
            Some(v) => v,
            None => i,
        })
        .map(|i| i.to_string())
        .collect::<std::vec::Vec<String>>()
        .join("\n");

    return Ok(s);
}
