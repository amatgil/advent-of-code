# Experimental!
~ "git: github.com/amatgil/steal-gift" ~ AoCGet

# ====== INPUTS ======
Input ← AoCGet 2023 2

# ====== Shared ======

# ====== PART 1 ======
X      ← $ 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Amount ← map {"red" "green" "blue"} [12 13 14]

ParseCubes ← ⊜(⊜□⊸(≠@\s))⊸(¬⦷", ")
ParseRHS   ← ⊜(□ParseCubes) ⊸(¬⦷"; ")
IsSafe     ←
ParseRHS "3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"
°□⊢
≡(°⊟)

# (≡(⊓(get ⊙Amount|⋕) :°⊟)°□)
# ◇(?) °(□¤)
# F ← ⊜(IsSafe ParseGame) ⊸(¬⦷"; ")
# F X

# "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"
# ⊜□ ⊸(≠@:)
# ⍜⊢(▽⊸(↧⊃(≥@0|≤@9))°□)

Part₁ ← ∘

# ====== PART 2 ======
Part₂ ← ∘

# ====== TESTS =======
$ Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
$ Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
$ Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
$ Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
$ Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
⍤⤙≍ 8 Part₁

# ======== RUNNING ========
# $"Part one solution is: '_'" Part₁ Input

# $"Part two solution is: '_'" Part₂ Input
