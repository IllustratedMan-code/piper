# Architecture

Below is an informal description of why piper exists and how it works. For a detailed description of the syntax, check [GRAMMER.md](./GRAMMAR.md)

## The problem

Scientific analysis often requires the connection of many different tools.
Analysts often create a series of scripts that generate intermediate files. Assuming the analyst is
organized, they might label the script with the appropiate step in the analysis.

```
script1.sh
script2.sh
script3.sh
```

This is better than a single script as only a portion of the scripts need to be rerun if they change
during development. For example, if `script3.sh` were to change, only `script3.sh` would need to be rerun.
However, if `script1.sh` is changed, than all the scripts need to be rerun. In a single script model,
the entire script needs to be rerun after every change.

This requires the analyst to be conscious of every change they make, rerunning the appropriate scripts as they go.
As with every human process, the analyst will not be correct every time. They may forget to run an appropriate step,
or they may not ensure that each step finishes correctly before beginning the next step.

Additionally, there are many times when an analyst will need to use different environments for their scripts at different
steps. `script1.sh` might requires version 1.0 of an R package, while script3.sh may require version 2.0 of the same R package.
One way of handling this problem is with containers, though this puts more pressure on the analyst to remember which container goes
to which script.

There are also many times when a script will require significant amounts of computing resources. High performance cluster environments
allow one to request computational resources for specific scripts, though they often do not provide an easy way to ensure a script
ran successfully. In the case where the analyst must run one script after another, they must wait for the execution of one script
to finish before starting another, to ensure the script ran successfully.

## The solution: Pipeline managers

To assuage these problems with analysis, computational tools have popped up to handle the connections between scripts. Snakemake
and Nextflow are the two primary tools for this, each having their own set of problems. Piper (this tool), is also
meant to solve these analysis problems and create a minimum of friction for the analyst. Piper was born out
of the author's pain using existing solutions.

## Piper's evaluation model

Piper models scripts or "processes" as nodes in a directed acyclic graph. Nodes can depend upon other nodes, however there are
always nodes in the graph with no dependencies. Cycles, or loops in the graph are also guaranteed to not exist.

TODO! (add image of directed acyclic process graph).

Any node in the graph is known as a "derivation", which is evaluated after the entire graph is created. A derivation is
associated with a unique id, created by hashing the inputs as well as the contents of the derivation itself. For example,
if process A depends on process B, then the hash of process A is based on the script associated with process A as well as
the hash of process B. This means that the hashes of nodes with no inward edges must be computed first, then the nodes that connect to those nodes must
be computed and so on and so forth.

This hashing process ensures that when a node changes, all nodes that depend on that node also change. A cache is created for each
hash, so derivations will only be evaluated once. If the hash changes, piper considers the derivation to be different than it was before,
connected in no way to the previous version. A new cache would be created for the new hash. The old cache will not be deleted and must be manually garbage collected
by piper. This way, if the developer of a piper pipeline makes a change to a script, then reverts that change, the cache will still exist.

There are three types of derivation, a process derivation, a file derivation, and an output derivation.

Process derivations haver already been introduced, but they contain a script, typically in a shell language like bash, that creates output files when it
is run. The process outputs are cached after it is run, which occurs after the pipeline has been defined.

File derivations do not run scripts, instead they simply cache the contents of a file. These exist to make it easy to include scripts
written in various languages easy to integrate into the pipeline. For example, process derivation "A" could make use of file derivation "python_script"
when it is run. This ensures that if any changes are made to the file in the file derivation, process derivation A will be rerun and recached.

There is only one output derivation per pipline, defining the root, or "apex" of the process graph. Any process nodes connected to the output node will
be run when the pipeline runs. The output of the output derivation will also not be contained in the work directory, but in an "output" directory defined in the config.

That's it! Piper's model is intended to be intuitive, requiring a minimum amount of time to learn.

## Using piper

Piper pipelines are created using scheme. Pipelines are evaluated after all of the scheme code is run. The scheme interpreter is embedded into the piper program. The version of scheme used in this project is named ["Steel"](https://github.com/mattwparas/steel), a written by Matthew Paras. Steel is embedded into piper, which injects the interpreter with custom syntax and functions for pipeline creation.

### Why scheme?

Scheme is a type of lisp, a family of languages that have very simple syntax that can be extended with "macros", a feature very few languages have.
These macros allow the creation of new syntax features without much effort, making lisp an ideal choice as a base for domain specific languages (DSL).

Derivations are not functions, but rather macros, so the syntax can be adjusted
to make them more intuitive. This should make it easier for newcomers to scheme to get right into pipeline development.

# Hash invalidation

- hashes are computed from the contents of the derivation, so a change to a preceding derivation should change the hashes of all decending derivations.
- TODO Need to ensure that the user cannot accidentally modify a symlink by setting it to read only
  - So, calculate the output, then set all files in output to read only.
  - This is probably how nix does it, though it has an easier time because everything in the nix store is guaranteed to be read-only
  - This is another problem with nextflow :(
- TODO Need a way to invalidate the hash of a node and all its dependents in the case of derivations that depend on external resources (like sql queries to external databases). Hashes won't change normally as the script won't change, but the outputs will.
- TODO Implement a "run derivation" function that runs only the graph defined by a particular derivation, useful for repl based evalutations and should "just work" for the output derivation.

## Other notes

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
