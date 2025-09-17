

(define proc (process
 (hash 'name "cool-process"
       'script "
        echo ${(+ 1 2 3 4)}
      ")))
