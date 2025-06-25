# Experimental!
~ "git: github.com/amatgil/steal-gift" ~ AoCGet

# ====== INPUTS ======
Input  ← AoCGet 2024 11
Sample ← "125 17"

# ====== Shared ======
N ← memo(⍣([1]°0|⍜°⋕([⊃↙↘]÷2⊸(°0◿2)⊸⧻)|[×2024]))

# ? n Input
# Sol ← ⧻⍥(⊙(&p-Start now?₁+₁)/◇⊂⍚N) ⊙⊙0 ⊙(⊜⋕⊸≠@\s)
# Sol ← ⧻⍥(/◇⊂⍚N) ⊙(⊜⋕⊸≠@\s)
# ====== PARTS ======
# Part₁ ← Sol 25
# Part₂ ← Sol 75

# ====== TESTS =======
# ⍤⤙≍ 55312 Part₁ Sample

# ======== RUNNING ========
# &p$"Part one solution is: '_'" Part₁ Input
# 
# &p$"Part two solution is: '_'" Part₂ Input

# ? t x
L ← |2 ⨬(⋅1|⍚L-₁⊙N)⊸≠₀
Input
⊜⋕⊸≠@\s
/+≡(⧻⍥/◇⊂∞ L) 75
