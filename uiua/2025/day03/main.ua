# Experimental!
~ "git: github.com/amatgil/steal-gift" ~ AoCGet

# ====== INPUTS ======
Input ← AoCGet 2025 3

$ 987654321111111
$ 811111111111119
$ 234234234234278
$ 818181911112111
Sample ←

Parse ← ⊜∘⊸≠@\n
Part₁ ← /+≡(/↥≡⋕⧅₂<) Parse
Part₂ ← (
  Parse
  ≡(
    ⊓≡⋕[]
    ◌⍥₁₂(⊃(↘+₁⤚(⊢⊚=)|⋅⊂) ˜⟜(/↥˜↘) -11◡⋅⧻)
  )
  /+≡°⊥₁₀
)
⍤⤙≍ 357 Part₁ Sample
⍤⤙≍ 3121910778619 Part₂ Sample
