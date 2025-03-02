;(load "./utils.lisp")

(defparameter sample "MMMSXXMASM
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
