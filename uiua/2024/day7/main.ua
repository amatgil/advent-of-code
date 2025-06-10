# Experimental!
~ "git: github.com/amatgil/steal-gift" ~ AoCGet

# ====== INPUTS ======
Input ← AoCGet 2024 7
$ 190: 10 19
$ 3267: 81 40 27
$ 83: 17 5
$ 156: 15 6
$ 7290: 6 8 6 15
$ 161011: 16 10 13
$ 192: 17 8 14
$ 21037: 9 7 18 13
$ 292: 11 6 16 20
$ 
Sample ←

# ====== Shared ======
ParseLine ← ⊓⋕(⊜⋕⊸≠@\s) °{⊙∘} ⊜□¬⊸⌕": "
E₁        ← |2 ⨬([∩⌟E₁] ⊃+× ⊙°⊂ :|⋅∘) =₀ ⊸⧻:           # ? Acc Rest
E₂        ← |2 ⨬([∩₃⌟E₂] ⊃(+|×|⍜∩°⋕⊂) ⊙°⊂ :|⋅∘) =₀ ⊸⧻: # ? Acc Rest
Solve! ← (
  ⊜□⊸≠@\n
  ˜⊙∘ ⍚ParseLine
  °□/+▽≥₁⊸≡◇(/+♭= ^0°⊂)
)

# ====== PART 1 ======
Part₁ ← Solve!E₁

# ====== PART 2 ======
Part₂ ← Solve!E₂

# ====== TESTS =======
⍤⤙≍ 3749 Part₁ Sample
⍤⤙≍ 11387 Part₂ Sample

# ======== RUNNING ========
$"Part one solution is: '_'" Part₁ Input

$"Part two solution is: '_'" Part₂ Input
