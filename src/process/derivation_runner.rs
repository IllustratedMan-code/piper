use crate::process::derivation::evaluator::HPCRuntimeFunctions;
use crate::process::{ProcessGraph, derivation::Derivation};
use std::collections::HashSet;
use std::collections::VecDeque;

impl ProcessGraph {
    /// runs arbitrary derivation based on its hash
    pub fn run_derivation(
        &self,
        derivation_hash: String,
    ) -> Result<(), String> {
        let mut run_order = VecDeque::<Vec<Derivation>>::new();
        let mut stop = false;
        let root = self
            .nodes
            .get(&derivation_hash)
            .ok_or("Derivation not in process graph".to_string());
        run_order.push_back(vec![root?.clone()]);
        while !stop {
            let mut iteration: Vec<Derivation> = Vec::new();
            let last_iter = run_order.back().expect(
                "no first element of run_order, this should never happen",
            );
            for i in last_iter {
                if let Some(edges) = i.clone().inward_edges {
                    let mut derivations: Vec<Derivation> =
                        edges.iter().map(|edges| {
                            self.nodes.get(edges).expect("inward edges are not in process graph, this should never happen").clone()
                        }).collect();
                    iteration.append(&mut derivations);
                }
            }

            if iteration.is_empty() {
                stop = true
            } else {
                run_order.push_back(iteration);
            }
        }

        let mut i = 0;
        for iteration in run_order {
            i += 1;
            println!(
                "run iteration: {}, {:?}",
                i,
                iteration
                    .iter()
                    .map(|v| v.hash.clone())
                    .collect::<Vec<Option<String>>>()
            );
            let handles: Vec<
                Option<crate::process::derivation::evaluator::HPCRuntime>,
            > = iteration
                .iter()
                .map(|derivation| derivation.run())
                .collect();

            for mut handle in &mut handles.into_iter().flatten() {
                handle.wait();
            }
        }

        Ok(())
    }

    /// runs outputs derivation
    fn run(&self) -> Result<(), String> {
        // need to replace with custom error type for derivations

        todo!()
    }
}
