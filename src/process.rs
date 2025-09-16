//! Module for creating the process graph.
//! Processes are "compiled" into derivations which
//! form the nodes of the DAG
use daggy::Dag;
use steel::rvals::IntoSteelVal;
use std::collections::HashMap;
use std::hash::Hash;
use steel::SteelVal;
use steel::steel_vm::engine::Engine;
use steel::steel_vm::builtin::BuiltInModule;
use steel::steel_vm::register_fn::RegisterFn;
use steel_derive::Steel;

mod derivation;

/// Directed Acyclic Graph containing the derivation nodes
#[derive(Clone, Steel)]
pub struct DAG {
    dag: Dag<String, i32>,
    nodes: HashMap<String, derivation::Derivation>,
    pub vm: Engine,
}

impl DAG {
    pub fn new(vm: Engine) -> DAG {
        let mut dag = DAG {
            dag: Dag::<String, i32>::new(),
            nodes: HashMap::<String, derivation::Derivation>::new(),
            vm: vm,
        };

        let mut module = BuiltInModule::new("process/dag");
        

        
        module.register_type::<DAG>("DAG?");
        module.register_value("dag", dag.clone().into_steelval().unwrap());

        module.register_fn("process", DAG::process);
        module.register_fn("node_count", DAG::node_count);
        module.register_fn("display_nodes", DAG::display_nodes);
        dag.vm.register_module(module);
        dag.vm
            .register_steel_module(
                "process".to_string(),
                include_str!("steel-modules/process.scm").to_string(),
            );
        dag.vm.run(r#"(require "process")"#).unwrap();
        return dag;
    }

    pub fn process(&mut self, attributes: HashMap<SteelVal, SteelVal>) -> Result<String, String> {
        match derivation::Derivation::new(attributes) {
            Ok(v) => {
                let hash = v.hash.clone();
                match self.nodes.safe_insert(hash.clone(), v) {
                    Ok(_) => {
                        self.dag.add_node(hash.clone());
                        ()
                    }
                    Err(_) => return Ok(hash),
                }

                // TODO add duplicate checking
                Ok(hash)
            }
            Err(v) => Err(v),
        }
    }

    pub fn node_count(&self) {
        println!("{:?}", self.dag.graph().node_count());
    }

    pub fn display_nodes(&self) {
        println!("{:?}", self.nodes)
    }

    pub fn outputs(&self) {
        todo!()
    }
}

trait SafeInsert<K, V> {
    fn safe_insert(&mut self, key: K, value: V) -> Result<(), ()>;
}

impl<K: Eq + Hash, V> SafeInsert<K, V> for HashMap<K, V> {
    fn safe_insert(&mut self, key: K, value: V) -> Result<(), ()> {
        match self.get(&key) {
            Some(_) => Err(()),
            None => {
                self.insert(key, value);
                Ok(())
            }
        }
    }
}
