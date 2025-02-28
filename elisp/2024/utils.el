(defmacro curry (f bound)
  `(lambda (x) (,f ,bound x)))

(defun trim-trailing-newline (text)
  (string-trim-right text "\n"))
