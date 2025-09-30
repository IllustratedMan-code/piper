
(provide process
	 count-nodes
	 display-nodes
	 dot-viz)

(require-builtin process/dag as dag.)

(define (process hashmap)
  (let ((derivation (dag.process hashmap)))
    (dag.process.set-interpolations derivation (map (lambda (x) (eval-string x)) (dag.process.interpolations derivation)))
    (dag.add-process dag.dag derivation)))

(define (count-nodes)
  (dag.node_count dag.dag))

(define (display-nodes)
  (dag.display_nodes dag.dag))

(define (dot-viz)
  (dag.dot-viz dag.dag))


