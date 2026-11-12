# Experimental!
~ "git: github.com/amatgil/steal-gift" ~ AoCGet
B ~ "git: github.com/Marcos-cat/uiua-bigint"

# ====== INPUTS ======
I ← AoCGet 2025 6

$ 123 328  51 64
$  45 64  387 23
$   6 98  215 314
$ *   +   *   +
Sample ←

Parse ← ≡⌞⬚@\s⊜∘ ⊸(¬/×=@\s) ⬚@\s⊜∘⊸≠@\n
P₁    ← /+ ⨬(/×|/+) ⊓≡(˜∊@+)(⍉⋕)⊃⊣↘₋₁ Parse
P₂    ← /+ ≡(⨬(/×|/+) ⊙(▽⊸≠¯1? ≡⍣⋕¯1⍉)) ⊓≡(˜∊@+)⤸₁ ⊃⊣↘₋₁ Parse
P₂ I
