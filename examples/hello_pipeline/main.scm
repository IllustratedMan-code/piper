

(define proc1 (process
 (hash 'name "proc1"
       'script "
        echo hi there")))


(define proc2 (process
 (hash 'name "proc2"
       'script "
        echo ${proc1} ${out}")))
