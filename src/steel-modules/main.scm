(define x 5)


(define proc1 (process
 (hash 'name "first-process"
       'script "
        mkdir -p ${out}
        echo ${(+ 5 6 2 x)} > ${out}/result.txt"
       'time 5
       'memory 5
 
       )))



(define proc2 (process
 (hash 'name "cool-process"
       'script "
         cat ${proc1}/result.txt > ${out}"
       'time 5
       'memory 5
)))



(define proc3 (process
 (hash 'name "third-process"
       'script "
        mkdir -p ${out}
        echo ${(+ 5 7 2 x)} > ${out}/result.txt"
       'time 5
       'memory 5
 
       )))
