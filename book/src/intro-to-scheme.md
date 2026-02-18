# Introduction to Scheme

Piper pipelines are written in [scheme](<https://en.wikipedia.org/wiki/Scheme_(programming_language)>), the particular variant of which is called [Steel](https://github.com/mattwparas/steel). Scheme is in the family of lisps, languages which are easy to create. Many users of Piper may not have ever used or even heard of scheme, but not to fear! Scheme is a very simple language and can be learned in only a few minutes.

Scheme has 2 easy concepts and one more difficult one to master. The first two concepts are
most likely familar to readers coming from any other programming language.

# Variables

Scheme has variables, big shocker! Variables are dynamically typed, like many other interpreted languages such as python or R. They can be defined like so:

```scheme
(define x 5) ;; x is set to the value of 5
x ;; returns 5
```

While not as convenient as a typical assignment operator, like `x=5`, `define` fits the syntax of scheme, conforming to its minimalist philosophy.

## Functions

It should be noted that scheme does not have operators, so statements like `5 + 6` are invalid in scheme. This may seem unintuitive, but only if you aren't [Jan Łukasiewicz](https://en.wikipedia.org/wiki/Polish_notation). Instead of operators, functions are used instead, so `5+6` is instead written `(+ 5 6)`.

Function syntax (if you haven't caught on already) is `(function_name args)`.

Functions calls can be nested:

```scheme

(+ 5 (+ 5 6)) ;; 5 + 5 + 6

```

functions can be defined like so:

```scheme
(define (plus2 x)
	(+ x 2))

(plus2 2) ;; returns 4
```

And that's it! You've almost learned scheme. So far, it might not seem that useful, why not use some other, more powerful language? The reader might be banging their fists against their desk, shouting "No loops? I need my loops!". Perhaps frightening others in the office.

Scheme's last feature, the "macro", makes scheme much more powerful for Piper's use-case and solves the lack of other features.

## Macros

This feature, unlike the previous two, is present in very few languages. It is
the most important feature, while also the most controversial. So what are macros?

Drumroll please.....

Macros allow the developer to add **new syntax** to scheme! The careful reader might
have noticed something fishy about the define statements in the previous examples.

The first time it was used, it defined a variable, but the second time, it defined a function. Yet how could `define` itself be a function? If it were, using it as we did in either case would result in a syntax error. The experienced programmer might have accepted `define` as a special language feature, like in other languages.

For example, in python:

```python
def add2(x):
	x + 2
```

The python parser knows the keyword `def` and what to expect after it. In python, one cannot create new language keywords. Once the developer has learned them all, that's it, they can read any and all python code.

Scheme, on the other hand, can define new keywords and syntax whenever. So when a scheme developer uses other scheme code, they may have to learn new syntax to use that code. This makes scheme ideal as a base for domain specific languages (DSLs), like Piper's pipelines.

`define` is in fact a macro, or an extensions of the normal functionality of scheme. Since `define` is somewhat integral to the usage of scheme, it isn't really an "extension", but whatever. Macros can be defined by the programmer, but it is quite difficult in comparison to the other features in scheme. The users of Piper are not expected to create their own macros, but many of the features of piper are implemented as macros.

For example:

```scheme


(define proc1
  (process
	(time (hours 1))
	(script ""
	echo "hi world" > ${out}.txt
	"")

(outputs
	("proc1-output.txt" proc1))


```

In this case, `process` and `outputs` are macros.

The syntax of macros is `(macro_name whatever_you_want)`. So anything inside a macro might defy normal scheme syntax. Macros can replicate essentially every other syntax feature of any other language. For example, python's list comprehensions can be [easily implemented as a macro](http://www.phyast.pitt.edu/~micheles/scheme/scheme17.html). The implementation for this version of scheme is left to the reader as an excercise.

Please refer to the [Steel docs](https://mattwparas.github.io/steel/book/introduction.html) for more help with this particular version of scheme.

## Extra examples (for fun!)

At this point the reader is either pulling their hair out in chunks (my loops!), or wants more examples to read.

### Dot operators

Many languages have dot (`.`) operators to access attributes of objects. Scheme does not have those, but in the words of every lisp developer, there's a macro for that. In the case of scheme, this macro is typically called the "threading macro" and is written like `~>`.

```scheme
(define myhashmap (hash 'x 5 'y 6)) ;; returns a hashmap (or dictionary)

(hash-insert
  (hash-insert
	(hash-insert myhashmap 'a 5)
  'b 6)
'c 7) ;; normal way to insert a b and c to hashmap


(~> myhashmap
  (hash-insert 'a 5)
  (hash-insert 'b 6)
  (hash-insert 'c 7)) ;; this has the same result

```

It should be noted that the hash functions here are embedded versions of the equivalent rust functions, implemented by Steel, the version of scheme used in Piper.

### A note about Object Oriented Programming

The pipeline developer cannot create their own types, however many rust types and their associated functions have been implemented into scheme.

If the reader has never used functional languages before, they may not understand that any function associated with a type in languages with OOP are simply "syntactic sugar" for calling a function where the first argument is the object in question.

For example in python:

```python

class myclass:
	def __init__(self):
		self.x = 5
	def add(self, y):
		self.x += y

def external_add(self, y):
	self.x += y

obj = myclass()

obj.add(5)
myclass.add(obj, 5) # is the same
external_add(obj, 5) # is the same

```

So in the above python code, when `obj.add(5)` is called, it is simply syntactic sugar for `myclass.add(obj, 5)`. In the `->` example above, `hash-insert` is a "dot method" in the underlying rust code, but is called without the syntactic sugar in the scheme code. The `~>` macro essentially adds the syntactic sugar back in.

## But Wait!! There's loops!

Scheme does not have loops. Any looping functionality must be implemented recursively. Of course, there is a macro for this as well.

While "loops" can be created with the [`while`](https://mattwparas.github.io/steel/book/stdlib/private_steel_stdlib.html#while) macro.

```scheme
(define x -4)
(while (negative? x) ;; function executed at every iteration
  (set! x (+ x 1))
  (println x))
;; prints -3 -2 -1 0
```

For "loops" can be implemented using `map`, `for-each` can be used if there are no side effects in the `for` body
