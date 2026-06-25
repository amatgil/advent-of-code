(use judge)
(use util.janet)


(def test-input ```
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
```)
(def real-input (slurp "day1.txt"))

(def peg ~{
           :main (* (split "\n" :line))
           :line (+ (if "L" (* 1 (/ (number :d+) ,-)))
                    (if "R" (* 1 (number :d+)))
                    -1) # trailing newline, empty group at EOF
           })
(defn solve (deltas)
  (length (filter |(= 0 $) (accumulate |(mod (+ $0 $1) 100) 50 deltas))))

(defn part1 (input)
  (solve (peg/match peg input)))

(defn part2 (input)
  (def ones @[])
  (loop [delta :in (peg/match peg input)]
    (loop [:repeat (math/abs delta)] (array/push ones (sign delta))))
  (solve (flatten ones)))


(test (part1 test-input) 3)
(test (part1 real-input) 1135)

(test (part2 test-input) 6)
(test (part2 real-input) 6558)

