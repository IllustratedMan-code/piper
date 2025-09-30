use super::process::ProcessGraph;
use steel::steel_vm::engine::Engine;

pub fn engine() -> Engine {
    let vm = Engine::new();
    let dag = ProcessGraph::new(vm);
    dag.vm
}

macro_rules! test_scm_file {
    ($file:expr) => {{
        let mut e = engine();
        e.run(include_str!($file)).expect("Failed Test");
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_interpolations() {
        let mut e = engine();
        e.run(include_str!("steel-modules/tests/basic_interpolations.scm"))
            .expect("Failed Test");
    }

    #[test]
    fn node_interpolation() {
        let mut e = engine();
        e.run(include_str!("steel-modules/tests/node_interpolation.scm"))
            .expect("Failed Test");
    }

    #[test]
    fn cycle_panic() {
        test_scm_file!("steel-modules/tests/cycle_panic.scm");
    }
}
