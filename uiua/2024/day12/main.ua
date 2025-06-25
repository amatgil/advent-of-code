# Experimental!
~ "git: github.com/amatgil/steal-gift" ~ AoCGet

# ====== INPUTS ======

Input ← AoCGet 2024 12
$ OOOOO
$ OXOXO
$ OOOOO
$ OXOXO
$ OOOOO
Sample₁ ←
$ RRRRIICCFF
$ RRRRIICCCF
$ VVRRRCCFFF
$ VVRCCCJFFF
$ VVVVCJJCFE
$ VVIVCCJJEE
$ VVIIICJJEE
$ MIIIIIJJEE
$ MIIISIJEEE
$ MMMISSJEEE
Sample₂ ←

# ====== Shared ======

# ====== PART 1 ======
CountNeighs ← |1 (
  ⊓♭₋₁¤°⊡
  ≡(                          # foreach coord
    /+♭ ⊸=⊃(≡⬚∞⊡ ⊓(⧋+A₂|¤)|⊡) # counts neighs equal to myself
  )
  ˜⊕/+ # sum of counts
)

Part₁ ← (
  ⊜∘⊸≠@\n                   # pad
  ˙⍜⊜□(⍚˜(↯⧻)°⊏) +₁-@A      # assign id by region
  ⟜(˜-×₄)⊃(˙⊕⧻|CountNeighs) # compute regions from inners
  /+×                       # math it up
)
# ====== PART 2 ======
# Part₂ ← 4◌

# ====== TESTS =======
⍤⤙≍ 772 Part₁ Sample₁
⍤⤙≍ 1930 Part₁ Sample₂

# ======== RUNNING ========
$"Part one solution is: '_'" Part₁ Input

# $"Part two solution is: '_'" Part₂ Input
