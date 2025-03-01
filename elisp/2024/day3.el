;;; -*- lexical-binding: t -*- 
(load-file "utils.el")

(setq sample "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))")
;              ^^^^^^^^--------------------^^^^^^^^----------------^^^^^^^^^^^^^^^^^

(defun string-to-list (s) (append s nil))
(defun chardigit-to-digit (c) (- c ?0))
(defun is-digit (d) (and (<= ?0 d) (>= ?9 d)))

(defun digitlist-to-int (l)
  "Expects non-empty list of digits. If it is empty, it will return 0. If it does not contain digits, output is undefined"
  (if (null l) 0
    (+ (- (car (last l)) ?0)
       (* 10 (digitlist-to-int (butlast l))))))


(defun parse-int (s)
  "Expects l to be a list of chars, not a string. Returns either nil or a cons cell of the number and the non-consumed part of the string"
  (let ((digits (-take-while (lambda (c) (is-digit c)) s)))
    (if (equal 0 (length digits)) (cons nil s)
      (cons (digitlist-to-int digits) (-drop (length digits) s)))))

(-find-index (lambda (c) (not (is-digit c))) (string-to-list "123"))

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
(parse-prefix parse-do "do(")
(parse-prefix parse-dont "don't(")
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

;; (defun extract-mul-pairs (acc s)
;;   "Expects list of chars, not string"
;;   (if (null s) acc
;;     (let ((r (parse-mul s)))
;;       (if r (extract-mul-pairs
;;              (+ acc (apply #'* (-take 2 r)))
;;              (-drop (caddr r) s))
;;         (extract-mul-pairs acc (cdr s))))))

(defun extract-mul-pairs (s)
  (let ((acc 0))
    (while (not (null s))
      (let ((r (parse-mul s)))
        (if r (progn
                (setq acc (+ acc (apply #'* (-take 2 r))))
                (setq s (-drop (caddr r) s)))
          (setq s (cdr s)))))
    acc))

(defun part1 (input)
  (extract-mul-pairs (string-to-list input)))

(part1 sample)

;(setq max-lisp-eval-depth 1000000)
(print (format "Part 1: %d" (part1 (f-read "../inputs/2024-03.txt"))))
