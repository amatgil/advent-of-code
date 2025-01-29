# Experimental!
~ "../../fetcher/lib.ua" ~ AoCGet

# ====== INPUTS ======
Input ← AoCGet 2023 1

# ====== PART 1 ======
Part₁ ← /+≡(+×10°(□⊟))⍚(⊂⊃⊢(⊢⇌) ▽⊸(>0)∵⍣(⋕|0))⊜□⊸(≠@\n)

# ====== PART 2 ======
Numbers ← {
  "one" "two" "three"
  "four" "five" "six" "seven"
  "eight" "nine" "1" "2" "3"
  "4" "5" "6" "7" "8" "9"}
Vals   ← ⊂.+1⇡9
Decode ← map Numbers Vals

Lettered ← (
  /◇⊂⊸⍚(⊞=+1⇡⊸/↥ ⦷°□) Numbers ¤
  ▽⊸≡/↥
  ⊟⊃⊣⊢⍆
  ◌/↥⟜≡(□▽)
)

Part₂ ← (
  °□/+⍚(+×10°⊟∵(˜get Decode) Lettered) ⊜□⊸(≠@\n)
)

# ====== TESTS =======
"1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet"
⍤⤙≍ 142 Part₁
"two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen"
⍤⤙≍ 281 Part₂

# ======== RUNNING ========
$"Part one solution is: '_'" Part₁ Input
$"Part two solution is: '_'" Part₂ Input
