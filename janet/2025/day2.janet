(use judge)

(def test-input "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124")
(def real-input (slurp "day2.txt"))

# === Solution ===
(defn is-invalid1 (id)
  (if (< (length id) 2) false
      (compare= (string/repeat (take (math/floor (/ (length id) 2)) id) 2)
                id)))

(defn is-invalid2 (id)
  (prompt :a
     (let [l (length id)]
       (loop [d :in (range 1 l) :when (= 0 (mod l d))]
         (when (compare= (string/repeat (take d id) (/ l d))
                         id)
           (return :a true))))
     false))

(defmacro run (input invalid-check)
  (with-syms [$l $u]
    ~(sum
      (flatten
       (seq [[$l $u] :in (peg/match '(split "," (group (* (number :d+) "-" (number :d+)))) ,input)]
            (filter |(,invalid-check (string/format "%i" $)) (range $l (+ 1 $u))))))))

# === Running ===
(test (run test-input is-invalid1) 1227775554)
(test (run real-input is-invalid1) 29818212493)

(test (run test-input is-invalid2) 4174379265)
(test (run real-input is-invalid2) 37432260594)



# === Tests ===
(test (seq [[l u] :in (peg/match '(split "," (group (* (number :d+) "-" (number :d+)))) test-input)]
           (filter |(is-invalid1 (string/format "%i" $)) (range l (+ 1 u))))
  @[@[11 22]
    @[99]
    @[1010]
    @[1188511885]
    @[222222]
    @[]
    @[446446]
    @[38593859]
    @[]
    @[]
    @[]])

(test (seq [[l u] :in (peg/match '(split "," (group (* (number :d+) "-" (number :d+)))) test-input)]
           (filter |(is-invalid2 (string/format "%i" $)) (range l (+ 1 u))))
  @[@[11 22]
    @[99 111]
    @[999 1010]
    @[1188511885]
    @[222222]
    @[]
    @[446446]
    @[38593859]
    @[565656]
    @[824824824]
    @[2121212121]])

