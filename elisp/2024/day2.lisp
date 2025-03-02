(load "utils.lisp")

(defvar sample "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
")

(defun parse (input)
  (mapcar (lambda (l) (mapcar #'parse-integer
                              (split-on-space l)))
          (split-on-newline input)))

(defun stencil (n xs)
  (if (< (length xs) n) nil
    (cons (take n xs) (stencil n (nthcdr (1- n) xs)))))

(defun is-monotonic (xs)
  (let ((steps (mapcar (curry apply #'-) (stencil 2 xs))))
    (or (every (curry > 0) steps)
        (every (curry < 0) steps))))

(defun grows-slowly (xs)
  (every
   (lambda (pair) (let ((delta (abs (apply #'- pair))))
                    (and (<= 1 delta) (>= 3 delta))))
   (stencil 2 xs)))

(defun is-safe (xs)
  (and (grows-slowly xs) (is-monotonic xs)))

(defun variations (l)
  (if (null l) l
    (cons (cdr l)
          (mapcar (curry cons (car l))
                  (variations (cdr l))))))

(defun part1 (input)
  (count-if #'is-safe (parse input)))

(defun part2 (input)
  (count-if (lambda (row)
            (some #'is-safe (variations row)))
          (parse input)))


(format t "~&Part 1: ~A~&" (part1 (uiop:read-file-string "../inputs/2024-02.txt")))
(format t "~&Part 2: ~A~&" (part2 (uiop:read-file-string "../inputs/2024-02.txt")))
