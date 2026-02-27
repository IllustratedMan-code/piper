Pipelines are written in scheme, the particular variant of which is called [Steel](https://github.com/mattwparas/steel), which has support for the [r5rs](https://standards.scheme.org/official/r5rs.pdf) syntax.

Pipelines have access to various functions and macros for building pipelines.

To create a derivation, there are three macros, `file`, `process`, and `outputs`.
After the macro is run, a derivation will be added to the process graph. Once all
the scheme code has run, piper will evaluate all of the derivations connected to the
outputs derivation.

```scheme

(define myfile (file "myscript.py"))

;; Big files and binary files should be hashed by their timestamp
;; instead of their contents, though
;; big files will switch to this hashing method automatically based
;; on the cache_filesize_cutoff config item
(define myfile2 (file "myscript.py" #:hash_method "timestamp")


(define mypath
	(process
		(script ''bash
			mkdir ${out}
			echo "hi" > ${out}/mypath.txt
			python ${myfile2} > ${out}/pyout.txt
		''
		)))


(define (MyProcess message)
	(process
		name : "chairs"
		container : "user/repo:tag"
		memory : "1Gb")
		time : "1h"
		script ''python
			mkdir ${out}
			cp ${mypath}/mypath.txt ${out}/mypath.txt
			echo "chairs" > ${out}/chairs.txt
		''))

(define myiter (process_iter mypath) "*.txt")


(define next_iter (map myiter my_function))

(define final_derivation (to_derivation next_iter))

(outputs
  ("my/path" final_derivation) ;; copies $out of MyProcess to my/path
)

```

Each of these macros evaluates to a derivation object, which is understood to be equivalent to its hash.

Before pipelines are created, piper also loads in a config, which is also defined in scheme. There are two functions here, `config` and `param`, which are used to define configuration items, and parameters, which can be specified on the command line as well as through the configuration file.

```scheme
(config workDir "./workdir")
(param dataFile "/path/to/my/datafile.txt")
```

Config entries will affect the execution of the pipeline, but are predefined, so
trying to add a config entry that isn't known by piper will fail. Config entries
typically have default values.

Parameters are completely arbitrary and none exist that are not defined by the pipeline creator.

Parameters can be accessed within the pipeline under `params.*` (e.g `params.dataDir`)

```scheme

(define datafile (file params.dataFile))

(outputs
	("output_file"  datafile))

```
