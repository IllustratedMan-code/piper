use super::process::ProcessGraph;
use steel::steel_vm::engine::Engine;
use super::config::Config;

pub fn engine(config_path: Option<std::path::PathBuf>) -> Engine {
    let mut vm = Engine::new();
    let  c = Config::new(config_path);
    c.register_params(&mut vm);
    ProcessGraph::init(&mut vm, c);
    vm
}

macro_rules! test_scm_file {
    ($file:expr) => {{
        let mut e = engine(None);
        e.run(include_str!($file)).expect("Failed Test");
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_interpolations() {
        test_scm_file!("steel-modules/tests/basic_interpolations.scm")
    }

    #[test]
    fn node_interpolation() {
        test_scm_file!("steel-modules/tests/node_interpolation.scm");

    }

    #[test]
    fn cycle_panic() {
        test_scm_file!("steel-modules/tests/cycle_panic.scm");
    }
}
