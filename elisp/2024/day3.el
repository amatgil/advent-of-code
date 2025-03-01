;;; -*- lexical-binding: t -*- 
(load-file "utils.el")

(setq sample "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))")
;              ^^^^^^^^--------------------^^^^^^^^----------------^^^^^^^^^^^^^^^^^

(defun chardigit-to-digit (c) (- c ?0))
(defun is-digit (d) (and (<= ?0 d) (>= ?9 d)))

(defun digitlist-to-int (l)
  "Expects non-empty list of digits. If it is empty, it will return 0. If it
does not contain digits, output is undefined"
  (if (null l) 0
    (+ (- (car (last l)) ?0)
       (* 10 (digitlist-to-int (butlast l))))))

(defun parse-int (s)
  "Expects l to be a list of chars, not a string. Returns either nil or a
cons cell of the number and the non-consumed part of the string"
  (let ((digits (-take-while (lambda (c) (is-digit c)) s)))
    (if (equal 0 (length digits)) (cons nil s)
      (cons (digitlist-to-int digits) (-drop (length digits) s)))))

(defun dropprefix (l)
  "returns (t . rest) or (nil . input)"
  (if (equal mulprefix (-take 4 l))
      (cons t (-drop 4 l))
    (cons nil l)))

(defun int-length (n)
  (if (equal n 0) 1 (1+ (floor (log10 n)))))

(defmacro parse-prefix (fn-name prefix)
  `(defun ,fn-name (s)
     (if (equal `,(string-to-list ,prefix) (-take ,(length prefix) s))
         (cons (-take ,(length prefix) s) (-drop ,(length prefix) s))
       (cons nil s))))

(parse-prefix parse-mul-prefix "mul(")
(parse-prefix parse-do "do()")
(parse-prefix parse-dont "don't()")
(parse-prefix parse-comma ",")
(parse-prefix parse-close ")")

(defun parse-mul (l)
  "If valid, returns (x y <chars consumed>). If invalid, returns nil"
  (let* ((prefix-ret   (parse-mul-prefix  l))
         (lhs-ret      (parse-int         (cdr prefix-ret)))
         (comma-ret    (parse-comma       (cdr lhs-ret)))
         (rhs-ret      (parse-int         (cdr comma-ret)))
         (close-paren  (parse-close       (cdr rhs-ret))))
    (if (and (car prefix-ret)
             (car lhs-ret)
             (car comma-ret)
             (car rhs-ret)
             (car close-paren))
        (list (car lhs-ret) (car rhs-ret) (+ 4   ; prefix
                                             (int-length (car lhs-ret))
                                             1   ; comma
                                             (int-length (car rhs-ret))
                                             1)) ; close paren
      nil)))

(defun extract-mul-pairs (s)
  (let ((acc 0))
    (while (not (null s))
      (let ((r (parse-mul s)))
        (if r (progn
                (setq acc (+ acc (apply #'* (-take 2 r))))
                (setq s (-drop (caddr r) s)))
          (setq s (cdr s)))))
    acc))

(defun toggleable-extract-mul-pairs (s)
  (let ((acc 0)
        (enabled 't))
    (while (not (null s))
      (let ((r (parse-mul s))
            (do (parse-do s))
            (dont (parse-dont s)))
        (cond ((car dont)
               (setq enabled nil)
               (setq s (cdr dont)))
              ((car do) 
               (setq enabled 't)
               (setq s (cdr do)))
              ((and r enabled) 
               (setq acc (+ acc (apply #'* (-take 2 r))))
               (setq s (-drop (caddr r) s)))
              ('t (setq s (cdr s))))))
    acc))

(defun part1 (input)
  (extract-mul-pairs (string-to-list input)))

(defun part2 (input)
  (toggleable-extract-mul-pairs (string-to-list input)))

(print (format "Part 1: %d" (part1 (f-read "../inputs/2024-03.txt"))))
(print (format "Part 2: %d" (part2 (f-read "../inputs/2024-03.txt"))))
