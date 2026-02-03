use crate::process::{derivation::Derivation, ProcessGraph};

struct DerivationRunner {
    graph: ProcessGraph
}

impl DerivationRunner {
    fn run_derivations(&self) {
        let seen_hashes = std::collections::HashSet::<String>::new();
        let mut run_order: Vec<Vec<Derivation>> = vec![];

        
    }
}



