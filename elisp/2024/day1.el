(load-file "utils.el")

(setq sample "3   4
4   3
2   5
1   3
3   9
3   3
")

(defun part1 (input)
  (let ((parsed (cl-reduce (lambda (acc x)
                             (let ((new-left  (car x))
                                   (new-right (cadr x))
                                   (old-left  (car acc))
                                   (old-right (cadr acc)))
                               (list
                                (cons (string-to-number new-left) old-left)
                                (cons (string-to-number new-right) old-right))))
                           (mapcar
                            (lambda (l) (let ((split (split-string l " ")))
                                          (list (car split) (cadddr split))))

                            (split-string (string-trim-right input "\n") "\n"))
                           :initial-value '(() ()))))
    (-reduce '+
             (mapcar* (lambda (x y) (abs (- x y)))
                      (sort (car parsed) '<)
                      (sort (cadr parsed) '<)))))


(format "Part 1: %d" (part1 (f-read "../inputs/2024-01.txt")))
