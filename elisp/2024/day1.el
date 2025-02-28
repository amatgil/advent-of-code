(setq sample "3   4
4   3
2   5
1   3
3   9
3   3
") ; trailing newline!

(defun parse (rawinput)
  (cl-reduce (lambda (acc x)
                 (list
                  (cons (string-to-number (car x)) (car acc))
                  (cons (string-to-number (cadr x)) (cadr acc))))
             (mapcar
              (lambda (l) (let ((split (split-string l " ")))
                            (list (car split) (car (last split)))))

              (split-string (string-trim-right input "\n") "\n"))
             :initial-value '(() ())))

(defun part1 (input)
  (let ((parsed (parse input)))
    (-reduce '+
             (mapcar* (lambda (x y) (abs (- x y)))
                      (sort (car parsed) '<)
                      (sort (cadr parsed) '<)))))


(format "Part 1: %d" (part1 (f-read "../inputs/2024-01.txt")))
