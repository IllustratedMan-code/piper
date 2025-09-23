use steel::steel_vm::engine::Engine;
use super::process::{DAG};

pub fn engine() -> Engine {
    let vm = Engine::new();
    let dag = DAG::new(vm);
    return dag.vm;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_interpolations() {
        let mut e = engine();
        e.run(include_str!("steel-modules/tests/basic_interpolations.scm")).unwrap();
    }

    #[test]
    fn node_interpolation() {
        let mut e = engine();
        e.run(include_str!("steel-modules/tests/node_interpolation.scm")).unwrap();

    }


}
