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

;;;;;;;;;;;;;;;;;;;; PART 1 ;;;;;;;;;;;;;;;;;;;;

(defun is-prefix-p (prefix other)
  (cond ((null prefix) 't)
        ((null other) nil)
        ('t (and
             (equal (car prefix) (car other))
             (is-prefix-p (cdr prefix) (cdr other))))))

(defun count-instances-in-row (needle haystack)
  "Counts PER ROW"
  (if (> (length needle) (length haystack)) 0
      (let ((rest (count-instances-in-row needle (cdr haystack))))
        (if (is-prefix-p needle haystack)
            (1+ rest)
            rest))))
  
(defun main-diag-of (m)
  (remove-if #'null ; Dumb way of removing trailing nil
             (if (null m) nil
                 (cons (car (car m))
                       (main-diag-of (mapcar #'cdr (cdr m)))))))

(defun diags-of-go-removing-lines (m)
  (if (null m) nil
      (cons (main-diag-of m)
            (diags-of-go-removing-lines (cdr m)))))

(defun diags-of-go-removing-cols (m)
  (if (null (car m)) nil
      (cons (main-diag-of m)
            (diags-of-go-removing-cols (mapcar #'cdr m)))))

(defun diags-of (m)
  (if (or (null m) (null (car m))) nil
      (cons
       (main-diag-of m)
       (concatenate 'list
                    (diags-of-go-removing-lines (cdr m))
                    (diags-of-go-removing-cols (mapcar #'cdr m))))))

(defun count-pattern-horiz (needle haystack)
  (reduce #'+ (mapcar (curry count-instances-in-row needle) haystack)))

(defun count-pattern-vert (needle haystack)
  (reduce #'+ (mapcar (curry count-instances-in-row needle) (transpose haystack))))

(defun count-pattern-major-diag (needle haystack)
  (count-pattern-horiz needle (diags-of haystack)))

(defun count-pattern-minor-diag (needle haystack)
  (count-pattern-horiz needle (diags-of (mapcar #'reverse haystack))))

(defun part1 (input)
  (let* ((parsed (mapcar #'string-to-list (split-on-newline input)))
         (needle-normal (string-to-list "XMAS"))
         (needle-reversed (reverse needle-normal)))
    (+
     (count-pattern-horiz        needle-normal   parsed)
     (count-pattern-vert         needle-normal   parsed)
     (count-pattern-major-diag   needle-normal   parsed)
     (count-pattern-minor-diag   needle-normal   parsed)
     (count-pattern-horiz        needle-reversed parsed)
     (count-pattern-vert         needle-reversed parsed)
     (count-pattern-major-diag   needle-reversed parsed)
     (count-pattern-minor-diag   needle-reversed parsed))))

;;;;;;;;;;;;;;;;;;;; PART 2 ;;;;;;;;;;;;;;;;;;;;

(defun there-is-cross-here (needle haystack)
  "Expects lists"
  (if (or (> (length needle) (length haystack))
          (> (length needle) (length (car haystack))))
      nil
      (let* ((n (length needle))
             (submat (mapcar (curry take n) (take n haystack)))
             (maj-diag (main-diag-of submat))
             (min-diag (main-diag-of (mapcar #'reverse submat))))
        (and (or (is-prefix-p needle maj-diag)
                 (is-prefix-p (reverse needle) maj-diag))
             (or (is-prefix-p needle min-diag)
                 (is-prefix-p (reverse needle) min-diag))))))

(defun count-crosses-moving-across (needle haystack)
  "Moves the sliding window exclusively rightward"
  (if (or (null haystack) (null (car haystack))) 0
      (+ (if (there-is-cross-here needle haystack) 1 0)
         (count-crosses-moving-across needle (mapcar #'cdr haystack)))))
  
(defun count-crosses (needle haystack)
  (if (or (null haystack) (null (car haystack))) 0
      (+
       (count-crosses-moving-across needle haystack)
       (count-crosses needle (cdr haystack)))))

(defun part2 (input)
  (let* ((parsed (mapcar #'string-to-list (split-on-newline input)))
         (needle (string-to-list "MAS")))
    (count-crosses needle parsed)))

(assert (equal 18 (part1 sample)))
(assert (equal 9 (part2 sample)))

(format t "~&Part 1: ~A~&" (part1 (uiop:read-file-string "../inputs/2024-04.txt")))
(format t "~&Part 2: ~A~&" (part2 (uiop:read-file-string "../inputs/2024-04.txt")))
