# Experimental!
~ "git: github.com/amatgil/steal-gift" ~ AoCGet

# ====== INPUTS ======
Sample ← "2333133121414131402"
Input  ← AoCGet 2024 9

# ====== Shared ======

# ====== PART 1 ======
Part₁ ← (
  ≡⋕                              # parse lmao
  ˙⍜▽\+¬◿2°⊏                      # ids
  /◇⊂-₁⍚˜↯                        # compute starting layout
  ⍢(⍜⊏◌ ⊃(⊢⊚=₋₁|⊃↘₋₁⊣)|¬≍⊸\↧⇌=₋₁) # move to first free space
  /+×°⊏ ▽⊸≠₋₁                     # ignore empty, checksum
)

# ====== PART 2 ======
Part₂ ← 4◌

# ====== TESTS =======
# Input here
⍤⤙≍ 1928 Part₁ Sample
# ⍤⤙≍ 2858 Part₂ Sample

# ======== RUNNING ========
# $"Part one solution is: '_'" Part₁ Input

# $"Part two solution is: '_'" Part₂ Input

# ? state currId
MoveFile ← (
  ⍜˙⊜(?₃)⊙: ⊸=₋₁∩⌟▽⊸¬◡=

  # ⊙(˙⊜⧻⊸≠₋₁?)

  # ⊃(˜▽°▽=₋₁◌|⊓⊣◌°▽◡=) #
  # ⊃(⊙∘|⋅⋅=₋₁) ⊢⊚≤
  # +⇡°⊟⊏[⟜+₁] ⊙(˜▽?°▽?:)
  # ⍣(⟜⊃(=₋₁◌◌|▽⤚⊸≠◌)?⊢)∘⊚
  # repeat
  # ⊃(/+=|▽⤚⊸≠)
  # ⍜⊏◌+⇡

  # -₁⍥⟜(⍜⊏◌+⇡⊃(/+=|⊢⊚=₋₁◌|▽⤚⊸≠))
)

# PART 2 IS UNFINISHED
# I lost the plot a bit

F ← (
  ⊚≤◡⊃(▽=₋₁◌|⊡⊚=)
  ⍣(⊢
    ⊃(⊙∘|▽⊸≠◌)
    ⊃(⋅∘|⍜▽⍜˜⊡◌⊸=₋₁:⊙:)
  | ◌
  )
  -₁
)

# ? State CurrIndex
F ← (
  ◡(°□˜⊡)
  ⊚≥◡⊃(⧻|⋅(≡◇/+=₋₁))
  ⊙+₁⍣(
    ⊣
    ⍩(
      ⍜(⊙°□𝄐⌞⊡ # get file
      | ⍜(▽|⍜⇌(⍜↙◌◡⋅⧻))⊸=₋₁:)
      ⊸˜⍜⊏⋅{[]} # "Remove" old one
    )
  | ◌◌)
)

≡⋕ Sample
-₁˙⍜▽\+¬◿2°⊏ # ids
⇌⍚˜↯
⊙0
# ⊙◌⍥F?⊸⧻
# ⇌/◇⊂ # 00992111777.44.333....5555.6666.....8888..
# another idea: instead of finding the index, then slotting it in,
# fold over the memory until you find a hole
