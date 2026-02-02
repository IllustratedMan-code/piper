# Architecture

Processes are "compiled" into derivations, then the
derivations are assembled into a directed acyclic graph.

Since processes must be defined with incoming edges included,
there is no way to add a cycle. The steel interpreter will
error out because the process has not been defined yet if a cycle is
attempted. This is because the program is defined linearly, i.e. multiple processes
cannot be defined at the same time. This is only possible if the steel language if processes can only be defined in the main thread. What this means is that
derivations can be defined as recursive data structures, i.e structs that can contain
instances of other structs of the same type.

Recursive data structures in Rust are a pain in the ass, and require unsafe code. Instead, it is recommended to use an existing data structure. In this case, I'm using a hashmap.

The user should be able to use other pipelines. I think that a pipeline can be an optionally parametrizable set of derivations. A pipeline function could pull the required scheme files from a git repo or url (including a file url).

So, we'll need at minimum, 4 scheme functions/macros.

1. `process`: A functiont that create a derivation
2. `outputs`: A function that identifies the endpoint derivations
3. `path`: A function that loads an external path: (a special instance of a derivation)
4. `pipeline`: A function that returns a set of derivations corresponding to the `outputs` function in another pipeline.

So the program defines a set of derivations which are connected to each other by directed edges. The graph is acyclic since edges can only be defined at node creation. Cycle detection would have to be in place for external pipelines only (this could be problematic). Once all the derivations are created, they have to be evaluated, which should occur after all the scheme code has run. There will be two "backends" for each process. The workflow manager (i.e. slurm/LSF) and the container (docker/apptainer/singularity).

Since `process` functions can be wrapped in functions, derivations can be parameterized. This should enable unit-like testing even though derivations are unique. Unit testing should not be required unless one wants their code to be particularly parameterizable (i.e. used by others). Instead, the pipeline will work more like a makefile. The user defines the dependency tree, and the program evaluates it.

The program should evaluate a configuration script before normal script execution. The interpreter will store all variables and such, with some additional type-safe
parameter functions

1. `config`: used for configuring execution of the program on the computer, but is not required for external pipelines
2. `params`: used for parameterizing the pipeline. This would be required to run someone elses pipeline.

Other definitions will also be loaded into the interpreter, but these should not be used to parameterize the pipeline, only as static definitions. I could block these entirely, but
not sure if that is ideal.

Do we reuse the same VM or do we have multiple VMs for the config and the processes?

## Config files

- default config file path: .piper_config.scm
- can also use --config

## Params

- params can be passed like so --params.x="value"
- params can also be defined in the config
- command line args will override those in the config
- `(param x "value")`
