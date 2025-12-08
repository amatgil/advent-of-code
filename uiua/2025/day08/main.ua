# Experimental!
~ "git: github.com/amatgil/steal-gift" ~ AoCGet

# ====== INPUTS ======
Input ← AoCGet 2025 8

D ← ⍜ⁿ₂/+-

$ 162,817,812
$ 57,618,57
$ 906,360,560
$ 592,479,940
$ 352,342,300
$ 466,668,158
$ 542,29,236
$ 431,825,988
$ 739,650,466
$ 52,470,668
$ 216,146,977
$ 819,987,18
$ 117,168,530
$ 805,96,715
$ 346,949,466
$ 970,615,88
$ 941,993,340
$ 862,61,35
$ 984,92,344
$ 425,690,689
Sample ←

Part₁ ← (
  ⊜(⊜⋕⊸≠@,)⊸≠@\n
  ⊃(⊏⍏∩⌞▽⊸≠₀ ⊸≡/(⍜ⁿ₂/+-)♭₋₁˙⊞⊟|≡(□¤)) # Sort by distance ignoring self ones
  ↙⤙⊙⊙◌ ◴ ≡⍆                          # Distance is symmetrical, fish amount
  ∧(
    ⊸≡⌞(/∨∊⊙°□)  # Where are they currently?
    ⊂□ /◇⊂ ∩⌟▽⟜¬ # Set union
  )
  /× ↙₃⇌⍆≡◇⧻
)

⍤⤙≍ 40 Part₁ Sample 10
# ◌⍢(∪⌞∘Adv °⊂|±⧻)
# ◌⍥(∪⌞∘Adv °⊂)∞

# /× ↙₃⇌⍆≡◇⧻

# # Sets ? Pair Sets
#  Adv ← |2 (
#  ⊸≡⌞◇(°¤?∊?)
#  ∩⌟▽⟜¬
#  ⊂□≡(°¤°□)
#  ?
# )

# ⊢₂
# ⊃(⋅⊙|Adv⊙⋅⊙)
# ⊃(⋅⊙|Adv ⊙⋅⊙) ⊢₂
# Adv

# ⊢
# Adv
# ⊸≡⌞◇(°¤?∊?)
# ∧Adv

# ⊸≡⌞(/∨≡(/∨⌕)) # Find elems of pair in array of sets
# ∩△            # ∩⌟▽⟜¬

# pop
# ⊸(♭₋₁˙⊞{◡D}
#  ▽⊸(≠₀≡°□⊢⍉) # Filter self-distances
#  ⊏⍏⊸(≡°□⊢⍉)  # Sort by distance
#  ↙10
#  ≡(≡°□↘₁)
# )

# ⊙(⊸≡⋅∞)
# ⊢ # fold placeholder
# ⊃(<₂⨂⊙⋅⊙|⋅⊙∘)

# ∩₃△
