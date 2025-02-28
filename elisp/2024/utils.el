(defmacro curry (f bound)
  `(lambda (x) (,f ,bound x)))
