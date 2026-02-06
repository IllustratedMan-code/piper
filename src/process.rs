//! Module for creating the process graph.
//! Processes are "compiled" into derivations which
//! form the nodes of the ProcessGraph
use std::{collections::HashMap, hash::Hash};
use steel::rvals::Custom;
use steel::rvals::{FromSteelVal, IntoSteelVal};
use steel::steel_vm::builtin::BuiltInModule;
use steel::steel_vm::engine::Engine;
use steel::steel_vm::register_fn::RegisterFn;
use steel_derive::Steel;
pub mod derivation;
use super::config::Config;
use derivation::Derivation;

/// Directed Acyclic Graph containing the derivation nodes
#[derive(Clone, Steel)]
pub struct ProcessGraph {
    pub nodes: HashMap<String, derivation::Derivation>,
    pub config: Config,
}

static OUT_PLACEHOLDER: &str = "0000000000000000000-outdir";

pub fn extract_graph(vm: &mut Engine) -> ProcessGraph {
    let vm_dag = vm.extract_value("dag.dag").expect("couldn't extract dag");
    ProcessGraph::from_steelval(&vm_dag)
        .expect("Couldn't convert dag to process graph")
}

impl ProcessGraph {
    pub fn init(vm: &mut Engine, config: Config) {
        let dag = ProcessGraph {
            nodes: HashMap::<String, derivation::Derivation>::new(),
            config,
        };

        let mut module = BuiltInModule::new("process/dag");

        module.register_type::<ProcessGraph>("ProcessGraph?");
        module.register_value(
            "dag",
            dag.clone().into_steelval().expect("Couldn't add dag to Vm"),
        );

        //module.register_fn("process", ProcessGraph::process);
        module.register_fn("node_count", ProcessGraph::node_count);
        module.register_fn("display_nodes", ProcessGraph::display_nodes);
        module.register_fn("outputs", ProcessGraph::outputs);
        module.register_fn("add-process", ProcessGraph::add_process);
        module.register_type::<Derivation>("Derivation?");
        module.register_fn("process", Derivation::new);
        module.register_fn("process.attr", Derivation::attr);
        module.register_fn("process.script", Derivation::script);
        module.register_fn("process.hash", Derivation::hash);
        module.register_fn("process.display", Derivation::display);
        module
            .register_fn("process.interpolations", Derivation::interpolations);
        module.register_fn(
            "process.set-interpolations",
            Derivation::set_interpolations,
        );
        module.register_fn("process.inward-hashes", Derivation::inward_hashes);
        module.register_value(
            "config",
            dag.config
                .clone()
                .into_steelval()
                .expect("couldn't insert config into process vm"),
        );
        module.register_value(
            "out-hash-placeholder",
            OUT_PLACEHOLDER
                .into_steelval()
                .expect("Couldn't convert OUT_PLACEHOLDER to steelval"),
        );
        vm.register_module(module);

        vm.register_steel_module(
            "process".to_string(),
            include_str!("steel-modules/process.scm").to_string(),
        );
        vm.run(r#"(require "process")"#)
            .expect("Couldn't require process module!");
    }

    pub fn add_process(
        &mut self,
        mut derivation: Derivation,
    ) -> Result<Derivation, InsertError<String>> {
        derivation.write_hash();

        let hash = derivation
            .clone()
            .hash
            .expect("Derivation hash has no value!");

        self.nodes.safe_insert(hash.clone(), derivation.clone())
    }

    pub fn node_count(&self) {
        println!("{}", self.nodes.len())
    }

    pub fn display_nodes(&self) {
        println!("{:?}", self.nodes)
    }

    pub fn outputs(&self) {
        todo!()
    }
}

pub struct InsertError<K> {
    node_id: K,
}

impl<K: std::fmt::Display + 'static> Custom for InsertError<K> {}

impl<K: std::fmt::Display> std::fmt::Display for InsertError<K> {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> Result<(), std::fmt::Error> {
        write!(f, "Node already in graph: {}", self.node_id)
    }
}

trait SafeInsert<K, V> {
    fn safe_insert(&mut self, key: K, value: V) -> Result<V, InsertError<K>>;
}

impl<K: Eq + Hash, V: Clone> SafeInsert<K, V> for HashMap<K, V> {
    fn safe_insert(&mut self, key: K, value: V) -> Result<V, InsertError<K>> {
        match self.get(&key) {
            Some(_) => Err(InsertError { node_id: key }),
            None => {
                self.insert(key, value.clone());
                Ok(value)
            }
        }
    }
}
