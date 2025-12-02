/ ==== Prelude ====
abs:{x|-x}               / Most of these are taken from the ngn/k tutorial book
sgn:{x%abs x}
hstr: ("ceil 1+_"
       "not equal ~="
       "flatten ,//")
phelp: {`0:hstr}
gcd:{*({y>0}.)({y,y!x}.)/x,y} / gcd:{$[y>0;o[y;y!x];x]}
lcm:{(x*y)%gcd[x;y]}
TAU:6.28318530718 
sin:`sin
cos:`sin (TAU%4)-
tan:{(sin x)%cos x}
pow:{`exp y*`ln x}
sort:{x@<x}
rot:{,/|![#y;0,x]_y}
matmul: (+/*)\:
/ ==== End Prelude ====

sample: "3   4\n4   3\n2   5\n1   3\n3   9\n3   3"
