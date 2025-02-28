(load-file "utils.el")

(setq sample "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
")

(defun parse (input)
  (mapcar (lambda (l) (mapcar #'string-to-number
                              (string-split l " ")))
          (string-split (trim-trailing-newline input) "\n")))

(defun stencil (n xs)
  "Assumes length is multiple of n !!"
  (if (< (length xs) n) nil
    (cons (-take n xs)
          (stencil n (nthcdr (1- n) xs)))))

(defun is-monotonic (xs)
  (let ((steps (mapcar
                (curry apply #'-)
                (stencil 2 xs))))
    (or (-all? (curry > 0) steps)
        (-all? (curry < 0) steps))))

(defun grows-slowly (xs)
  (-all?
   (lambda (pair) (let ((delta (abs (apply #'- pair))))
                    (and (<= 1 delta) (>= 3 delta))))
   (stencil 2 xs)))

(stencil 2 '(1 2 3 4 5))
(mapcar
 (curry apply #'-)
 (stencil 2 (car (parse sample))))

(mapcar
 (lambda (pair) (let ((delta (abs (apply #'- pair))))
                  (and (<= 1 delta) (>= 3 delta))))
 (stencil 2 (car (parse sample))))

(defun part1 (input)
  (-count 
   (lambda (l) (and (grows-slowly l)
                    (is-monotonic l)))
   (parse input)))

(print (format "Part 1: %d" (part1 (f-read "../inputs/2024-02.txt"))))
