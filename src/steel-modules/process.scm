
(provide process
	 count-nodes
	 display-nodes
	 DG::config
	 drv::display
	 DG::graph
	 )

(require-builtin DerivationGraph as DG::)

(define (process hashmap)
  (let* ((script (~> (hash-get hashmap 'script)
		     (DG::ScriptString)))
	 (out-hash DG::out-hash-placeholder))
    (DG::ScriptString::set_interpolations
     script
     (map
      (lambda (x)
	(eval-string
	 (string-append "(let ((out \"" out-hash "\"))" x ")")))
      (DG::ScriptString::interpolations script)
      ))
    (set! hashmap (hash-insert hashmap 'script script)) 
    (~> (DG::Process::new hashmap DG::config)
	(DG::Process::as_derivation))))

;; (define (process hashmap)
;;   (let* ((derivation (dag.process hashmap dag.config))
;; 	 (out-placeholder dag.out-hash-placeholder))
;;     (dag.process.set-interpolations
;;      derivation
;;      (map
;;       (lambda (x) (eval-string (string-append "(let ((out \"" out-placeholder "\"))" x ")")))
;;       (dag.process.interpolations derivation)))
;;     (dag.add-process dag.dag derivation)))

(define (count-nodes)
  (DG::node_count DG::graph))

(define drv::display DG::Derivation::display)

(define (display-nodes)
  (DG::display_nodes DG::graph))



