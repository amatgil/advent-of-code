(defmacro curry (f &rest bound) `(lambda (x) (,f ,@bound x)))

(defmacro backcurry (bound f)   `(lambda (x) (,f ,@bound x)))

(defun trim-trailing-newline (text) (string-right-trim "\n" text))

(defun split-on-newline (text) (uiop:split-string (trim-trailing-newline text)
                                                  :separator '(#\linefeed)))

(defun string-to-list (s) (append s nil))
