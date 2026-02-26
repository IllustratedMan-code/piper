

(provide process
	 process!
	 file
	 count-nodes
	 display-nodes
	 DG::config
	 drv::display
	 DG::graph
	 )

(require-builtin DerivationGraph as DG::)

(define (process hashmap #:bindings [bindings '()])
  (let* ((script (~> (hash-get hashmap 'script)
		     (DG::ScriptString)))
	 (out-hash DG::out-hash-placeholder))
    (DG::ScriptString::set_interpolations
     script
     (map
      (lambda (x)
	(eval
	 (quasiquote
	  (let ,(append `((out ,out-hash)) bindings)
	    ,(with-input-from-string x read) ))))
      (DG::ScriptString::interpolations script)
      ))
    (set! hashmap (hash-insert hashmap 'script script)) 
    (define derivation (DG::add_derivation DG::graph
     (~> (DG::Process::new hashmap DG::config)
	 (DG::Process::as_derivation)
	 ))
      )
    derivation
    ))


(define-syntax process!
  (syntax-rules ()
    [(_ (bindings) rest ...)
     (process
      (process-helper rest ...)
      #:bindings (bindings-helper (bindings))  )]
    [(_ rest ...) (process (process-helper rest ...))]

  ))
	
(define (file path #:hashMethod [hashMethod DG::File::HashTimestamp])
  (let* ((derivation
	  (~> (DG::File::new path hashMethod)
	      (DG::File::as_derivation))))
    (DG::add_derivation DG::graph derivation)
  ))

(define-syntax bindings-helper
  (syntax-rules ()
    [(_ ())
     '()]
    [(_ (var ...))
     `((var ,var) ...)]))

(define-syntax process-helper
  (syntax-rules (:)
    [(_ key : val)
     (hash 'key val)]
    [(_ key : val rest ...) (hash-union (process-helper rest ...) (hash 'key val))]
    ))



(define (count-nodes)
  (DG::node_count DG::graph))

(define drv::display DG::Derivation::display)

(define (display-nodes)
  (DG::display_nodes DG::graph))


