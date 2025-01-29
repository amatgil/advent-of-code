# Experimental!
~ "git: github.com/amatgil/steal-gift" ~ AoCGet

# ====== INPUTS ======
Input ← AoCGet 2024 3

# ====== Shared ======

# ====== PART 1 ======
Part₁ ← /+/×⋕↘₁⍉ regex "mul\\((\\d+),(\\d+)\\)"

# ====== PART 2 ======
IsConditional ← ⍜¤(/+⊞≍ {"do()" "don't()"})□
ComputeMul    ← ×∩⋕°$"mul(_,_)"
HandleMul     ← ⨬(◌|⊃(⋅∘|⊂⊙⋅∘)ComputeMul),
HandleCond    ← ≍"do()" ⊓°□◌

Part₂ ← (
  ⬚@ regex $ (mul\((\d+),(\d+)\)|do\(\)|don't\(\))
  ◇⊢⍉
  ∧(⨬(HandleMul|HandleCond)⊸IsConditional°□) ⊙(1 [])
  /+◌
)

# ====== TESTS =======
"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"
⍤⤙≍ 161 Part₁
"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"
⍤⤙≍ 48 Part₂

# ======== RUNNING ========
$"Part one solution is: '_'" Part₁ Input
$"Part two solution is: '_'" Part₂ Input
