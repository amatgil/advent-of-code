(defmacro curry (f &rest bound)
  `(lambda (x) (,f ,@bound x)))

(defmacro backcurry (bound f)
  `(lambda (x) (,f ,@bound x)))

(defun trim-trailing-newline (text)
  (string-trim-right text "\n"))
