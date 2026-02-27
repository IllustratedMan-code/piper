(define x 5)

(define file1 (file! "src/main.rs"))

(define proc1
  (process!
   name : "first-process"
   container : "ubuntu:latest"
   script : "
        mkdir -p ${out}
        echo ${(+ 5 6 2 x)} > ${out}/result.txt
        cp ${file1} ${out}/script.rs"))


(define proc2
  (process!
   name : "second-process"
   time : 5
   memory : 5
   script : "
          cat ${proc1}/result.txt > ${out}"
))



(output!
 "results/proc2" : proc2)
