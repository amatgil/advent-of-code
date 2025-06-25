# Experimental!
~ "git: github.com/amatgil/steal-gift" ~ AoCGet

# ====== INPUTS ======
Input ← AoCGet 2024 13
$ Button A: X+94, Y+34
$ Button B: X+22, Y+67
$ Prize: X=8400, Y=5400
$ 
$ Button A: X+26, Y+66
$ Button B: X+67, Y+21
$ Prize: X=12748, Y=12176
$ 
$ Button A: X+17, Y+86
$ Button B: X+84, Y+37
$ Prize: X=7870, Y=6450
$ 
$ Button A: X+69, Y+23
$ Button B: X+27, Y+71
$ Prize: X=18641, Y=10279
Sample₁ ←

$ Button A: X+94, Y+34
$ Button B: X+22, Y+67
$ Prize: X=10000000008400, Y=10000000005400
$ 
$ Button A: X+26, Y+66
$ Button B: X+67, Y+21
$ Prize: X=10000000012748, Y=10000000012176
$ 
$ Button A: X+17, Y+86
$ Button B: X+84, Y+37
$ Prize: X=10000000007870, Y=10000000006450
$ 
$ Button A: X+69, Y+23
$ Button B: X+27, Y+71
$ Prize: X=10000000018641, Y=10000000010279
Sample₂ ←

# ====== Shared ======
Parse ← (
  ⊜(⊜□⊸≠@\n)¬⊸⌕"\n\n"
  ≡(
    °{⊙⊙∘}
    ⊓(°$"Button A: X_, Y_"|°$"Button B: X_, Y_"|°$"Prize: X=_, Y=_")
    ∩₃(⊟∩⋕)
  )
)

Sol! ← (
  ^0Parse
  ≡(⊟∩⌟˜÷ ∩₃(/-/×⍜⊢⇌) ⤚⊃(⍜⍉⍜⊢◌|⍜⍉⍜⊣◌) ⍉⊟) # cramer it up
  ⍉▽⊸(≡/×=⊸⁅)
  /+♭×3_1
)

# ====== PARTS ======
Part₁ ← Sol!∘
Part₂ ← Sol!⊙⊙(+10000000000000)

# ====== TESTS =======
⍤⤙≍ 480 Part₁ Sample₁

# ======== RUNNING ========
$"Part one solution is: '_'" Part₁ Input
$"Part two solution is: '_'" Part₂ Input
