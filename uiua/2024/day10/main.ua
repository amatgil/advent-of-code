# Experimental!
~ "git: github.com/amatgil/steal-gift" ~ AoCGet

# ====== INPUTS ======
Input ← AoCGet 2024 10

# ====== Shared ======
$ 89010123
$ 78121874
$ 87430965
$ 96549874
$ 45678903
$ 32019012
$ 01329801
$ 10456732
Sample ←

Neighs ← ▽=₁˜-⊙:⊃(⤚≡⊡⊙¤⧋+A₂|⊡)
Sol! ← (
  ⊜≡⋕⊸≠@\n       # parse
  ⬚∞(⌝↘¯⟜⌝↘) 1_1 # pad with inf
  ⊃(⊚=₀|¤)
  /+≡◇^0⍚(path(Neighs|≍9⊡))
)

# ====== PART 1 ======
Part₁ ← Sol!(/+◰≡⊣)

# ====== PART 2 ======
Part₂ ← Sol!⧻

# ====== TESTS =======
⍤⤙≍ 36 Part₁ Sample
⍤⤙≍ 81 Part₂ Sample

# ======== RUNNING ========
$"Part one solution is: '_'" Part₁ Input

$"Part two solution is: '_'" Part₂ Input
