(require-builtin process/dag as dag.)


(define x 5)

(define proc1 (process
 (hash 'name "proc1"
       'script "
        echo ${(+ 1 2 3 x)} ${x}")))


