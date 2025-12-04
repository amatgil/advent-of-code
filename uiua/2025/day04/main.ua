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

Accessibles ← ×⊸(<₄/+≡⌟⬚0↻⊂A₂C₂)
Part₁       ← /+♭ Accessibles =@@ ⊜∘⊸≠@\n
Part₂ ← (
  =@@ ⊜∘⊸≠@\n
  ⍥⊸(≠⊸Accessibles)∞
  ⌵/+⧈-≡(/+♭)
)
⍤⤙≍ 13 Part₁ Sample
⍤⤙≍ 43 Part₂ Sample
