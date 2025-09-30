(define x 5)

(require-builtin process/dag as dag.)


(define proc1 (process
 (hash 'name "first-process"
       'script "
        echo ${(+ 5 6 2 x)}")))


(define proc2 (process
 (hash 'name "cool-process"
       'script "
        echo ${proc1} ${(+ 1 2 3)} ${proc1} ")))


