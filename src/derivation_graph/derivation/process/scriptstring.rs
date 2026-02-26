use regex::Regex;
use steel::SteelVal;
use std::collections::VecDeque;
use std::vec::Vec;
use steel_derive::Steel;
use steel::steel_vm::builtin::BuiltInModule;
use steel::steel_vm::register_fn::RegisterFn;

#[derive(Debug, Clone, Steel)]
pub struct ScriptString {
    pub string_fragments: VecDeque<String>,
    pub interpolations: Vec<SteelVal>,
}


impl ScriptString {
    pub fn new(script: String) -> Result<ScriptString, String> {
        let indent_script = indent_string(script)?;
        let interpolation_regex =
            Regex::new(r"\$\{(.*?)\}").expect("couldn't make regex");
        let matches: Vec<SteelVal> = interpolation_regex
            .captures_iter(indent_script.as_str())
            .map(|captures| captures[1].to_string()).map(|x| SteelVal::from(x)).collect();

        let split: Vec<String> = interpolation_regex
            .split(indent_script.as_str())
            .map(|s| s.to_string())
            .collect();

        Ok(ScriptString {
            string_fragments: VecDeque::from(split),
            interpolations: matches,
        })
    }
    pub fn set_interpolations(&mut self, new_interpolations: Vec<SteelVal>){
        self.interpolations = new_interpolations;
    }

    pub fn interpolations(&self) -> Vec<SteelVal>{
        self.interpolations.clone()
    }
}

pub fn register_steel_functions(module: &mut BuiltInModule){
    module.register_type::<ScriptString>("ScriptString?");
    module.register_fn("ScriptString", ScriptString::new);
    module.register_fn("ScriptString::interpolations", ScriptString::interpolations);
    module.register_fn("ScriptString::set_interpolations", ScriptString::set_interpolations);
}


impl std::fmt::Display for ScriptString {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut script_fragments = self.string_fragments.clone();
        let mut s = script_fragments
            .pop_front()
            .expect("couldn't get string fragments");
        for (i, frag) in
            std::iter::zip(self.interpolations.iter(), script_fragments.iter())
        {
            let is = i.to_string();
            s = s + &is + frag;
        }
        write!(
            f,
            "{}",
            s
        )
    }
}

pub fn indent_string(s: String) -> Result<String, String> {
    let mut strings = s.split("\n").peekable();
    //strings.next(); // consumes first element of iterator (will be needed to add script annotations like 'bash')
    let whitespace_regex =
        Regex::new(r"^(\s*)").expect("Couldn't make whitespace regex");
    let first_elem = match strings.peek() {
        Some(v) => v,
        None => return Err("String is empty!!".to_string()),
    };
    let indents = match whitespace_regex.captures(first_elem) {
        Some(v) => v.get(1).expect("indent regex failed").as_str(),
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

    Ok(s)
}
