use std::collections::HashSet;
use crate::process::derivation::evaluator::HPCRuntimeFunctions;

use crate::process::{ProcessGraph, derivation::Derivation};

pub struct DerivationRunner {
    pub graph: ProcessGraph,
}

impl DerivationRunner {
    pub fn run_derivations(&self) {
        let mut run_order: Vec<Vec<Derivation>> = vec![];
        let mut nodes = self.graph.nodes.clone();
        let mut seen_nodes = HashSet::<String>::new();
        while !nodes.is_empty() {
            let mut run_iteration = vec![];
            let mut nodes_to_remove = vec![];
            for (hash, derivation) in &nodes {
                let can_run = match &derivation.inward_edges {
                    Some(edges) => {
                        edges.iter().all(|dep| seen_nodes.contains(dep))
                    }
                    None => true,
                };
                if can_run {
                    run_iteration.push(derivation.clone());
                    nodes_to_remove.push(hash.clone());
                }
            }

            for hash in nodes_to_remove {
                nodes.remove(&hash);
                seen_nodes.insert(hash);
            }

            if run_iteration.is_empty() {
                panic!(
                    "Cycle detected in process graph! This should never happen."
                )
            }

            run_order.push(run_iteration)
        }
        let mut i = 0;
        for iteration in run_order {
            i += 1;
            println!("run iteration: {}, {:?}", i, iteration.iter().map(|v| v.hash.clone()).collect::<Vec<Option<String>>>());
            let handles: Vec<Option<crate::process::derivation::evaluator::HPCRuntime>> = iteration
                .iter()
                .map(|derivation| derivation.run())
                .collect();


            for mut handle in &mut handles.into_iter().flatten() {
                    handle.wait();
            }

        }
    }
}
