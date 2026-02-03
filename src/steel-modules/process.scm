
(provide process
	 count-nodes
	 display-nodes
	 dag.config
	 process.display
	 dag.dag
	 )

(require-builtin process/dag as dag.)

(define (process hashmap)
  (let* ((derivation (dag.process hashmap))
	 (out-placeholder dag.out-hash-placeholder))
    (dag.process.set-interpolations
     derivation
     (map
      (lambda (x) (eval-string (string-append "(let ((out \"" out-placeholder "\"))" x ")")))
      (dag.process.interpolations derivation)))
    (dag.add-process dag.dag derivation)))

(define (count-nodes)
  (dag.node_count dag.dag))

(define process.display dag.process.display)

(define (display-nodes)
  (dag.display_nodes dag.dag))



