# Experimental!
~ "git: github.com/amatgil/steal-gift" ~ AoCGet

# ====== INPUTS ======
Input ← AoCGet 2024 8

# ====== Shared ======
$ ............
$ ........0...
$ .....0......
$ .......0....
$ ....0.......
$ ......A.....
$ ............
$ ............
$ ........A...
$ .........A..
$ ............
$ ............
$ 
Sample ←

# ? GridShape Positions
KeepValid ← (
  ▽≡/×⊸⧋< # Too large
  ▽≡/×⊸≥₀ # Too small
)
# ====== PART 1 ======
# ? AntennaMask
AntiNodesOf₁ ← ⍥₂/◇⊂˙⊞(□˜(▽¬⊸∊)⊸(⊂⊃+-) ¤⊸/-⊟)⊚
Part₁ ← (
  ⊜∘⊸≠@\n     # parse
  ⊸⊞=▽⊸≠@.◴⊸♭ # map of antennas per letter
  ⧻◴KeepValid ⊃(⋅△|/◇⊂⍚AntiNodesOf₁)
)

# ====== PART 2 ======
TilInvalid! ← ⋅⊙◌⍢(⊙⊂⤙˜^0◡⋅⊢|/×♭⊂⊃>>₀⊢◌) ⊙¤               # diff acc shape
IterBoth    ← ⨬(⊂⊃(TilInvalid!+|TilInvalid!-)|[]) ⊸(≍0_0) # diff position shape
# ? AntennaMask GridShape
AntiNodesOf₂ ← (
  ⊙¤
  ⍚(⊙◌⊞(□⊂⊃(IterBoth⊙⊙⋅⊙|IterBoth⊙⋅⊙⊙)◡-). ⊓⊚⊸(¤△))
  ⍥₂/◇⊂ ⍚♭
)

Part₂ ← (
  ⊜∘⊸≠@\n
  ⊸(⊞=▽⊸≠@.◴⊸♭)
  ⊸AntiNodesOf₂
  ⊙◌ ⧻◴ KeepValid ◡⋅△
)

# ====== TESTS =======
⍤⤙≍ 14 Part₁ Sample
⍤⤙≍ 34 Part₂ Sample

# ======== RUNNING ========
$"Part one solution is: '_'" Part₁ Input
$"Part two solution is: '_'" Part₂ Input
