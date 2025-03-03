(load "./utils.lisp")

(defparameter sample
"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
")

(defun is-prefix-p (prefix other)
  (cond ((null prefix) 't)
        ((null other) nil)
        ('t (and
             (equal (car prefix) (car other))
             (is-prefix-p (cdr prefix) (cdr other))))))

(defun is-suffix-p (suffix other)
  (is-prefix-p (reverse suffix) (reverse other)))

(defun transpose (grid)
  (loop for i from 0 to (1- (length (car grid)))
        collect (loop for j from 0 to (1- (length grid))
                      collect (elt (elt grid j) i))))

(defun count-instances-in-row (needle haystack)
  "Counts PER ROW"
  (if (<= (length needle) (length haystack))
      (let ((rest (count-instances-in-row needle (cdr haystack))))
        (if (is-prefix-p needle haystack)
            (1+ rest)
            rest))
      0))
  
(defun count-pattern-horizontally (needle haystack)
  "Counts PER GRID"
  (reduce #'+ (mapcar (curry count-instances-in-row needle) haystack)))

(defun count-pattern-vertically (needle haystack)
  "Counts PER GRID"
  (reduce #'+ (mapcar (curry count-instances-in-row needle) (transpose haystack))))

(defun main-diag-of (m)
  (remove-if #'null
             (if (null m) nil
                 (cons (car (car m))
                       (main-diag-of (mapcar #'cdr (cdr m)))))))

(defun diags-of-go (m)
  (if (or (null m) (null (car m))) nil
      (cons
       (main-diag-of m)
       (concatenate 'list
                    (diags-of-go (cdr m))
                    (diags-of-go (mapcar #'cdr m))))))

(defun index-2d (coords grid)
  (let ((i (car coords))
        (j (cadr coords)))
    (nth j (nth i grid))))

(index-2d '(2 3)
          '((a b c d)
            (e f g h)
            (i j k l)
            (m n o p)))

(defun exists-already-as-suffix (elem set)
  (some
   (lambda (other)
     (and (is-suffix-p elem other)
          (not (equal elem other))))
   set))

(let ((m '((a b c d)
           (e f g h)
           (i j k l)
           (m n o p))))
  (loop for i from 0 to (1- (length m))
        collect (loop for j from 0 to (1- (length (car m)))
                 collect (list i j))))

(defun diags-of (m)
  (let* ((indices-dup (diags-of-go
                       (loop for i from 0 to (1- (length m))
                             collect (loop for j from 0 to (1- (length (car m)))
                                           collect (list i j)))))
         (indices (remove-if
                   (lambda (elem) (exists-already-as-suffix elem indices-dup))
                   indices-dup)))
    (mapcar (lambda (p) (index-2d p m)) indices)))


(diags-of '((a b c d)
            (e f g h)
            (i j k l)
            (m n o p)))

(diags-of (mapcar #'string-to-list (split-on-newline sample)))

;; i := 0, 1, 2
;; d := 0, 1, 2
;; (0, 1) -> (0, 1)
;; (0, 2) -> (0, 2)
;; (1, 1) -> (1, 0)
;; (1, 2) -> (1, 1)
;; (2, 0) -> (2, 2)
;; (2, 1) -> (2, 1)
;; (2, 2) -> (2, 0)

;; (0 0) (0 1) (0 2) (0 3) (0 4)
;; (1 0) (1 1) (1 2) (1 3) (1 4)
;; (2 0) (2 1) (2 2) (2 3) (2 4)

(concatenate 'list '(1 2 3) '(4 5 6))

(diag-indices 10 10)

(defun count-pattern-main-diag (needle haystack)
  (loop for i from 0 to (1- (length haystack))
        do (loop for j from 0 to (1- (length (car haystack)))
                 do ())))

(count-pattern-horizontally
 (string-to-list "MA")
 (mapcar #'string-to-list (split-on-newline sample)))

(defun part1 (input)
  (let ((parsed (mapcar #'string-to-list (split-on-newline input))))
    (+
     (count-pattern-horizontally          (string-to-list "XMAS")  parsed)
     (count-pattern-horizontally (reverse (string-to-list "XMAS")) parsed)
     (count-pattern-vertically            (string-to-list "XMAS")  parsed)
     (count-pattern-vertically   (reverse (string-to-list "XMAS")) parsed))))

(part1 sample)





