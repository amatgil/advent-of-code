# Experimental!
~ "git: github.com/amatgil/steal-gift" ~ AoCGet

~ "git: github.com/Omnikar/uiua-plot" ~ Data Plot
# ====== INPUTS ======
I ← AoCGet 2025 9
$ 7,1
$ 11,1
$ 11,7
$ 9,7
$ 9,5
$ 2,5
$ 2,3
$ 7,3
Sample ←

Part₁ ← /↥♭˙⊞(/×+₁⌵-) ⊜(⊜⋕⊸≠@,)⊸≠@\n
# Disp  ← ⌝↘¯⟜⌝↘2_2 °⊚
# Disp  ← &fwa"/tmp/plot"img"png" Plot Data

# idea: per rectangle, for each x (and then each y, not nested, because rectangle)
# we check for point-in-polygon

Between ← /⊂ ⧋+⟜(⇡-) ⍜(⍆□₂|⍜⊣+₁)
Between ← ⧋+⟜(/⊂⇡↥₁-)⍜□₂⍆
Between ← /⊂ ⧋+⟜(⨬(¯⇡⌵|⇡)⊸(=₁±/↧) ⍜▽⋅1⊸=₀ -) # negative range is ass..
Sample
⊜(⊜⋕⊸≠@,)⊸≠@\n
⊃(⊃⊡₄⊡₇|pop /◇⊂⧈(□Between) ⊂⌟⤙⊢)
⤚-⊙+₁⍜□₂⍆ ?₂

# ≡⌟+≡⨬(˜⊟|⊟) ⊙0 ⊃(⊢⊚↧1|⇡/↥) ⊃(⌵-)↧ 7_1 11_1

# /⊂⧋+⟜(⇡ ⍜▽⋅1⊸=₀ -)
# for(⍜⊣+₁ sortbox,2)
# ⟜(/⊂⇡-) ⍜(⍆□₂|⍜⊣+₁)

# Part₁ Sample

# Sample
# ⊜(⊜⋕⊸≠@,)⊸≠@\n
# ⍜⍉≡⍜⍆⊛ # Compactify (per axis! since they're linearly independent)
# /◇⊂⧈(□Between) ˜⊂⊸⊢
# Disp
