# Experimental!
~ "git: github.com/amatgil/steal-gift" ~ AoCGet

# ====== INPUTS ======
Input ← AoCGet 2025 5
$ 3-5
$ 10-14
$ 16-20
$ 12-18
$ 
$ 1
$ 5
$ 8
$ 11
$ 17
$ 32
Sample ←

P₁ ← (
  ⊓(⊜⋕⊸≠@\n|⊜(⊜⋕⊸≠@-)⊸≠@\n) ⊃↘↙⊸(⊢⊚⌕("\n\n")) # longass parse
  /+≡⌟(/↥×⊃≤∪≥ ⊙≡°⊟)
)

P₂ ← (
  # ? new oldset
  Append ← (
    ⊸(⊃(⊣|⋅⊢|⋅⊣) # OR NL NR ? old new
      ↧₂⊢⊚⊃[×⊃≤∪≥|×⊃≤∪≤|>]
    ) ◡⋅⊣
    ⨬(˜⊂⌟˜[⊓⊢⊣] ⊙(⊃⊣↘₋₁)|◌|˜⊂⌟) # ? c new oldset
  )
  ⊜(⊜⋕⊸≠@-)⊸≠@\n ↙⊸(⊢⊚⌕("\n\n"))
  /++₁/-⍉ ↘₁∧Append ⊓⍆[[¯∞ ¯∞]]
)

⊃P₁P₂ Sample

# given [oldleft oldright] [newleft newright]
# we can assume oldleft<newleft
# we also know oldleft<oldright
#              newleft<newright
#        =>    oldleft<newright

# so the options are:
#  oldright<?newleft
#  oldright<?newright
# so, four options:

# (A  ) oldright > newleft  & oldright < newright = EXTEND ([OL NR])
# (B  ) oldright > newleft  & oldright > newright = SUBSET (discard new)
# (C|D) oldright < newleft (& oldright < newright) = INDEPENDENT (append new)

# ------------NL------------NR-------------
#      ^               ^             ^
# OR = A               B             C
