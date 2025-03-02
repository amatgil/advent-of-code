(load "utils.lisp")

(defvar sample "3   4
4   3
2   5
1   3
3   9
3   3
") ; trailing newline!

(defun parse (rawinput)
  (let ((ret (reduce (lambda (acc x)
                       (list
                        (cons (parse-integer (car x)) (car acc))
                        (cons (parse-integer (cadr x)) (cadr acc))))
                     (butlast (mapcar
                               (lambda (l) (let ((split (uiop:split-string l :separator " ")))
                                             (list (car split) (car (last split)))))
                               (split-on-newline rawinput)))
                     :initial-value '(() ()))))
    (list (reverse (car ret)) (reverse (cadr ret)))))

(defun part1 (input)
  (let ((parsed (parse input)))
    (reduce #'+
            (mapcar (lambda (x y) (abs (- x y)))
                    (sort (car parsed) #'<)
                    (sort (cadr parsed) #'<)))))

(defun part2 (input)
  (reduce #'+
          (let* ((parsed (parse input))
                 (left (car parsed))
                 (right (cadr parsed)))
            (mapcar #'*
                    (mapcar (lambda (l)
                              (length (remove-if-not (curry equal l) right)))
                            left)
                    left))))


(format t "~&Part 1: ~A~&" (part1 (uiop:read-file-string "../inputs/2024-01.txt")))
(format t "~&Part 2: ~A~&" (part2 (uiop:read-file-string "../inputs/2024-01.txt")))


