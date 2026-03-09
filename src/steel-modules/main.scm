(define x 5)

(define myscript
  (file! "src/main.rs"))

(define-syntax ~>
  (syntax-rules ()
    ((_ val) 
     val)
    ((_ val (f args ...) rest ...)
     (let ((self val))
       (~> (f args ...) rest ...)))
    ((_ val f rest ...)
     (~> (f val) rest ...))))

(define metadata (df::read-csv "examples/test.csv"))
(define metadata
  (~> metadata
      (df::with-column
       (~> (df::select-column metadata 'price)
	   (column::map file!)
	   )
       )
  )
)

(define metadata
  (df::map-column metadata 'price file!))

;; This syntax-case macro will parse anything in the "select" statement as a string
;; I can parse this string in rust using a "mini" language for select statements
(define-syntax select
  (lambda (stx)
    (syntax-case stx ()
      ((_ cond ...)
       (symbol->string (syntax->datum #'(cond ...))))))


(define proc1
  (process!
   name : "first-process"
   container : "ubuntu:latest"
   script : #<<''
        mkdir -p "${out}"
        echo ${(+ 5 6 2 x)} > ${out}/result.txt
        ${myscript} ${out}/script-out.txt
	''
   )
  )


(define proc2
  (process!
   name : "second-process"
   time : (hours 5)
   memory : (GB 5)
   script : #<<''
        cat ${proc1}/result.txt > ${out}
   '')
  )


(output!
 "results/proc2-result.txt" : proc2
 "results/proc1" : proc1
 )


















;; (define metadata (read_csv "metadata.csv"))

;; (define metadata (~> metadata
;;     (with-column "files"
;; 		 (~> metadata
;; 		     column "files"
;; 		     apply file!
;; 		     )
;; 		 ))

;; (define proc2_iter
;;   (iter!
;;    proc1
;;    "**.txt"
;;    #:sorting "lexicographic"))

;; (define proc3
;;   (process!
;;    name : "second-process"
;;    time : 5
;;    memory : 5
;;    script : "
;;         cat ${proc2_iter} > ${out}
;;         cat ${proc3_iter} >> ${out}
;;    "))
;; (define proc2_iter2
;;   (generator!
;;    proc2
;;    "**.txt"
;;    #:sorting "lexicographic"))
