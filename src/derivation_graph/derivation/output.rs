use std::collections::HashMap;
use sha2::Digest;
use steel::steel_vm::builtin::BuiltInModule;
use steel::steel_vm::register_fn::RegisterFn;

use super::{Derivation, DisplayTable, Output, DerivationHash};
use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL;
use comfy_table::{ContentArrangement, Table};

impl Output {
    pub fn new(outputs: HashMap<String, Derivation>) -> Output {
        let hash = calculate_hash(&outputs);
        Output{
            hash,
            inward_edges: outputs.into_values().map(|v|v.hash()).collect()
        }
    }

    pub fn into_derivation(self) -> Derivation {
        Derivation::Output(self)
    }

    pub fn display(&self) -> DisplayTable {
        let mut table = Table::new();
        table
            .load_preset(UTF8_FULL)
            .apply_modifier(UTF8_ROUND_CORNERS)
            .set_content_arrangement(ContentArrangement::Dynamic)
            //.set_width(40)
            .add_row(vec!["hash".to_string(), format!("{}", self.hash)]);

        DisplayTable { table }
    }
}

fn calculate_hash(outputs: &HashMap<String, Derivation>) -> DerivationHash {
    let mut hasher = sha2::Sha256::new();
    for (k,v) in outputs.iter(){
        hasher.update(format!("{}{}", k, v.hash()));
    }
    let hash = format!("{:x}-Output", hasher.finalize());
    DerivationHash(hash)
}

pub fn register_steel_functions(module: &mut BuiltInModule){
    module.register_fn("Output::new", Output::new);
    module.register_fn("Output::into_derivation", Output::into_derivation);
}


