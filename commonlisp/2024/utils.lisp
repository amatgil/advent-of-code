(defmacro curry (f &rest bound) `(lambda (x) (,f ,@bound x)))

(defmacro backcurry (bound f)   `(lambda (x) (,f ,@bound x)))

(defun trim-trailing-newline (text) (string-right-trim "\n" text))

(defun split-on-space (text) (uiop:split-string (trim-trailing-newline text)
                                                  :separator " "))
(defun split-on-newline (text) (butlast (uiop:split-string text
                                                     :separator '(#\linefeed))))
(defun take (n l) (subseq l 0 (min n (length l))))

(defun string-to-list (s) (coerce s 'list))

(defun take-while (f l)
  (if (null l) nil
      (if (funcall f (car l)) (cons (car l)
                              (take-while f (cdr l)))
          nil)))

(defun flatten (l)
  (cond ((null l) nil)
        ((null (car l)) (flatten (cdr l)))
        ('t (cons (car (car l))
                  (concatenate 'list
                               (cdr (car l))
                               (flatten (cdr l)))))))
