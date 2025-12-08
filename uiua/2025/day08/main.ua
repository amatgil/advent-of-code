# Experimental!
~ "git: github.com/amatgil/steal-gift" ~ AoCGet

# ====== INPUTS ======
Input ← AoCGet 2025 8

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

Adv ← (
  ⊸≡⌞(/∨∊⊙°□)  # Where are they currently?
  ⊂□ /◇⊂ ∩⌟▽⟜¬ # Set union, bit scuffed
)

Parse ← ⊃(⊏⍏ ⊸≡/(⍜ⁿ₂/+-) ⧅₂<|≡(□¤)) ⊜(⊜⋕⊸≠@,)⊸≠@\n # Sort by distance, upper triangle 

Part₁ ← /×↙₃⇌⍆≡◇⧻ ∧Adv ↙⊙Parse
Part₂ ← /×⊢⍉ ⊣ ⍢(∪⌞∘⤚Adv °⊂|≠₁⧻◌) Parse

⍤⤙≍ 40 Part₁ 10 Sample
⍤⤙≍ 25272 Part₂ Sample

$Part₁ ⊙◌⍜nowPart₁1000 Input
$Part₂ ⊙◌⍜nowPart₂ Input
