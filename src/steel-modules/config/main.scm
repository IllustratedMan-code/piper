(provide param
	 config
	 Config.config
	 )

(require-builtin config as Config.)


(define-syntax param
  (syntax-rules ()
    ((param name value)
     (Config.insert_param Config.config (symbol->string 'name) value))))


(define-syntax config
  (syntax-rules ()
    ((param name value)
     (Config.insert_config Config.config (symbol->string 'name) value))))

