# Experimental!

~ "git: github.com/amatgil/steal-gift" ~ AoCGet

# ====== INPUTS ======
Input ← AoCGet 2024 5

# ====== Shared ======

# ==== = PART 1 ======
Parse ← (
  ↘₂∩°□°⊟ ⊕□\↧⊸(¬⦷"\n\n")
  ⊓(⊜(□⊜⋕⊸(≠@,))⊸(≠@\n)|⊜(⊜⋕⊸(≠@|))⊸(≠@\n))
)
IsValid ← (
  ⊸(⇡⧻)
  ⊙∩¤
  ≡( # Index Seq Map
    ⊃(↙
    | ↘+₁
    | ⊃(⊣⍜⊙⍉▽⊢|⊢⍜⊙⍉▽⊣)⊸= ⊡)
    # before after MustAfter MustBefore
    ⊃(⊙⋅⊙⋅|⋅⊙⋅⊙)
    # before MustAfter afterMustBefore
    =₀+∩(/+∈)
  )
  /×
)

Part₁ ← (
  Parse
  ⊙(¤⍉)
  ⟜≡(IsValid°□)
  ˜▽
  ≡(⊡⌊÷2 ⊸⧻ °□)
  /+
)

# ====== PART 2 ======
# Insertion sort = ∧(⍣(⍜⊙↘⊂ ⊃⋅⊙◌⊙⋅∘ ⊢⊚|⍜⊙⇌⊂◌) ◡> dip[]

# ? A B Map
GoesBefore ← ∈ ⍜⍉⊣ ▽ ⍜⍉⊢ ⊸= ⊃(⊙⋅⊙|⋅⊙⋅)

# SortedSeq ? Seq Map
Sort ← (
  ⊙¤ :°⊂
  ⊙◌∧(?⍣(⍜⊙↘⊂ ⊃⋅⊙◌⊙⋅∘ ⊢⊚|⍜⊙⇌⊂ ◌) ? ◡≡(¬GoesBefore?)?)
)

Part₂ ← (
  Parse
  ⊙(¤⍉)
  ⟜⊸≡(¬IsValid°□)
  ˜▽
  ≡(⊡⌊÷2 ⊸⧻ Sort ⊙¤ °□)
)

# ====== TESTS =======
$ 47|53
$ 97|13
$ 97|61
$ 97|47
$ 75|29
$ 61|13
$ 75|53
$ 29|13
$ 97|29
$ 53|29
$ 61|53
$ 97|53
$ 61|29
$ 47|13
$ 75|47
$ 97|75
$ 47|61
$ 75|61
$ 47|29
$ 75|13
$ 53|13
$ 
$ 75,47,61,53,29
$ 97,61,53,29,13
$ 75,29,13
$ 75,97,47,61,53
$ 61,13,29
$ 97,13,75,29,47
X ←

⍤⤙≍ 143 Part₁ X
⍤⤙≍ 123 Part₂ X

# ======== RUNNING ========
$"Part one solution is: '_'" Part₁ Input

# $"Part two solution is: '_'" Part₂ Input

$POne ⊙◌⍜now Part₁ X
$PTwo ⊙◌⍜now Part₂ X
