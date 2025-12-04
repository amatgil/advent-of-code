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

Parse ← =@@ ⊜∘⊸≠@\n
Ac    ← ↧⊸(<₄/+⬚0↻⊂A₂C₂) # Accessibles
Part₁ ← /+♭ Ac Parse
Part₂ ← -∩(/+♭)⊃⊣⊢ ⍥⊸(≠⊸Ac)∞ Parse
⍤⤙≍ 13 Part₁ Sample
⍤⤙≍ 43 Part₂ Sample
