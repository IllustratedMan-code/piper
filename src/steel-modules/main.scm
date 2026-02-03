(define x 5)

(require-builtin process/dag as dag.)


(define proc1 (process
 (hash 'name "first-process"
       'script "
        mkdir ${out}
        echo ${(+ 5 6 2 x)} > ${out}/result.txt")))


(define proc2 (process
 (hash 'name "cool-process"
       'script "
        cat ${out}/result.txt
        ${out}
")))


