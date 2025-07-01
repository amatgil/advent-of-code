# Experimental!
~ "git: github.com/amatgil/steal-gift" ~ AoCGet

# ====== INPUTS ======
Input     ← AoCGet 2024 14
InputSize ← 101_103
$ p=0,4 v=3,-3
$ p=6,3 v=-1,-3
$ p=10,3 v=-1,2
$ p=2,0 v=2,-1
$ p=0,0 v=1,3
$ p=3,0 v=-2,-2
$ p=7,6 v=-1,-3
$ p=3,0 v=-1,-2
$ p=9,3 v=2,3
$ p=7,3 v=-1,2
$ p=2,4 v=2,-3
$ p=9,5 v=-3,-3
Sample     ←
SampleSize ← 11_7

# ====== Shared ======
Parse ← ⊜(∩⊟∩₄⋕°$"p=_,_ v=_,_")⊸≠@\n # poss vels ?

# ====== PART 1 ======
# Grid is visualizable with ⍉°⊚
# ? Input Size
Part₁ ← (
  Parse
  ⊃(⌊÷2⋅⋅∘|⧋˜◿⊙⋅∘)⍥₁₀₀⊸+
  ⊸(+⧋⊃(<|¯>))     # mask of quadrants
  /×⊕⧻⊛∩⌞▽⊸(≠₀≡/×) # group per quadrant class
)

# ====== PART 2 ======
Part₂ ← (
  M ← 10000 # upper bound
  Parse
  ⊸⧋˜◿⊸⍥(⊃(⊙⊙∘|∘)⊸+)M               # Compute 
  ≡(⬚0˜↙⍉°⊚) ⊙(¤⇌)                  # Materialize frames
  ≡(⬚0+ layout20 °⋕)°⊏              # Burn in frame number
  \+ ♭⇌⍢(⊂+2_¯2⊸⊢|<M /↥⊢) [[70 31]] # filter for the ones...
  ⊏ +1 ▽⊸<-₁M                       # ...with stripes
  &gifs 4                           # look for tree, add 1 to frame number 
)

# ====== TESTS =======
# Input here
⍤⤙≍ 12 Part₁ Sample SampleSize

# ======== RUNNING ========
&p$"Part one solution is: '_'" Part₁ Input InputSize
Part₂ Input InputSize # look for tree in gif, add 1 to frame nº
