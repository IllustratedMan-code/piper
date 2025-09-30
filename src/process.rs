//! Module for creating the process graph.
//! Processes are "compiled" into derivations which
//! form the nodes of the ProcessGraph
use daggy::Dag;

use daggy::petgraph::dot::Dot;
use std::collections::HashMap;
use steel::rvals::IntoSteelVal;
use steel::steel_vm::builtin::BuiltInModule;
use steel::steel_vm::engine::Engine;
use steel::steel_vm::register_fn::RegisterFn;
use steel_derive::Steel;
mod derivation;
use derivation::Derivation;
use derivation::SafeInsert;

/// Directed Acyclic Graph containing the derivation nodes
#[derive(Clone, Steel)]
pub struct ProcessGraph {
    dag: Dag<String, i32>,
    nodes: HashMap<String, derivation::Derivation>,
    pub vm: Engine,
}

impl ProcessGraph {
    pub fn new(vm: Engine) -> ProcessGraph {
        let mut dag = ProcessGraph {
            dag: Dag::<String, i32>::new(),
            nodes: HashMap::<String, derivation::Derivation>::new(),
            vm,
        };

        let mut module = BuiltInModule::new("process/dag");

        module.register_type::<ProcessGraph>("ProcessGraph?");
        module.register_value("dag", dag.clone().into_steelval().unwrap());

        //module.register_fn("process", ProcessGraph::process);
        module.register_fn("node_count", ProcessGraph::node_count);
        module.register_fn("display_nodes", ProcessGraph::display_nodes);
        module.register_fn("outputs", ProcessGraph::outputs);
        module.register_fn("add-process", ProcessGraph::add_process);
        module.register_fn("dot-viz", ProcessGraph::dot_viz);

        module.register_type::<Derivation>("Derivation?");
        module.register_fn("process", Derivation::new);
        module.register_fn("process.attr", Derivation::attr);
        module.register_fn("process.script", Derivation::script);
        module.register_fn("process.hash", Derivation::hash);
        module.register_fn("process.interpolations", Derivation::interpolations);
        module.register_fn("process.set-interpolations", Derivation::set_interpolations);
        module.register_fn("process.inward-hashes", Derivation::inward_hashes);
        dag.vm.register_module(module);

        dag.vm.register_steel_module(
            "process".to_string(),
            include_str!("steel-modules/process.scm").to_string(),
        );
        dag.vm.run(r#"(require "process")"#).unwrap();

        return dag;
    }

    pub fn add_process(&mut self, mut derivation: Derivation) -> Result<Derivation, String> {
        derivation.write_hash();

        let hash = derivation
            .clone()
            .hash
            .expect("Derivation hash has no value!");

        match self.nodes.safe_insert(hash.clone(), derivation.clone()) {
            Ok(_) => {
                let node_index = self.dag.add_node(hash.clone());
                derivation.node_index = Some(node_index);

                self.nodes.insert(hash.clone(), derivation.clone());

                for i in derivation
                    .clone()
                    .inward_edges
                    .expect("No inward edges in derivation!")
                {
                    let Ok(_) = self.dag.add_edge(
                        self.nodes
                            .get(&i)
                            .unwrap_or_else(|| panic!("{:?}", i))
                            .node_index
                            .expect("no index"),
                        node_index,
                        1,
                    ) else {
                        return Err("Adding process node would create a cycle!!".to_string());
                    };
                    // .unwrap_or
                    // .expect("Adding process node would create a cycle!!");
                }
                Ok(derivation)
            }
            Err(_) => Ok(derivation),
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

    pub fn dot_viz(&self) {
        println!("{:?}", Dot::new(&self.dag));
    }
}
