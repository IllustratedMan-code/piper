//! Module for creating the process graph.
//! Processes are "compiled" into derivations which
//! form the nodes of the DAG
use daggy::Dag;
use steel::rvals::IntoSteelVal;
use std::collections::HashMap;
use steel::SteelVal;
use steel::steel_vm::engine::Engine;
use steel::steel_vm::builtin::BuiltInModule;
use steel::steel_vm::register_fn::RegisterFn;
use steel_derive::Steel;


mod derivation;
use derivation::Derivation;
use derivation::SafeInsert;

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


        //module.register_fn("process", DAG::process);
        module.register_fn("node_count", DAG::node_count);
        module.register_fn("display_nodes", DAG::display_nodes);


        module.register_type::<Derivation>("Derivation?");
        module.register_fn("process", Derivation::new);
        module.register_fn("process.attr", Derivation::attr);
        module.register_fn("process.script", Derivation::script);
        module.register_fn("process.hash", Derivation::hash);
        module.register_fn("process.interpolations", Derivation::interpolations);
        module.register_fn("process.set-interpolations", Derivation::set_interpolations);
        module.register_fn("process.inward-hashes", Derivation::inward_hashes);
        module.register_fn("add-process", DAG::add_process);
        dag.vm.register_module(module);


        dag.vm
            .register_steel_module(
                "process".to_string(),
                include_str!("steel-modules/process.scm").to_string(),
            );
        dag.vm.run(r#"(require "process")"#).unwrap();


        

        return dag;
    }

    pub fn add_process(&mut self, mut derivation: Derivation) -> Result<Derivation, String> {
        derivation.write_hash();
        match self.nodes.safe_insert(derivation.clone().hash.unwrap(), derivation.clone()) {
            Ok(_) => {
                self.dag.add_node(derivation.clone().hash.unwrap());
                Ok(derivation)},
            Err(_) => Ok(derivation)
            
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


