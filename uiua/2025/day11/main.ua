# Experimental!
# ~ "git: github.com/amatgil/steal-gift" ~ AoCGet

~ "/home/casenc/code/uiua/steal-gift/lib.ua" ~ AoCGet
# ====== INPUTS ======
I ← AoCGet 2025 11

$ aaa: you hhh
$ you: bbb ccc
$ bbb: ddd eee
$ ccc: ddd eee fff
$ ddd: ggg
$ eee: out
$ fff: out
$ ggg: out
$ hhh: ccc fff iii
$ iii: out
I₁ ←

$ svr: aaa bbb
$ aaa: fft
$ fft: ccc
$ bbb: tty
$ tty: ccc
$ ccc: ddd eee
$ ddd: hub
$ hub: fff
$ eee: dac
$ dac: fff
$ fff: ggg hhh
$ ggg: out
$ hhh: out
$ fft: aaa
I₂ ←
# Parse′ ← ≡°□°⊟⍉⊜(⍜⊣⍚(⊜∘⊸≠@\s)⊜□⊸≠@:)⊸≠@\n
Parse ← ⊜(⊃(↙|□⊜∘⊸≠@\s↘+₂) ⊸(⊢⊚=@:))⊸≠@\n
MVP   ← ⊞(/+×)⊙⍉

M ← ⬚0↙+₁˙⊟⊃⧻⬚0≡⌞◇(°⊚⊗)
P₁ ← (
  Parse
  ⊓(⊂⌞"out"|⊂⌞□["out"])
  ⊃(°⊚˜⊗"you"|M)
  ⬚0↙◡⋅⧻
  ⊓⊢◌⍜¤(⍥⊸MVP∞)
)

P₂ ← (
  Parse
  ⊓(⊂⌞"out"|⊂⌞□["out"])
  ⊃(°⊚˜⊗ "svr"|M|˜⊗"fft"_"dac")
  ⍥₂(⊂⌞0)¤⬚0↙◡⋅⧻
  ⊙⍜˜⊏(×i)
  ⊓(⊢⊢|◌)⍥⊸(+↻₁°ℂMVP)∞
)

⊓($Time|$P₁|$P₂)⍜now(⊃P₁P₂ I)
