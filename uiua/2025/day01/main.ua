# Experimental!
~ "git: github.com/amatgil/steal-gift" ~ AoCGet

# ====== INPUTS ======
Input ← AoCGet 2025 1

$ L68
$ L30
$ R48
$ L5
$ R60
$ L55
$ L1
$ L99
$ R14
$ L82
Sample ←

Parse ← ˜⊏¯1_1⨂"LR"⊜(⊙⋕°⊂)⊸≠@\n
Part₁ ← /+=₀◿₁₀₀ \+⊂50 × Parse
Part₂ ← /+=₀◿₁₀₀ \+⊂50 ˜▽ Parse

⍤⤙≍ 3 Part₁ Sample
⍤⤙≍ 6 Part₂ Sample
