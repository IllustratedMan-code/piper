use crate::debug_utils::Runner;
use std::collections::HashMap;
use steel::steel_vm::builtin::BuiltInModule;
use steel::steel_vm::engine::Engine;
use steel::steel_vm::register_fn::RegisterFn;
use steel::{
    SteelErr, SteelVal,
    rvals::{FromSteelVal, IntoSteelVal},
};
use steel_derive::Steel;
use steel_repl::colored::Colorize;

/// Valid types for config params
#[derive(Debug, Clone)]
pub enum ParamValue {
    String(std::string::String),
    Int(isize),
    Float(f64),
    Bool(bool),
    Value(HashMap<String, ParamValue>),
}

/// Cast to ParamValue from a scheme value
impl FromSteelVal for ParamValue {
    fn from_steelval(val: &SteelVal) -> steel::rvals::Result<Self> {
        match val {
            SteelVal::IntV(i) => Ok(ParamValue::Int(*i)),
            SteelVal::StringV(i) => {
                Ok(ParamValue::String(i.clone().to_string()))
            }
            SteelVal::NumV(i) => Ok(ParamValue::Float(*i)),
            SteelVal::BoolV(i) => Ok(ParamValue::Bool(*i)),
            SteelVal::HashMapV(i) => Ok(ParamValue::Value(
                i.iter().map(| (k, v) |{
                    (String::from_steelval(&k.clone()).expect("couldn't convert to string"),
                    ParamValue::from_steelval(&v.clone()).expect("couldn't convert to ParamValue"))
                }).collect::<HashMap<String, ParamValue>>())),
            _ => Err(SteelErr::new(
                steel::rerrs::ErrorKind::ConversionError,
                format!("Cannot convert {:?} to ParamValue", val),
            )),
        }
    }
}


/// Cast to a scheme value from a ParamValue
impl IntoSteelVal for ParamValue {
    fn into_steelval(self) -> steel::rvals::Result<SteelVal> {
        match self {
            ParamValue::String(s) => s.into_steelval(),
            ParamValue::Int(s) => s.into_steelval(),
            ParamValue::Float(s) => s.into_steelval(),
            ParamValue::Bool(s) => s.into_steelval(),
            ParamValue::Value(s) => s.into_steelval(),
        }
    }
}

/// Config objects store the config defined in the piper config file
#[derive(Debug, Clone, Steel)]
pub struct Config {
    pub params: HashMap<String, ParamValue>,
    pub config: HashMap<String, ParamValue>,
}

/// Macro for throwing error if paramvalue is not of the valid types
macro_rules! type_key {
    ($key:expr,$val_type:path) => {{
        if (!matches!($key, $val_type(_))) {
            return Err(format!(
                "{:?} must be a {:?}",
                $key,
                stringify!($val_type)
            ));
        }
    }};
}


impl Config {
    /// Run the scheme config file in a steel vm, then extract the config struct and return
    pub fn new(config_path: Option<std::path::PathBuf>) -> Config {
        let mut config = Config {
            params: HashMap::new(),
            config: HashMap::new(),
        };
        let mut vm = Engine::new();
        let mut module = BuiltInModule::new("config");
        module.register_type::<Config>("Config?");
        module.register_type::<ParamValue>("ParamValue?");
        module.register_value(
            "config",
            config
                .clone()
                .into_steelval()
                .expect("Couldn't register config object into config vm."),
        );

        module.register_fn("insert_config", Config::insert_config);
        module.register_fn("insert_param", Config::insert_param);
        vm.register_module(module);
        vm.register_steel_module(
            "config".to_string(),
            include_str!("steel-modules/config/main.scm").to_string(),
        );
        vm.run(r#"(require "config")"#)
            .expect("couldn't require config module");
        vm.run(include_str!("steel-modules/config/defaults.scm"))
            .expect("Couldn't create config defaults");

        if let Some(v) = config_path {
            vm.run_file_or_print_error(v.clone()).unwrap_or_else(|e| {
                eprintln!(
                    "{}: couldn't read {:?} because of '{}', using defaults",
                    "Warning".bold().yellow(),
                    v,
                    format!("{}", e).red()
                )
            })
        }
        config = vm
            .extract::<Config>("Config.config")
            .expect("couldn't extract config from config vm");
        config
    }

    /// insert a param into the internal params HashMap
    pub fn insert_param(&mut self, key: String, value: ParamValue) {
        self.params.insert(key, value);
    }

    /// Insert a config item into the internal config HashMap
    /// config items are not settable from the CLI
    pub fn insert_config(
        &mut self,
        key: String,
        steel_value: SteelVal,
    ) -> Result<(), String> {
        let value = ParamValue::from_steelval(&steel_value)
            .expect("Couln't convert steelval");
        match key.as_str() {
            "workDir" => type_key!(value, ParamValue::String),
            "entryPoint" => type_key!(value, ParamValue::String),
            "shell" => type_key!(value, ParamValue::String),
            _ => {}
        };
        self.config.insert(key, value);
        Ok(())
    }

    /// Loads params HashMap into the scheme vm as params.* variables
    pub fn register_params(&self, vm: &mut Engine) {
        for (k, v) in self.params.iter() {
            vm.register_external_value(format!("params.{}", k.clone()).as_str(), v.clone())
                .unwrap_or_else(|_| {
                    panic!("couldn't register params value: {:?}", k)
                });
        }
    }

    /// The location of the entrypoint for the piper pipeline, e.g. "main.scm"
    /// The entrypoint should contain an outputs macro
    pub fn entry_point(&self) -> String{
        let entry_point = self.config.get("entryPoint").expect("No entryPoint in config!");
        if let ParamValue::String(v) = entry_point{
            v.clone()
        } else {
            panic!("Entrypoint conversion to string failed, this should never happen!")
        }

    }
}
