# Experimental!
~ "git: github.com/amatgil/steal-gift" ~ AoCGet

# ====== INPUTS ======
Input ← AoCGet 2024 4

# ====== Shared ======

# ====== PART 1 ======
LowerTriangle ← ⊞≥ . (⇡⧻)
UpperTriangle ← ⊞≤ . (⇡⧻)
DiagsOf       ← ⌅(≡(⌝⤸0_0↻¯)⊃(⇡⧻|¤))
FindWord      ← +⊃(⌕"XMAS"|⌕⇌"XMAS")
FilterBy      ← ∵⨬(@.|∘)

Part₁ ← (
  ⊜∘⊸≠@\n
  ⊃(⍜(DiagsOf|FindWord FilterBy ⊸UpperTriangle)
  | ⍜(DiagsOf|FindWord FilterBy ⊸LowerTriangle)
  | ⍜(DiagsOf≡⇌|FindWord FilterBy ⊸UpperTriangle)
  | ⍜(DiagsOf≡⇌|FindWord FilterBy ⊸LowerTriangle)
  | FindWord
  | ⍜⍉FindWord
  )
  +++++
  /+/+
)

# ====== PART 2 ======
Part₂ ← 4◌

# ====== TESTS =======
$ MMMSXXMASM
$ MSAMXMSMSA
$ AMXSXMAAMM
$ MSAMASMSMX
$ XMASAMXAMM
$ XXAMMXXAMA
$ SMSMSASXSS
$ SAXAMASAAA
$ MAMMMXMMMM
$ MXMXAXMASX
⍤⤙≍ 18 Part₁
# ⍤⤙≍ "testout2" Part₂

# ======== RUNNING ========
$"Part one solution is: '_'" Part₁ Input

# $"Part two solution is: '_'" Part₂ Input
