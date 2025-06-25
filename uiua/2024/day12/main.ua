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

$ EEEEE
$ EXXXX
$ EEEEE
$ EXXXX
$ EEEEE
Sample₃ ←

# ====== Shared ======

# ====== PART 1 ======

# this one is like 3x faster, but is a bit uglier
# the other one is more aesthetic
PrimePart₁ ← (
  CountNeighs ← |1 (
    ⊓♭₋₁¤°⊡
    ≡(/+♭ ⊸=⊃(≡⬚∞⊡ ⊓(⧋+A₂|¤))⊡) # counts neighs equal to myself
    ˜⊕/+                        # sum of counts per id
  )
  ⊜∘⊸≠@\n                   # pad
  ˙⍜⊜□(⍚˜(↯⧻)°⊏) +₁-@A      # assign id by region
  ⟜(˜-×₄)⊃(˙⊕⧻|CountNeighs) # compute regions from inners
  /+×                       # math it up
)

Part₁ ← (
  ⊜∘⊸≠@\n                 # pad
  ˙⍜⊜□(⍚˜(↯⧻)°⊏) +₁-@A    # assign id by region
  ⊃(˙⊕⧻                   # Count area
  | =⊃(◴♭)¤               # separate into list of masks by id
    ≡₂(+∩(/+♭⬚0⧈(≠0-))⊸⍉) # compute regions=deltas
  )
  /+×
)
# ====== PART 2 ======
# Part₂ ← 4◌

# ====== TESTS =======
⍤⤙≍ 772 Part₁ Sample₁
⍤⤙≍ 1930 Part₁ Sample₂
⍤⤙≍ 772 PrimePart₁ Sample₁ # the fast one
⍤⤙≍ 1930 PrimePart₁ Sample₂

# ======== RUNNING ========
# $"Part one solution is: '_'" Part₁ Input

# $"Part two solution is: '_'" Part₂ Input

Sample₁
⊜∘⊸≠@\n              # pad
˙⍜⊜□(⍚˜(↯⧻)°⊏) +₁-@A # assign id by region
=⊃(◴♭)¤
≡₂(+∩(/+♭⬚0⧈(≠0-))⊸⍉)
