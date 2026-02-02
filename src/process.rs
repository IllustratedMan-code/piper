//! Module for creating the process graph.
//! Processes are "compiled" into derivations which
//! form the nodes of the ProcessGraph
use std::{collections::HashMap, hash::Hash};
use steel::rvals::Custom;
use steel::rvals::IntoSteelVal;
use steel::steel_vm::builtin::BuiltInModule;
use steel::steel_vm::engine::Engine;
use steel::steel_vm::register_fn::RegisterFn;
use steel_derive::Steel;
mod derivation;
use derivation::Derivation;
use super::config::Config;

/// Directed Acyclic Graph containing the derivation nodes
#[derive(Clone, Steel)]
pub struct ProcessGraph {
    nodes: HashMap<String, derivation::Derivation>,
    pub vm: Engine,
    pub config: Config
}



impl ProcessGraph {
    pub fn new(vm: Engine, config: Config) -> ProcessGraph {
        let mut dag = ProcessGraph {
            nodes: HashMap::<String, derivation::Derivation>::new(),
            vm,
            config
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
        module
            .register_fn("process.interpolations", Derivation::interpolations);
        module.register_fn(
            "process.set-interpolations",
            Derivation::set_interpolations,
        );
        module.register_fn("process.inward-hashes", Derivation::inward_hashes);
        module.register_value("config", dag.config.clone().into_steelval().expect("couldn't insert config into process vm"));
        dag.vm.register_module(module);

        dag.vm.register_steel_module(
            "process".to_string(),
            include_str!("steel-modules/process.scm").to_string(),
        );
        dag.vm
            .run(r#"(require "process")"#)
            .expect("Couldn't require process module!");


        dag
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
