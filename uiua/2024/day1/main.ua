# Experimental!
~ "git: github.com/amatgil/steal-gift" ~ AoCGet

# ====== INPUTS ======
Input ← AoCGet 2024 1

# ====== Shared ======
Parse ← °⊟⍉ ⊜(⊜⋕⊸(≠@\s))⊸(≠@\n)
# ====== PART 1 ======
Part₁ ← /+⌵-∩⍆Parse

# ====== PART 2 ======
Part₂ ← /+× ⟜(/+⍉⊞=) ∩⍆ Parse

# ====== TESTS =======
$ 3   4
$ 4   3
$ 2   5
$ 1   3
$ 3   9
$ 3   3
⍤⤙≍ 11 Part₁ .
⍤⤙≍ 31 Part₂

# ======== RUNNING ========
$"Part one solution is: '_'" Part₁ Input

$"Part two solution is: '_'" Part₂ Input
