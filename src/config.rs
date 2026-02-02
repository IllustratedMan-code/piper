use std::collections::HashMap;
use steel::steel_vm::engine::Engine;
use steel_derive::Steel;
use steel::steel_vm::builtin::BuiltInModule;
use steel::rvals::IntoSteelVal;
use steel::steel_vm::register_fn::RegisterFn;
use crate::debug_utils::Runner;


#[derive(Debug, Steel, Clone)]
pub enum ParamValue {
    String(std::string::String),
    Int(i64),
    Float(f64),
    Bool(bool),
    Value(Box<ParamValue>),
}

#[derive(Debug,Clone,Steel)]
pub struct Config {
    params: HashMap<String, ParamValue>,
    config: HashMap<String, ParamValue>,
}


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
    pub fn new(config_string: String) -> Config {
        let mut config = Config{
            params: HashMap::new(),
            config: HashMap::new(),
        };
        let mut vm = Engine::new();
        let mut module = BuiltInModule::new("config");
        module.register_type::<Config>("Config?");
        module.register_value("config", config.clone().into_steelval().expect("Couldn't register config object into config vm."));
        module.register_fn("config_config", Config::insert_config);
        module.register_fn("param_config", Config::insert_param);
        vm.register_module(module);
        vm.run(r#"(require-builtin config as config.)"#).expect("couldn't require config");
        vm.run(config_string).expect("config error (TODO need better handling)");
        config = vm.extract::<Config>("config.config").expect("couldn't extract config from config vm");
        config

    }
    pub fn insert_param(&mut self, key: String, value: ParamValue) {
        self.params.insert(key, value);
    }

    pub fn insert_config(
        &mut self,
        key: String,
        value: ParamValue,
    ) -> Result<(), String> {
        match key.as_str() {
            "work_dir" => type_key!(value, ParamValue::String),
            "runtime" => type_key!(value, ParamValue::String),
            _ => {}
        };
        self.config.insert(key, value);
        Ok(())
    }
    pub fn register_params(&self, vm: &mut Engine) {
        for (k, v) in self.params.iter() {
            vm.register_external_value(k.clone().as_str(), v.clone())
                .unwrap_or_else(|_| panic!("couldn't register params value: {:?}", k));
        }
    }
}
