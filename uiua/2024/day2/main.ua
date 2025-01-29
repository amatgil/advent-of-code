# Experimental!
~ "git: github.com/amatgil/steal-gift" ~ AoCGet

# ====== INPUTS ======
Input ← AoCGet 2024 2

# ====== Shared ======
IsValid₁   ← |1 ↧⊃(/×↧⊃≥₁≤₃ ⌵/- ⍉ ⧈∘2|↥⊃(≍⇌|≍)⊃(⍖⍏|⇡⧻))
VariantsOf ← ≡⍜⊏⋅[] ⊃(⇡⧻|¤)

IsValid₂ ← |1 /↥ ≡IsValid₁ VariantsOf

# ====== PART 1 ======
Part₁ ← |1 /+ ⊜(IsValid₁ ⊜⋕ ⊸(≠@\s)) ⊸(≠@\n)

# ====== PART 2 ======
Part₂ ← |1 /+ ⊜(IsValid₂ ⊜⋕ ⊸(≠@\s)) ⊸(≠@\n)

# ====== TESTS =======
$ 7 6 4 2 1
$ 1 2 7 8 9
$ 9 7 6 2 1
$ 1 3 2 4 5
$ 8 6 4 4 1
$ 1 3 6 7 9
⍤⤙≍ 2 Part₁ .
⍤⤙≍ 4 Part₂

# ======== RUNNING ========
$"Part one solution is: '_'" Part₁ Input

$"Part two solution is: '_'" Part₂ Input
