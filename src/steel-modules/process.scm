
(provide process count-nodes display-nodes)

(require-builtin process/dag as dag.)

(define (process map)
  (dag.process dag.dag map))

(define (count-nodes)
  (dag.node_count dag.dag))

(define (display-nodes)
  (dag.display_nodes dag.dag))



