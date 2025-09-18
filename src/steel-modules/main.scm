
(define x 5)

(require-builtin process/dag as dag.)

(define proc (process
 (hash 'name "cool-process"
       'script "
        echo ${(+ 5 6 2 )}
")))


