use steel::steel_vm::engine::Engine;
use steel::steel_vm::register_fn::RegisterFn;
use super::process::{DAG};
use steel::SteelVal;

pub fn engine() -> Engine {
    let vm = Engine::new();
    let dag = DAG::new(vm);
    return dag.vm;
}
