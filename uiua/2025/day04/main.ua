# Experimental!
~ "git: github.com/amatgil/steal-gift" ~ AoCGet

# ====== INPUTS ======
Input ← AoCGet 2025 4

$ ..@@.@@@@.
$ @@@.@.@.@@
$ @@@@@.@.@@
$ @.@@@@..@.
$ @@.@@@@.@@
$ .@@@@@@@.@
$ .@.@.@.@@@
$ @.@@@.@@@@
$ .@@@@@@@@.
$ @.@.@@@.@.
Sample ←

Parse       ← =@@ ⊜∘⊸≠@\n
Accessibles ← ↧⊸(<₄/+≡⌟⬚0↻⊂A₂C₂)
Part₁       ← /+♭ Accessibles Parse
Part₂       ← -⊃⊣⊢≡(/+♭) ⍥⊸(≠⊸Accessibles)∞ Parse
⍤⤙≍ 13 Part₁ Sample
⍤⤙≍ 43 Part₂ Sample
