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
# Part₁ ← 3◌

# ====== PART 2 ======
# Part₂ ← 4◌

# ====== TESTS =======
# Input here
# ⍤⤙≍ "testout1" Part₁
# ⍤⤙≍ "testout2" Part₂

# ======== RUNNING ========
# $"Part one solution is: '_'" Part₁ Input

# $"Part two solution is: '_'" Part₂ Input

# ? Pos Grid
Neighs ← ⨬(⋅⋅[]|▽⤚(=𝄐⌞⊡)⊸⊃(⧋+A₂|⊡)) ≠@\s◡⊡

CountNeighs ← |1 (
  ⊓♭₋₁¤°⊡
  ≡(
    /+♭ ⊸=⊃(≡⬚∞⊡ ⊓(⧋+A₂|¤)|⊡)
  )
  ˜⊕/+
)

Sample₂
⊜∘⊸≠@\n           # pad
˙⍜⊜□(⍚˜(↯⧻)°⊏)-@A # assign id by region
⟜(˜-×₄)⊃(˙⊕⧻|CountNeighs)

# ⊸(↘¯⟜↘1_1⬚∞⧈(/+♭?=?⊸⊡1_1)3_3) # counts A__3 instead of A__2 :(

# idea for regions: mask for each capital letter, add up sections

# ≡(/+⤚=⊃(⊡|≡⍣⊡@Ç ⊓(⧋+A₂)¤))⊓♭₋₁¤°⊡ ⬚@\s⌝↘¯⟜⌝↘1_1

# pad with spaces
# ⊓♭₋₁¤°⊡
# ⍚path(Neighs|0)
# ⍜(⊕□|⟜⍚⧻) ⊸-@A # <- areas

# goal: assign different numbers to each region
