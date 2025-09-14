use steel::steel_vm::engine::Engine;
use super::process::{DAG};

pub fn engine() -> Engine {
    let vm = Engine::new();
    let dag = DAG::new(vm);
    return dag.vm;
}
