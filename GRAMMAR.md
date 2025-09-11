Piper works using "processes" which define the "jobs" that are run on the HPC platform.
`process` is a macro that creates a derivation, a unique set of build instructions. Each derivation is a node in a directed acyclic graph. Nodes with no inward edges are built first,
then their children and so on.

When referencing a derivation in a script string, the variable name resolves to a path.

Each derivation, when built, is stored in a directory inside of a "work" directory. This derivation directory contains a `build` folder and an `out` folder. The out directory can be referenced with the `out` variable that is given to all script strings. Within an interpolation, any scheme code is valid `${(+ 1 2 3 4)}`. Derivation hashes will be calculated from the final evaluated script string (before evaluating `out`) and all process attributes. When referencing the derivation, the path will refer to the `out` path.

```scheme


(define mypath
	(process
		(script ''bash
			echo "hi" > $out/mypath.txt
		''
		)))



(define (MyProcess a)
	(process
		(container "user/repo:tag")
		(memory "1Gb")
		(time a)
		(script ''bash
			cp ${mypath}/mypath.txt ${out}/mypath.txt
			echo "chairs" > ${out}/chairs.txt
		'')))


(outputs
  '("my/path" . (MyProcess "1h") ) ;; copies $out of MyProcess to my/path
)

```

Any attributes are valid in a process, but there are some special ones that modify how a process is run/created.

Special attributes
