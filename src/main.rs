mod process;
mod vm;
use vm::engine;

fn main() {
    let mut engine = engine();
    engine.run(include_str!("steel-modules/main.scm")).unwrap();
    steel_repl::run_repl(engine).unwrap();
}
