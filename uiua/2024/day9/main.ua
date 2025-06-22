# Experimental!
~ "git: github.com/amatgil/steal-gift" ~ AoCGet

# ====== INPUTS ======
Sample ← "2333133121414131402"
Input  ← AoCGet 2024 9

# ====== Shared ======

# ====== PART 1 ======
Part₁ ← (
  ≡⋕
  ˙⍜▽\+¬◿2°⊏                     # ids
  /◇⊂-₁⍚˜↯                       # compute starting layout
  ⍢(⍜⊏◌⊃(⊢⊚=₋₁|⊃↘₋₁⊣)|¬≍⊸\↧⇌=₋₁) # move to first free space
  /+×°⊏ ▽⊸≠₋₁                    # ignore empty, checksum
)

# ====== PART 2 ======
Part₂ ← 4◌

# ====== TESTS =======
# Input here
# ⍤⤙≍ 1928 Part₁ Sample
# ⍤⤙≍ "testout2" Part₂

# ======== RUNNING ========
# $"Part one solution is: '_'" Part₁ Input

# $"Part two solution is: '_'" Part₂ Input
