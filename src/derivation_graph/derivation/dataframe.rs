use super::{Dataframe, DerivationHash, DisplayTable};
use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL;
use comfy_table::{ContentArrangement, Table};
use polars::prelude::*;
use polars_utils::aliases::PlSeedableRandomStateQuality;
use polars_utils::total_ord::TotalHash;
use sha2::Digest;
use steel::steel_vm::builtin::BuiltInModule;
use steel::SteelVal;
use steel::SteelErr;
use steel::steel_vm::register_fn::RegisterFn;

<<<<<<< Updated upstream
=======
use steel::{
    rvals::{Custom, FromSteelVal, IntoSteelVal},
};
>>>>>>> Stashed changes


impl Dataframe {
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

    pub fn read_csv(path: String) -> Result<Self, String> {
        let frame = CsvReadOptions::default()
            .with_has_header(true)
            .try_into_reader_with_file_path(Some(std::path::PathBuf::from(path)))
            .map_err(|x| x.to_string())?
            .finish()
            .map_err(|x| x.to_string())?;
        Self {
            frame,
            hash: DerivationHash::default(),
            derivations: vec![],
        }.hash()
    }

    pub fn into_derivation(self) -> super::Derivation{
        super::Derivation::Dataframe(self)
    }

    pub fn hash(mut self) -> Result<Dataframe, String> {
        let mut hasher = sha2::Sha256::new();
        let frame_hash = hash_frame(&self.frame)?.0;
        hasher.update(frame_hash);
        hasher.update(
            self.derivations
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(""),
        );
        let result = hasher.finalize();
        self.hash = DerivationHash(format!("{:x}", result));
        Ok(self)
    }

    pub fn with_column(
        mut self,
        name: String,
        values: Vec<SteelVal>,
    ) -> Result<Dataframe, String> {
        let first = std::mem::discriminant(&values[0]);
        let all_same_type =
            values.iter().all(|x| first == std::mem::discriminant(x));

        if !all_same_type {
            return Err(
                "All elements in column must be the same type!".to_string()
            );
        }

        let column = match values[0] {
            SteelVal::BoolV(_) => {
                let vals: Vec<bool> = values
                    .into_iter()
                    .map(|v| {
                        if let SteelVal::BoolV(b) = v {
                            b
                        } else {
                            unreachable!("Already checked all same type")
                        }
                    })
                    .collect();
                Column::from(Series::new(name.into(), vals))
            }
            _ => return Err("Unsupported data type for Dataframe".to_string()),
        };

        self.frame.with_column(column).map_err(|x| x.to_string())?;
        self.hash()
    }
}


pub fn register_steel_functions(module: &mut BuiltInModule) {
    module.register_fn("read-csv", Dataframe::read_csv);
    module.register_fn("with-column", Dataframe::with_column);
    module.register_fn("Dataframe::into_derivation", Dataframe::into_derivation);
<<<<<<< Updated upstream
=======
    module.register_fn("df::display", Dataframe::display);
>>>>>>> Stashed changes
}


// looks like polars will work with custom types
fn test_polars() {
    let data = [
        DerivationHash("hi".to_string()),
        DerivationHash("there".to_string()),
    ];

    // undocumented bullshit
    let s = ObjectChunked::<DerivationHash>::new_from_vec(
        "my_col".into(),
        data.into(),
    );

    let df = DataFrame::new_infer_height(vec![s.into_column()]).expect("blah");
    let mut iter = df.columns().iter();
    let first = iter.next().expect("a key");
}

fn hash_frame(frame: &DataFrame) -> Result<DerivationHash, String> {
    let mut columns = frame.columns().iter();
    let first = columns.next().ok_or_else(|| {
        "At least one column must exist for hashing".to_string()
    })?;
    let hasher = PlSeedableRandomStateQuality::default();
    let mut hashes = Vec::<u64>::new();

    for col in columns {
        col.vec_hash_combine(hasher.clone(), &mut hashes);
    }

    Ok(DerivationHash(
        hashes
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(""),
    ))
}

// Stuff needed for custom types in polars
impl Default for DerivationHash {
    fn default() -> Self {
        DerivationHash("".to_string())
    }
}

impl polars_utils::total_ord::TotalHash for DerivationHash {
    fn tot_hash<H>(&self, state: &mut H)
    where
        H: std::hash::Hasher,
    {
        state.write(self.0.as_bytes())
    }
}
impl polars_utils::total_ord::TotalEq for DerivationHash {
    fn tot_eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl PolarsObject for DerivationHash {
    fn type_name() -> &'static str {
        "DerivationHash"
    }
}
<<<<<<< Updated upstream
=======

impl Custom for Dataframe {
    fn fmt(&self) -> Option<std::result::Result<String, std::fmt::Error>> {
        Some(Ok(format!(
            "{}",
            self.frame
        )))
    }
}
>>>>>>> Stashed changes
