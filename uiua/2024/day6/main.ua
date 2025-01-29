# Experimental!
~ "git: github.com/amatgil/steal-gift" ~ AoCGet

# ====== INPUTS ======
Input ← AoCGet 2024 6

# ====== Shared ======
# "eboard" means "empty board"(no guard)
Guards      ← |0 "^>v<"                           # ?
GuardPos    ← |1 ⊢⊚∊Guards                        # pos ? board
DeltaOf     ← |1 ⍣(¯1_0°@^|0_¯1°@<|1_0°@v|0_1°@>) # delta ? guard
RemoveGuard ← |2 ⍜⊡⋅@.                            # board ? pos board
RotGuard    ← |1 ⍜(⊗:Guards|◿₄+₁)                 # rotguard ? guard
UnRotGuard  ← |1 ⍜(⊗:Guards|◿₄-₁)                 # rotguard ? guard
UndoAdv     ← |2.2 -: ⊙⊸DeltaOf                   # postpos postdir ? prepos predir
SetSubtr    ← |2 ▽⊸(¬∊) ¤                         # A-B ? A B

# pos dir ? pos dir eboard
NextFrame ← |3.2 (
  +⊙⊸DeltaOf                             # possiblepos dir eboard
  ◡(≍ @# ⍣(⊡|⍤"OOB"0) ⊙◌)                # Should we rotate?
  ⨬(∘|⊸NextFrame ⊃(⊙◌UndoAdv|⋅RotGuard)) # Adjust 
  ⊙⊙◌                                    # delete eboard
)

# ====== PART 1 ======
Part₁ ← (
  ⊜∘⊸≠@\n
  ⟜⊃(⊡|RemoveGuard) ⊸GuardPos             # pos dir eboard
  ⊙⊙⊙1                                    # "keep going?"
  [⋅⋅⋅⋅◌⍢(⍣(⤚∩₄∘ ⊸NextFrame|⊙⊙⊙⋅0)|⋅⋅⋅∘)] # pos dir eboard kg
  ⧻◴
)

# ====== PART 2 ======

# bool ? pos dir eboard
IsThereLoop ← |3 (
  # TODO: THIS BROKE-Y, FIX-Y
  # kg -> {1 (yes), 2 (no, oob), 3 (no, loop) }
  ⊙⊙⊙(1 {}) # pos dir eboard kg acc
  ⍢(⍣(⨬(⊃(⊙⊙⊙⊙|?⊂ {[⊙∘]} ⊙⊙⋅⋅⊙)|⊙⊙⊙⋅3) ◡(∊: {[⊙∘]}⊙⊙⋅⋅⊙) ⊸NextFrame|⊙⊙⊙⋅2)
  | ⋅⋅⋅=₁)
  =₃⋅⋅⋅⊙◌ ?
)

Part₂ ← (
  ⊜∘⊸≠@\n
  ⊃(♭₋₁⇡⋅△|∘|⊡|RemoveGuard) ⊸GuardPos # pos dir eboard
  ⊸(SetSubtr:)                        # do not override guard
  ≡(IsThereLoop⊃(⋅⊙⊙|⍜⊡⋅@# ⊙⋅⋅⊙)) ⊙∩₃¤
  /+ dump(@d&ims↯10_10)
)

# ====== TESTS =======
$ ....#.....
$ .........#
$ ..........
$ ..#.......
$ .......#..
$ ..........
$ .#..^.....
$ ........#.
$ #.........
$ ......#...
⍤⤙≍ 41 Part₁ .
⍤⤙≍ 6 Part₂

# ======== RUNNING ========
𝄈$"Part one solution is: '_' (_s)" ⍜nowPart₁ Input

# 𝄈$"Part two solution is: '_' (_s)" ⍜nowPart₂ Input
