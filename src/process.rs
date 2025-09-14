//! Module for creating the process graph.
//! Processes are "compiled" into derivations which
//! form the nodes of the DAG
use daggy::Dag;
use std::collections::HashMap;
use steel::SteelVal;
use steel::steel_vm::engine::Engine;
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
            nodes: HashMap::<String,derivation::Derivation>::new(),
            vm: vm,
        };

        dag.vm.register_type::<DAG>("DAG?");
        dag.vm.register_external_value("dag", dag.clone()).unwrap();

        dag.vm.register_fn("dag_process", DAG::process);
        dag.vm.register_fn("node_count", DAG::node_count);
        dag.vm
            .run(r#"(define (process map) (dag_process dag map))"#)
            .unwrap();
        return dag;
    }

    pub fn process(&mut self, attributes: HashMap<SteelVal, SteelVal>) -> Result<String, String> {
        match derivation::Derivation::new(attributes) {
            Ok(v) => {
                let hash = v.hash.clone();
                self.nodes.insert(hash.clone(), v);
                self.dag.add_node(hash.clone());

                // TODO add duplicate checking
                Ok(hash)
            }
            Err(v) => Err(v),
        }
    }

    pub fn node_count(&self) {
        println!("{:?}", self.dag.graph().node_count());
    }

    pub fn outputs(&self) {
        todo!()
    }
}
