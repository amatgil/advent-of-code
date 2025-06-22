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

# ? AntennaMask
AntiNodesOf₁ ← ⍥₂/◇⊂˙⊞(□˜(▽¬⊸∊)⊸(⊂⊃+-) ¤⊸/-⊟)⊚
# ? GridShape Positions
KeepValid ← (
  ▽≡/×⊸⧋< # Too large
  ▽≡/×⊸≥₀ # Too small
)
# ====== PART 1 ======
Part₁ ← (
  ⊜∘⊸≠@\n     # parse
  ⊸⊞=▽⊸≠@.◴⊸♭ # map of antennas per letter
  ⧻◴KeepValid ⊃(⋅△|/◇⊂⍚AntiNodesOf₁)
)

# ====== PART 2 ======
Part₂ ← 4◌

# ====== TESTS =======
⍤⤙≍ 14 Part₁ Sample
# ⍤⤙≍ 34 Part__2 Sample

# ======== RUNNING ========
# $"Part one solution is: '_'" Part₁ Input

# $"Part two solution is: '_'" Part₂ Input

# Stack: diff acc shape
TilInvalid! ← ⋅⊙◌⍢(⊙⊂⤙˜^0◡⋅⊢|/×♭⊂⊃>>₀⊢◌) ⊙¤

# Stack: diff position shape
IterateBoth ← ⨬(⊂⊃(TilInvalid!+|TilInvalid!-)|[]) ⊸(≍0_0)
AntiNodesOf₂ ← (
  ⍚(
    ⊓⊚⊸(¤△)
    ⊞(□⊂⊃(IterateBoth⊙⊙⋅⊙|IterateBoth⊙⋅⊙⊙)◡-).
    ⊙◌
  )
  ⍚♭
  ⍥₂/◇⊂
)

Input
⊜∘⊸≠@\n

⊃(⊞=▽⊸≠@.◴⊸♭|¤) # map of antennas per letter
⊸AntiNodesOf₂
◡⋅(△°¤)
⧻◴ KeepValid
⊙◌

# ? diff acc shape
