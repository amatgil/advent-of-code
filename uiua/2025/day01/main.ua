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

# fold is being really messy with this
# it feels like it misses the final item
# this looks more scuffed than it should

Parse ← ×˜⊏¯1_1⨂"LR"⊜(⊙⋕°⊂)⊸≠@\n
Part₁ ← /+=₀◿₁₀₀ ◌∧(⟜˜⊂+) ⊙(50[]) Parse

# Acc zerop ? Acc Delta
Add ← ⊃+(∊0◿₁₀₀⍜-⇡)

Parse Input
# ∧⊸(Add?) ⊙50

?
/+=₀⋅◌∧(⟜⊃(⊂⊙⋅⊙|⊂⋅⊙⋅⊙)Add) ⊙(0 [] [])⊂50

# N C ns cs

# $Pre \+⊂50
# ⊸(⌵⧈-⌊÷₁₀₀)
# $Post # ×⬚1↻₋₁$Zero⊸(≠₀)
## "Maximum execution time exceeded" 
# $Expe [1 0 1 0 1 1 0 1 0 1]
# ∩(⊂∞)
# ⇌⊟₃
