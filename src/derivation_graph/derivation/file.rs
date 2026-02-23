use super::{DerivationHash, DisplayTable, File};
use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL;
use comfy_table::{ContentArrangement, Table};
use sha2::Digest;
use std::fs;
use std::path::{PathBuf, Path};
use steel::steel_vm::builtin::BuiltInModule;
use steel::steel_vm::register_fn::RegisterFn;
use steel_derive::Steel;

#[derive(Steel, Clone)]
pub enum HashMethod {
    Contents,
    Timestamp,
}

impl File {
    pub fn new(
        path: PathBuf,
        hash_method: HashMethod
    ) -> Result<File, std::io::Error> {
        let contents = match hash_method {
            HashMethod::Contents => std::fs::read_to_string(&path)?,
            HashMethod::Timestamp => format!("{:?}",std::fs::metadata(&path)?.modified()?),
        };
        let hash = calculate_hash(&path, contents);

        Ok(File { hash, path })
    }
    pub fn display(&self) -> DisplayTable {
        let mut table = Table::new();
        table
            .load_preset(UTF8_FULL)
            .apply_modifier(UTF8_ROUND_CORNERS)
            .set_content_arrangement(ContentArrangement::Dynamic)
            //.set_width(40)
            .add_row(vec!["hash".to_string(), format!("{}", self.hash)])
            .add_row(vec!["path".to_string(), format!("{:?}", self.path)]);


        DisplayTable { table }
    }
}

fn calculate_hash(path: &Path, contents: String) -> DerivationHash {
    let mut hasher = sha2::Sha256::new();
    hasher.update(format!("{:?}", path));
    let result = hasher.finalize();
    let hash = format!("{:x}-{:?}", result, path.file_name());
    DerivationHash(hash)
}


pub fn register_steel_functions(module: &mut BuiltInModule){
    module.register_type::<File>("File?");
    module.register_fn("File::new", File::new);


}
