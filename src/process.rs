use std::collections::HashMap;
use steel::SteelVal;
use steel::SteelErr;
use daggy::Dag;
use std::fmt::Debug;
use steel_derive::Steel;
use steel::steel_vm::engine::Engine;
use steel::steel_vm::register_fn::RegisterFn;
use std::hash::{DefaultHasher, Hash, Hasher};
use steel::rerrs::ErrorKind;



#[derive(Clone, Steel)]
pub struct DAG {
    dag: Dag::<SteelVal,SteelVal>,
    attributes: HashMap<String, HashMap<SteelVal, SteelVal>>,
    pub vm: Engine
}

impl DAG {

    pub fn new(vm:  Engine) -> DAG{
        let mut dag =  DAG {dag: Dag::<SteelVal,SteelVal>::new(), attributes: HashMap::new(), vm: vm};

        dag.vm.register_type::<DAG>("DAG?");
        dag.vm.register_external_value("dag", dag.clone()).unwrap();

        dag.vm.register_fn("process", DAG::process);
        dag.vm.register_fn("node_count", DAG::node_count);
        return dag
    }

    pub fn process(&mut self, attributes: HashMap<SteelVal,SteelVal>) -> Option<String> {
        let name = attributes.get(&SteelVal::SymbolV("name".into())).unwrap();
        let script = match attributes.get(&SteelVal::SymbolV("script".into())) {
            Some(v) => v,
            None => {
                let err = "script has no value";
                self.vm.raise_error(SteelErr::new(ErrorKind::Parse, err.to_string()));
                return None
            }
        };
        self.dag.add_node(name.clone());
        self.attributes.insert(name.clone().to_string(), attributes.clone());
        let hash = DAG::calculate_hash(name.to_string(), script.to_string());
        Some(hash)
    }

    pub fn node_count(&self){
        println!("{:?}", self.dag.graph().node_count());
        println!("attributes: {:?}", self.attributes);
    }

    pub fn outputs(&self){
        
        todo!()
    }

    fn calculate_hash(name: String, script: String) -> String {
        let mut s = DefaultHasher::new();
        let combined = format!("{}{}", name, script);
        combined.hash(&mut s);
        let hash = s.finish().to_string();
        return format!("{}-{}", hash, name)
    }
}



