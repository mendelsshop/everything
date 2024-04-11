
; ModuleID = 'examples/goto.everything'
source_filename = "examples/goto.everything"

@printf-format = private unnamed_addr constant [3 x i8] c"()\00", align 1
@true = private unnamed_addr constant [5 x i8] c"true\00", align 1
@false = private unnamed_addr constant [6 x i8] c"false\00", align 1
@printf-format.1 = private unnamed_addr constant [5 x i8] c"%.2f\00", align 1
@printf-format.2 = private unnamed_addr constant [5 x i8] c"%.*s\00", align 1
@printf-format.3 = private unnamed_addr constant [5 x i8] c"%.*s\00", align 1
@printf-format.4 = private unnamed_addr constant [6 x i8] c"label\00", align 1
@printf-format.5 = private unnamed_addr constant [2 x i8] c"(\00", align 1
@printf-format.6 = private unnamed_addr constant [2 x i8] c" \00", align 1
@printf-format.7 = private unnamed_addr constant [2 x i8] c")\00", align 1
@printf-format.8 = private unnamed_addr constant [10 x i8] c"primitive\00", align 1
@printf-format.9 = private unnamed_addr constant [6 x i8] c"thunk\00", align 1
@printf-format.10 = private unnamed_addr constant [22 x i8] c"user define procedure\00", align 1
@"error exit" = private unnamed_addr constant [13 x i8] c"invalid type\00", align 1
@"error exit.11" = private unnamed_addr constant [13 x i8] c"invalid type\00", align 1
@printf-format.12 = private unnamed_addr constant [7 x i8] c"hempty\00", align 1
@"error exit.13" = private unnamed_addr constant [22 x i8] c"cannot compare labels\00", align 1
@"error exit.14" = private unnamed_addr constant [22 x i8] c"cannot compare thunks\00", align 1
@"error exit.15" = private unnamed_addr constant [23 x i8] c"cannot compare lambdas\00", align 1
@"error exit.16" = private unnamed_addr constant [26 x i8] c"cannot compare primitives\00", align 1
@"\0A" = private unnamed_addr constant [2 x i8] c"\0A\00", align 1
@"error exit.17" = private unnamed_addr constant [30 x i8] c"type mismtatch expected cons\0A\00", align 1
@"error exit.18" = private unnamed_addr constant [30 x i8] c"type mismtatch expected cons\0A\00", align 1
@"error exit.19" = private unnamed_addr constant [30 x i8] c"type mismtatch expected cons\0A\00", align 1
@"error exit.20" = private unnamed_addr constant [30 x i8] c"type mismtatch expected cons\0A\00", align 1
@"error exit.21" = private unnamed_addr constant [30 x i8] c"type mismtatch expected cons\0A\00", align 1
@"error exit.22" = private unnamed_addr constant [30 x i8] c"type mismtatch expected cons\0A\00", align 1
@"error exit.23" = private unnamed_addr constant [30 x i8] c"type mismtatch expected cons\0A\00", align 1
@"error exit.24" = private unnamed_addr constant [30 x i8] c"type mismtatch expected cons\0A\00", align 1
@"error exit.25" = private unnamed_addr constant [30 x i8] c"type mismtatch expected cons\0A\00", align 1
@"error exit.26" = private unnamed_addr constant [30 x i8] c"type mismtatch expected cons\0A\00", align 1
@"error exit.27" = private unnamed_addr constant [30 x i8] c"type mismtatch expected cons\0A\00", align 1
@"error exit.28" = private unnamed_addr constant [30 x i8] c"type mismtatch expected cons\0A\00", align 1
@"error exit.29" = private unnamed_addr constant [30 x i8] c"type mismtatch expected cons\0A\00", align 1
@"error exit.30" = private unnamed_addr constant [30 x i8] c"type mismtatch expected cons\0A\00", align 1
@"error exit.31" = private unnamed_addr constant [30 x i8] c"type mismtatch expected cons\0A\00", align 1
@"error exit.32" = private unnamed_addr constant [30 x i8] c"type mismtatch expected cons\0A\00", align 1
@"error exit.33" = private unnamed_addr constant [30 x i8] c"type mismtatch expected cons\0A\00", align 1
@"error exit.34" = private unnamed_addr constant [32 x i8] c"type mismtatch expected number\0A\00", align 1
@"error exit.35" = private unnamed_addr constant [30 x i8] c"type mismtatch expected cons\0A\00", align 1
@"error exit.36" = private unnamed_addr constant [32 x i8] c"type mismtatch expected number\0A\00", align 1
@"error exit.37" = private unnamed_addr constant [32 x i8] c"type mismtatch expected number\0A\00", align 1
@"+1.38" = private unnamed_addr constant [3 x i8] c"+1\00", align 1
@"error exit.39" = private unnamed_addr constant [30 x i8] c"type mismtatch expected cons\0A\00", align 1
@"error exit.40" = private unnamed_addr constant [30 x i8] c"type mismtatch expected cons\0A\00", align 1
@"error exit.41" = private unnamed_addr constant [30 x i8] c"type mismtatch expected cons\0A\00", align 1
@"error exit.42" = private unnamed_addr constant [30 x i8] c"type mismtatch expected cons\0A\00", align 1
@"error exit.43" = private unnamed_addr constant [30 x i8] c"type mismtatch expected cons\0A\00", align 1
@"error exit.44" = private unnamed_addr constant [30 x i8] c"type mismtatch expected cons\0A\00", align 1
@"error exit.45" = private unnamed_addr constant [30 x i8] c"type mismtatch expected cons\0A\00", align 1
@"error exit.46" = private unnamed_addr constant [30 x i8] c"type mismtatch expected cons\0A\00", align 1
@"error exit.47" = private unnamed_addr constant [30 x i8] c"type mismtatch expected cons\0A\00", align 1
@"error exit.48" = private unnamed_addr constant [30 x i8] c"type mismtatch expected cons\0A\00", align 1
@"error exit.49" = private unnamed_addr constant [30 x i8] c"type mismtatch expected cons\0A\00", align 1
@"error exit.50" = private unnamed_addr constant [30 x i8] c"type mismtatch expected cons\0A\00", align 1
@"error exit.51" = private unnamed_addr constant [30 x i8] c"type mismtatch expected cons\0A\00", align 1
@"error exit.52" = private unnamed_addr constant [30 x i8] c"type mismtatch expected cons\0A\00", align 1
@"error exit.53" = private unnamed_addr constant [30 x i8] c"type mismtatch expected cons\0A\00", align 1
@"error exit.54" = private unnamed_addr constant [30 x i8] c"type mismtatch expected cons\0A\00", align 1
@"error exit.55" = private unnamed_addr constant [30 x i8] c"type mismtatch expected cons\0A\00", align 1
@"error exit.56" = private unnamed_addr constant [30 x i8] c"type mismtatch expected cons\0A\00", align 1
@"error exit.57" = private unnamed_addr constant [30 x i8] c"type mismtatch expected cons\0A\00", align 1
@"error exit.58" = private unnamed_addr constant [30 x i8] c"type mismtatch expected cons\0A\00", align 1
@"error exit.59" = private unnamed_addr constant [30 x i8] c"type mismtatch expected cons\0A\00", align 1
@"error exit.60" = private unnamed_addr constant [30 x i8] c"type mismtatch expected cons\0A\00", align 1
@"error exit.61" = private unnamed_addr constant [30 x i8] c"type mismtatch expected cons\0A\00", align 1
@"error exit.62" = private unnamed_addr constant [30 x i8] c"type mismtatch expected cons\0A\00", align 1
@"error exit.63" = private unnamed_addr constant [30 x i8] c"type mismtatch expected cons\0A\00", align 1
@"error exit.64" = private unnamed_addr constant [30 x i8] c"type mismtatch expected cons\0A\00", align 1
@"error exit.65" = private unnamed_addr constant [30 x i8] c"type mismtatch expected cons\0A\00", align 1
@"error exit.66" = private unnamed_addr constant [30 x i8] c"type mismtatch expected cons\0A\00", align 1
@"error exit.67" = private unnamed_addr constant [30 x i8] c"type mismtatch expected cons\0A\00", align 1
@"error exit.68" = private unnamed_addr constant [30 x i8] c"type mismtatch expected cons\0A\00", align 1
@"error exit.69" = private unnamed_addr constant [30 x i8] c"type mismtatch expected cons\0A\00", align 1
@"error exit.70" = private unnamed_addr constant [30 x i8] c"type mismtatch expected cons\0A\00", align 1
@"error exit.71" = private unnamed_addr constant [30 x i8] c"type mismtatch expected cons\0A\00", align 1
@"error exit.72" = private unnamed_addr constant [30 x i8] c"type mismtatch expected cons\0A\00", align 1
@"error exit.73" = private unnamed_addr constant [30 x i8] c"type mismtatch expected cons\0A\00", align 1
@"error exit.74" = private unnamed_addr constant [30 x i8] c"type mismtatch expected cons\0A\00", align 1
@"error exit.75" = private unnamed_addr constant [30 x i8] c"type mismtatch expected cons\0A\00", align 1
@"error exit.76" = private unnamed_addr constant [30 x i8] c"type mismtatch expected cons\0A\00", align 1
@"error exit.77" = private unnamed_addr constant [30 x i8] c"type mismtatch expected cons\0A\00", align 1
@"error exit.78" = private unnamed_addr constant [30 x i8] c"type mismtatch expected cons\0A\00", align 1
@"error exit.79" = private unnamed_addr constant [30 x i8] c"type mismtatch expected cons\0A\00", align 1
@"error exit.80" = private unnamed_addr constant [30 x i8] c"type mismtatch expected cons\0A\00", align 1
@"error exit.81" = private unnamed_addr constant [30 x i8] c"type mismtatch expected cons\0A\00", align 1
@"error exit.82" = private unnamed_addr constant [30 x i8] c"type mismtatch expected cons\0A\00", align 1
@"error exit.83" = private unnamed_addr constant [30 x i8] c"type mismtatch expected cons\0A\00", align 1
@"error exit.84" = private unnamed_addr constant [30 x i8] c"type mismtatch expected cons\0A\00", align 1
@"error exit.85" = private unnamed_addr constant [30 x i8] c"type mismtatch expected cons\0A\00", align 1
@"error exit.86" = private unnamed_addr constant [30 x i8] c"type mismtatch expected cons\0A\00", align 1
@"error exit.87" = private unnamed_addr constant [30 x i8] c"type mismtatch expected cons\0A\00", align 1
@"error exit.88" = private unnamed_addr constant [30 x i8] c"type mismtatch expected cons\0A\00", align 1
@"error exit.89" = private unnamed_addr constant [30 x i8] c"type mismtatch expected cons\0A\00", align 1
@"error exit.90" = private unnamed_addr constant [30 x i8] c"type mismtatch expected cons\0A\00", align 1
@"error exit.91" = private unnamed_addr constant [30 x i8] c"type mismtatch expected cons\0A\00", align 1
@"error exit.92" = private unnamed_addr constant [30 x i8] c"type mismtatch expected cons\0A\00", align 1
@"error exit.93" = private unnamed_addr constant [30 x i8] c"type mismtatch expected cons\0A\00", align 1
@"error exit.94" = private unnamed_addr constant [30 x i8] c"type mismtatch expected cons\0A\00", align 1
@"error exit.95" = private unnamed_addr constant [30 x i8] c"type mismtatch expected cons\0A\00", align 1
@"error exit.96" = private unnamed_addr constant [30 x i8] c"type mismtatch expected cons\0A\00", align 1
@"error exit.97" = private unnamed_addr constant [30 x i8] c"type mismtatch expected cons\0A\00", align 1
@"error exit.98" = private unnamed_addr constant [30 x i8] c"type mismtatch expected cons\0A\00", align 1
@"error exit.99" = private unnamed_addr constant [30 x i8] c"type mismtatch expected cons\0A\00", align 1
@"error exit.100" = private unnamed_addr constant [30 x i8] c"type mismtatch expected cons\0A\00", align 1
@"error exit.101" = private unnamed_addr constant [30 x i8] c"type mismtatch expected cons\0A\00", align 1
@"error exit.102" = private unnamed_addr constant [30 x i8] c"type mismtatch expected cons\0A\00", align 1
@"error exit.103" = private unnamed_addr constant [30 x i8] c"type mismtatch expected cons\0A\00", align 1
@"error exit.104" = private unnamed_addr constant [30 x i8] c"type mismtatch expected cons\0A\00", align 1
@"error exit.105" = private unnamed_addr constant [30 x i8] c"type mismtatch expected cons\0A\00", align 1
@"error exit.106" = private unnamed_addr constant [30 x i8] c"type mismtatch expected cons\0A\00", align 1
@"error exit.107" = private unnamed_addr constant [30 x i8] c"type mismtatch expected cons\0A\00", align 1
@"error exit.108" = private unnamed_addr constant [30 x i8] c"type mismtatch expected cons\0A\00", align 1
@"error exit.109" = private unnamed_addr constant [30 x i8] c"type mismtatch expected cons\0A\00", align 1
@"error exit.110" = private unnamed_addr constant [30 x i8] c"type mismtatch expected cons\0A\00", align 1
@"error exit.111" = private unnamed_addr constant [30 x i8] c"type mismtatch expected cons\0A\00", align 1
@"error exit.112" = private unnamed_addr constant [30 x i8] c"type mismtatch expected cons\0A\00", align 1
@"error exit.113" = private unnamed_addr constant [30 x i8] c"type mismtatch expected cons\0A\00", align 1
@"error exit.114" = private unnamed_addr constant [30 x i8] c"type mismtatch expected cons\0A\00", align 1
@"error exit.115" = private unnamed_addr constant [30 x i8] c"type mismtatch expected cons\0A\00", align 1
@"error exit.116" = private unnamed_addr constant [30 x i8] c"type mismtatch expected cons\0A\00", align 1
@"error exit.117" = private unnamed_addr constant [30 x i8] c"type mismtatch expected cons\0A\00", align 1
@"error exit.118" = private unnamed_addr constant [30 x i8] c"type mismtatch expected cons\0A\00", align 1
@"error exit.119" = private unnamed_addr constant [30 x i8] c"type mismtatch expected cons\0A\00", align 1
@"error exit.120" = private unnamed_addr constant [30 x i8] c"type mismtatch expected cons\0A\00", align 1
@"error exit.121" = private unnamed_addr constant [30 x i8] c"type mismtatch expected cons\0A\00", align 1
@"error exit.122" = private unnamed_addr constant [30 x i8] c"type mismtatch expected cons\0A\00", align 1
@"error exit.123" = private unnamed_addr constant [30 x i8] c"type mismtatch expected cons\0A\00", align 1
@"error exit.124" = private unnamed_addr constant [30 x i8] c"type mismtatch expected cons\0A\00", align 1
@"error exit.125" = private unnamed_addr constant [30 x i8] c"type mismtatch expected cons\0A\00", align 1
@"error exit.126" = private unnamed_addr constant [30 x i8] c"type mismtatch expected cons\0A\00", align 1
@"error exit.127" = private unnamed_addr constant [30 x i8] c"type mismtatch expected cons\0A\00", align 1
@"error exit.128" = private unnamed_addr constant [30 x i8] c"type mismtatch expected cons\0A\00", align 1
@"error exit.129" = private unnamed_addr constant [30 x i8] c"type mismtatch expected cons\0A\00", align 1
@"error exit.130" = private unnamed_addr constant [30 x i8] c"type mismtatch expected cons\0A\00", align 1
@"error exit.131" = private unnamed_addr constant [30 x i8] c"type mismtatch expected cons\0A\00", align 1
@"error exit.132" = private unnamed_addr constant [30 x i8] c"type mismtatch expected cons\0A\00", align 1
@"error exit.133" = private unnamed_addr constant [30 x i8] c"type mismtatch expected cons\0A\00", align 1
@"error exit.134" = private unnamed_addr constant [30 x i8] c"type mismtatch expected cons\0A\00", align 1
@"error exit.135" = private unnamed_addr constant [30 x i8] c"type mismtatch expected cons\0A\00", align 1
@"error exit.136" = private unnamed_addr constant [30 x i8] c"type mismtatch expected cons\0A\00", align 1
@"error exit.137" = private unnamed_addr constant [30 x i8] c"type mismtatch expected cons\0A\00", align 1
@"error exit.138" = private unnamed_addr constant [30 x i8] c"type mismtatch expected cons\0A\00", align 1
@"error exit.139" = private unnamed_addr constant [30 x i8] c"type mismtatch expected cons\0A\00", align 1
@"error exit.140" = private unnamed_addr constant [30 x i8] c"type mismtatch expected cons\0A\00", align 1
@"error exit.141" = private unnamed_addr constant [30 x i8] c"type mismtatch expected cons\0A\00", align 1
@"error exit.142" = private unnamed_addr constant [30 x i8] c"type mismtatch expected cons\0A\00", align 1
@"error exit.143" = private unnamed_addr constant [30 x i8] c"type mismtatch expected cons\0A\00", align 1
@"error exit.144" = private unnamed_addr constant [30 x i8] c"type mismtatch expected cons\0A\00", align 1
@"error exit.145" = private unnamed_addr constant [30 x i8] c"type mismtatch expected cons\0A\00", align 1
@"error exit.146" = private unnamed_addr constant [30 x i8] c"type mismtatch expected cons\0A\00", align 1
@"error exit.147" = private unnamed_addr constant [30 x i8] c"type mismtatch expected cons\0A\00", align 1
@"error exit.148" = private unnamed_addr constant [30 x i8] c"type mismtatch expected cons\0A\00", align 1
@"error exit.149" = private unnamed_addr constant [30 x i8] c"type mismtatch expected cons\0A\00", align 1
@"error exit.150" = private unnamed_addr constant [30 x i8] c"type mismtatch expected cons\0A\00", align 1
@"error exit.151" = private unnamed_addr constant [30 x i8] c"type mismtatch expected cons\0A\00", align 1
@"error exit.152" = private unnamed_addr constant [30 x i8] c"type mismtatch expected cons\0A\00", align 1
@"error exit.153" = private unnamed_addr constant [30 x i8] c"type mismtatch expected cons\0A\00", align 1
@"error exit.154" = private unnamed_addr constant [30 x i8] c"type mismtatch expected cons\0A\00", align 1
@"error exit.155" = private unnamed_addr constant [30 x i8] c"type mismtatch expected cons\0A\00", align 1
@"error exit.156" = private unnamed_addr constant [30 x i8] c"type mismtatch expected cons\0A\00", align 1
@"error exit.157" = private unnamed_addr constant [30 x i8] c"type mismtatch expected cons\0A\00", align 1
@"error exit.158" = private unnamed_addr constant [30 x i8] c"type mismtatch expected cons\0A\00", align 1
@"error exit.159" = private unnamed_addr constant [30 x i8] c"type mismtatch expected cons\0A\00", align 1
@"error exit.160" = private unnamed_addr constant [30 x i8] c"type mismtatch expected cons\0A\00", align 1
@"error exit.161" = private unnamed_addr constant [30 x i8] c"type mismtatch expected cons\0A\00", align 1
@"error exit.162" = private unnamed_addr constant [30 x i8] c"type mismtatch expected cons\0A\00", align 1
@"error exit.163" = private unnamed_addr constant [30 x i8] c"type mismtatch expected cons\0A\00", align 1
@"error exit.164" = private unnamed_addr constant [30 x i8] c"type mismtatch expected cons\0A\00", align 1
@"error exit.165" = private unnamed_addr constant [30 x i8] c"type mismtatch expected cons\0A\00", align 1
@"error exit.166" = private unnamed_addr constant [30 x i8] c"type mismtatch expected cons\0A\00", align 1
@newline.167 = private unnamed_addr constant [8 x i8] c"newline\00", align 1
@"=" = private unnamed_addr constant [2 x i8] c"=\00", align 1
@print.168 = private unnamed_addr constant [6 x i8] c"print\00", align 1
@not.169 = private unnamed_addr constant [4 x i8] c"not\00", align 1
@"set_cdr!" = private unnamed_addr constant [9 x i8] c"set_cdr!\00", align 1
@"set_car!" = private unnamed_addr constant [9 x i8] c"set_car!\00", align 1
@cons.170 = private unnamed_addr constant [5 x i8] c"cons\00", align 1
@-1.171 = private unnamed_addr constant [3 x i8] c"-1\00", align 1
@car.172 = private unnamed_addr constant [4 x i8] c"car\00", align 1
@cdr.173 = private unnamed_addr constant [4 x i8] c"cdr\00", align 1
@caar.174 = private unnamed_addr constant [5 x i8] c"caar\00", align 1
@cadr.175 = private unnamed_addr constant [5 x i8] c"cadr\00", align 1
@cdar.176 = private unnamed_addr constant [5 x i8] c"cdar\00", align 1
@cddr.177 = private unnamed_addr constant [5 x i8] c"cddr\00", align 1
@caaar.178 = private unnamed_addr constant [6 x i8] c"caaar\00", align 1
@caadr.179 = private unnamed_addr constant [6 x i8] c"caadr\00", align 1
@cadar.180 = private unnamed_addr constant [6 x i8] c"cadar\00", align 1
@caddr.181 = private unnamed_addr constant [6 x i8] c"caddr\00", align 1
@cdaar.182 = private unnamed_addr constant [6 x i8] c"cdaar\00", align 1
@cdadr.183 = private unnamed_addr constant [6 x i8] c"cdadr\00", align 1
@cddar.184 = private unnamed_addr constant [6 x i8] c"cddar\00", align 1
@cdddr.185 = private unnamed_addr constant [6 x i8] c"cdddr\00", align 1
@caaaar.186 = private unnamed_addr constant [7 x i8] c"caaaar\00", align 1
@caaadr.187 = private unnamed_addr constant [7 x i8] c"caaadr\00", align 1
@caadar.188 = private unnamed_addr constant [7 x i8] c"caadar\00", align 1
@caaddr.189 = private unnamed_addr constant [7 x i8] c"caaddr\00", align 1
@cadaar.190 = private unnamed_addr constant [7 x i8] c"cadaar\00", align 1
@cadadr.191 = private unnamed_addr constant [7 x i8] c"cadadr\00", align 1
@caddar.192 = private unnamed_addr constant [7 x i8] c"caddar\00", align 1
@cadddr.193 = private unnamed_addr constant [7 x i8] c"cadddr\00", align 1
@cdaaar.194 = private unnamed_addr constant [7 x i8] c"cdaaar\00", align 1
@cdaadr.195 = private unnamed_addr constant [7 x i8] c"cdaadr\00", align 1
@cdadar.196 = private unnamed_addr constant [7 x i8] c"cdadar\00", align 1
@cdaddr.197 = private unnamed_addr constant [7 x i8] c"cdaddr\00", align 1
@cddaar.198 = private unnamed_addr constant [7 x i8] c"cddaar\00", align 1
@cddadr.199 = private unnamed_addr constant [7 x i8] c"cddadr\00", align 1
@cdddar.200 = private unnamed_addr constant [7 x i8] c"cdddar\00", align 1
@cddddr.201 = private unnamed_addr constant [7 x i8] c"cddddr\00", align 1
@nil = private unnamed_addr constant [4 x i8] c"nil\00", align 1
@compiled-procedure = private unnamed_addr constant [19 x i8] c"compiled-procedure\00", align 1
@"'0'" = private unnamed_addr constant [4 x i8] c"'0'\00", align 1
@print.202 = private unnamed_addr constant [6 x i8] c"print\00", align 1
@"error exit.203" = private unnamed_addr constant [18 x i8] c"unbound variable\0A\00", align 1
@"'0'.204" = private unnamed_addr constant [4 x i8] c"'0'\00", align 1
@"error exit.205" = private unnamed_addr constant [18 x i8] c"unbound variable\0A\00", align 1
@"'0'.206" = private unnamed_addr constant [4 x i8] c"'0'\00", align 1
@"error exit.207" = private unnamed_addr constant [18 x i8] c"unbound variable\0A\00", align 1
@newline.208 = private unnamed_addr constant [8 x i8] c"newline\00", align 1
@"error exit.209" = private unnamed_addr constant [18 x i8] c"unbound variable\0A\00", align 1
@println = private unnamed_addr constant [8 x i8] c"println\00", align 1
@ok = private unnamed_addr constant [3 x i8] c"ok\00", align 1
@println.210 = private unnamed_addr constant [8 x i8] c"println\00", align 1
@"error exit.211" = private unnamed_addr constant [18 x i8] c"unbound variable\0A\00", align 1
@gg = private unnamed_addr constant [3 x i8] c"gg\00", align 1
@gg.212 = private unnamed_addr constant [3 x i8] c"gg\00", align 1
@print.213 = private unnamed_addr constant [6 x i8] c"print\00", align 1
@"error exit.214" = private unnamed_addr constant [18 x i8] c"unbound variable\0A\00", align 1
@hi = private unnamed_addr constant [3 x i8] c"hi\00", align 1
@hi.215 = private unnamed_addr constant [3 x i8] c"hi\00", align 1
@println.216 = private unnamed_addr constant [8 x i8] c"println\00", align 1
@"error exit.217" = private unnamed_addr constant [18 x i8] c"unbound variable\0A\00", align 1
@"not skipped" = private unnamed_addr constant [12 x i8] c"not skipped\00", align 1
@"not skipped.218" = private unnamed_addr constant [12 x i8] c"not skipped\00", align 1
@println.219 = private unnamed_addr constant [8 x i8] c"println\00", align 1
@"error exit.220" = private unnamed_addr constant [18 x i8] c"unbound variable\0A\00", align 1
@aa = private unnamed_addr constant [3 x i8] c"aa\00", align 1
@aa.221 = private unnamed_addr constant [3 x i8] c"aa\00", align 1

declare void @exit(i32)

declare i32 @rand()

declare i32 @printf(ptr, ...)

declare i32 @strncmp(ptr, ptr, i32)

declare void @srand(i32)

declare i32 @time(ptr)

define i32 @main() {
entry:
  %"get time to further randomize rng" = call i32 @time(ptr null)
  call void @srand(i32 %"get time to further randomize rng")
  %env = alloca { i32, ptr }, align 8
  %argl = alloca { i32, ptr }, align 8
  %val = alloca { i32, ptr }, align 8
  %proc = alloca { i32, ptr }, align 8
  %continue = alloca { i32, ptr }, align 8
  %thunk = alloca { i32, ptr }, align 8
  %stack = alloca { { i32, ptr }, ptr }, align 8
  %flag = alloca { i32, ptr }, align 8
  %"load argl" = load { i32, ptr }, ptr %env, align 8
  %"object value3" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr (ptr, ptr null, i32 1) to i32))
  store ptr blockaddress(@main, %add1), ptr %"object value3", align 8
  %"insert value object4" = insertvalue { i32, ptr } { i32 5, ptr null }, ptr %"object value3", 1
  %"insert into struct" = insertvalue { { i32, ptr }, { i32, ptr } } zeroinitializer, { i32, ptr } %"insert value object4", 0
  %"insert into struct5" = insertvalue { { i32, ptr }, { i32, ptr } } %"insert into struct", { i32, ptr } %"load argl", 1
  %"object value6" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ { i32, ptr }, { i32, ptr } }, ptr null, i32 1) to i32))
  store { { i32, ptr }, { i32, ptr } } %"insert into struct5", ptr %"object value6", align 8
  %"insert value object7" = insertvalue { i32, ptr } { i32 9, ptr null }, ptr %"object value6", 1
  %"object value8" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  store { i32, ptr } { i32 2, ptr @"+1.38" }, ptr %"object value8", align 8
  %"insert value object9" = insertvalue { i32, ptr } { i32 4, ptr null }, ptr %"object value8", 1
  %"object value10" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr (ptr, ptr null, i32 1) to i32))
  store ptr @newline, ptr %"object value10", align 8
  %"insert value object11" = insertvalue { i32, ptr } { i32 7, ptr null }, ptr %"object value10", 1
  %"object value12" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  store { i32, ptr } { i32 7, ptr @newline.167 }, ptr %"object value12", align 8
  %"insert value object13" = insertvalue { i32, ptr } { i32 4, ptr null }, ptr %"object value12", 1
  %"car ptr" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  %"cdr ptr" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  store { i32, ptr } %"insert value object13", ptr %"car ptr", align 8
  store { i32, ptr } zeroinitializer, ptr %"cdr ptr", align 8
  %"insert car - cons" = insertvalue { ptr, ptr } zeroinitializer, ptr %"car ptr", 0
  %"insert cdr - cons" = insertvalue { ptr, ptr } %"insert car - cons", ptr %"cdr ptr", 1
  %"object value14" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ ptr, ptr }, ptr null, i32 1) to i32))
  store { ptr, ptr } %"insert cdr - cons", ptr %"object value14", align 8
  %"insert value object15" = insertvalue { i32, ptr } { i32 6, ptr null }, ptr %"object value14", 1
  %"car ptr16" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  %"cdr ptr17" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  store { i32, ptr } %"insert value object11", ptr %"car ptr16", align 8
  store { i32, ptr } zeroinitializer, ptr %"cdr ptr17", align 8
  %"insert car - cons18" = insertvalue { ptr, ptr } zeroinitializer, ptr %"car ptr16", 0
  %"insert cdr - cons19" = insertvalue { ptr, ptr } %"insert car - cons18", ptr %"cdr ptr17", 1
  %"object value20" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ ptr, ptr }, ptr null, i32 1) to i32))
  store { ptr, ptr } %"insert cdr - cons19", ptr %"object value20", align 8
  %"insert value object21" = insertvalue { i32, ptr } { i32 6, ptr null }, ptr %"object value20", 1
  %"object value22" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr (ptr, ptr null, i32 1) to i32))
  store ptr @eq, ptr %"object value22", align 8
  %"insert value object23" = insertvalue { i32, ptr } { i32 7, ptr null }, ptr %"object value22", 1
  %"object value24" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  store { i32, ptr } { i32 1, ptr @"=" }, ptr %"object value24", align 8
  %"insert value object25" = insertvalue { i32, ptr } { i32 4, ptr null }, ptr %"object value24", 1
  %"car ptr26" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  %"cdr ptr27" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  store { i32, ptr } %"insert value object25", ptr %"car ptr26", align 8
  store { i32, ptr } %"insert value object15", ptr %"cdr ptr27", align 8
  %"insert car - cons28" = insertvalue { ptr, ptr } zeroinitializer, ptr %"car ptr26", 0
  %"insert cdr - cons29" = insertvalue { ptr, ptr } %"insert car - cons28", ptr %"cdr ptr27", 1
  %"object value30" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ ptr, ptr }, ptr null, i32 1) to i32))
  store { ptr, ptr } %"insert cdr - cons29", ptr %"object value30", align 8
  %"insert value object31" = insertvalue { i32, ptr } { i32 6, ptr null }, ptr %"object value30", 1
  %"car ptr32" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  %"cdr ptr33" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  store { i32, ptr } %"insert value object23", ptr %"car ptr32", align 8
  store { i32, ptr } %"insert value object21", ptr %"cdr ptr33", align 8
  %"insert car - cons34" = insertvalue { ptr, ptr } zeroinitializer, ptr %"car ptr32", 0
  %"insert cdr - cons35" = insertvalue { ptr, ptr } %"insert car - cons34", ptr %"cdr ptr33", 1
  %"object value36" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ ptr, ptr }, ptr null, i32 1) to i32))
  store { ptr, ptr } %"insert cdr - cons35", ptr %"object value36", align 8
  %"insert value object37" = insertvalue { i32, ptr } { i32 6, ptr null }, ptr %"object value36", 1
  %"object value38" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr (ptr, ptr null, i32 1) to i32))
  store ptr @print, ptr %"object value38", align 8
  %"insert value object39" = insertvalue { i32, ptr } { i32 7, ptr null }, ptr %"object value38", 1
  %"object value40" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  store { i32, ptr } { i32 5, ptr @print.168 }, ptr %"object value40", align 8
  %"insert value object41" = insertvalue { i32, ptr } { i32 4, ptr null }, ptr %"object value40", 1
  %"car ptr42" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  %"cdr ptr43" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  store { i32, ptr } %"insert value object41", ptr %"car ptr42", align 8
  store { i32, ptr } %"insert value object31", ptr %"cdr ptr43", align 8
  %"insert car - cons44" = insertvalue { ptr, ptr } zeroinitializer, ptr %"car ptr42", 0
  %"insert cdr - cons45" = insertvalue { ptr, ptr } %"insert car - cons44", ptr %"cdr ptr43", 1
  %"object value46" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ ptr, ptr }, ptr null, i32 1) to i32))
  store { ptr, ptr } %"insert cdr - cons45", ptr %"object value46", align 8
  %"insert value object47" = insertvalue { i32, ptr } { i32 6, ptr null }, ptr %"object value46", 1
  %"car ptr48" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  %"cdr ptr49" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  store { i32, ptr } %"insert value object39", ptr %"car ptr48", align 8
  store { i32, ptr } %"insert value object37", ptr %"cdr ptr49", align 8
  %"insert car - cons50" = insertvalue { ptr, ptr } zeroinitializer, ptr %"car ptr48", 0
  %"insert cdr - cons51" = insertvalue { ptr, ptr } %"insert car - cons50", ptr %"cdr ptr49", 1
  %"object value52" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ ptr, ptr }, ptr null, i32 1) to i32))
  store { ptr, ptr } %"insert cdr - cons51", ptr %"object value52", align 8
  %"insert value object53" = insertvalue { i32, ptr } { i32 6, ptr null }, ptr %"object value52", 1
  %"object value54" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr (ptr, ptr null, i32 1) to i32))
  store ptr @not, ptr %"object value54", align 8
  %"insert value object55" = insertvalue { i32, ptr } { i32 7, ptr null }, ptr %"object value54", 1
  %"object value56" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  store { i32, ptr } { i32 3, ptr @not.169 }, ptr %"object value56", align 8
  %"insert value object57" = insertvalue { i32, ptr } { i32 4, ptr null }, ptr %"object value56", 1
  %"car ptr58" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  %"cdr ptr59" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  store { i32, ptr } %"insert value object57", ptr %"car ptr58", align 8
  store { i32, ptr } %"insert value object47", ptr %"cdr ptr59", align 8
  %"insert car - cons60" = insertvalue { ptr, ptr } zeroinitializer, ptr %"car ptr58", 0
  %"insert cdr - cons61" = insertvalue { ptr, ptr } %"insert car - cons60", ptr %"cdr ptr59", 1
  %"object value62" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ ptr, ptr }, ptr null, i32 1) to i32))
  store { ptr, ptr } %"insert cdr - cons61", ptr %"object value62", align 8
  %"insert value object63" = insertvalue { i32, ptr } { i32 6, ptr null }, ptr %"object value62", 1
  %"car ptr64" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  %"cdr ptr65" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  store { i32, ptr } %"insert value object55", ptr %"car ptr64", align 8
  store { i32, ptr } %"insert value object53", ptr %"cdr ptr65", align 8
  %"insert car - cons66" = insertvalue { ptr, ptr } zeroinitializer, ptr %"car ptr64", 0
  %"insert cdr - cons67" = insertvalue { ptr, ptr } %"insert car - cons66", ptr %"cdr ptr65", 1
  %"object value68" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ ptr, ptr }, ptr null, i32 1) to i32))
  store { ptr, ptr } %"insert cdr - cons67", ptr %"object value68", align 8
  %"insert value object69" = insertvalue { i32, ptr } { i32 6, ptr null }, ptr %"object value68", 1
  %"object value70" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr (ptr, ptr null, i32 1) to i32))
  store ptr @"set-cdr!", ptr %"object value70", align 8
  %"insert value object71" = insertvalue { i32, ptr } { i32 7, ptr null }, ptr %"object value70", 1
  %"object value72" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  store { i32, ptr } { i32 8, ptr @"set_cdr!" }, ptr %"object value72", align 8
  %"insert value object73" = insertvalue { i32, ptr } { i32 4, ptr null }, ptr %"object value72", 1
  %"car ptr74" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  %"cdr ptr75" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  store { i32, ptr } %"insert value object73", ptr %"car ptr74", align 8
  store { i32, ptr } %"insert value object63", ptr %"cdr ptr75", align 8
  %"insert car - cons76" = insertvalue { ptr, ptr } zeroinitializer, ptr %"car ptr74", 0
  %"insert cdr - cons77" = insertvalue { ptr, ptr } %"insert car - cons76", ptr %"cdr ptr75", 1
  %"object value78" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ ptr, ptr }, ptr null, i32 1) to i32))
  store { ptr, ptr } %"insert cdr - cons77", ptr %"object value78", align 8
  %"insert value object79" = insertvalue { i32, ptr } { i32 6, ptr null }, ptr %"object value78", 1
  %"car ptr80" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  %"cdr ptr81" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  store { i32, ptr } %"insert value object71", ptr %"car ptr80", align 8
  store { i32, ptr } %"insert value object69", ptr %"cdr ptr81", align 8
  %"insert car - cons82" = insertvalue { ptr, ptr } zeroinitializer, ptr %"car ptr80", 0
  %"insert cdr - cons83" = insertvalue { ptr, ptr } %"insert car - cons82", ptr %"cdr ptr81", 1
  %"object value84" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ ptr, ptr }, ptr null, i32 1) to i32))
  store { ptr, ptr } %"insert cdr - cons83", ptr %"object value84", align 8
  %"insert value object85" = insertvalue { i32, ptr } { i32 6, ptr null }, ptr %"object value84", 1
  %"object value86" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr (ptr, ptr null, i32 1) to i32))
  store ptr @"set-car!", ptr %"object value86", align 8
  %"insert value object87" = insertvalue { i32, ptr } { i32 7, ptr null }, ptr %"object value86", 1
  %"object value88" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  store { i32, ptr } { i32 8, ptr @"set_car!" }, ptr %"object value88", align 8
  %"insert value object89" = insertvalue { i32, ptr } { i32 4, ptr null }, ptr %"object value88", 1
  %"car ptr90" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  %"cdr ptr91" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  store { i32, ptr } %"insert value object89", ptr %"car ptr90", align 8
  store { i32, ptr } %"insert value object79", ptr %"cdr ptr91", align 8
  %"insert car - cons92" = insertvalue { ptr, ptr } zeroinitializer, ptr %"car ptr90", 0
  %"insert cdr - cons93" = insertvalue { ptr, ptr } %"insert car - cons92", ptr %"cdr ptr91", 1
  %"object value94" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ ptr, ptr }, ptr null, i32 1) to i32))
  store { ptr, ptr } %"insert cdr - cons93", ptr %"object value94", align 8
  %"insert value object95" = insertvalue { i32, ptr } { i32 6, ptr null }, ptr %"object value94", 1
  %"car ptr96" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  %"cdr ptr97" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  store { i32, ptr } %"insert value object87", ptr %"car ptr96", align 8
  store { i32, ptr } %"insert value object85", ptr %"cdr ptr97", align 8
  %"insert car - cons98" = insertvalue { ptr, ptr } zeroinitializer, ptr %"car ptr96", 0
  %"insert cdr - cons99" = insertvalue { ptr, ptr } %"insert car - cons98", ptr %"cdr ptr97", 1
  %"object value100" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ ptr, ptr }, ptr null, i32 1) to i32))
  store { ptr, ptr } %"insert cdr - cons99", ptr %"object value100", align 8
  %"insert value object101" = insertvalue { i32, ptr } { i32 6, ptr null }, ptr %"object value100", 1
  %"object value102" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr (ptr, ptr null, i32 1) to i32))
  store ptr @cons, ptr %"object value102", align 8
  %"insert value object103" = insertvalue { i32, ptr } { i32 7, ptr null }, ptr %"object value102", 1
  %"object value104" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  store { i32, ptr } { i32 4, ptr @cons.170 }, ptr %"object value104", align 8
  %"insert value object105" = insertvalue { i32, ptr } { i32 4, ptr null }, ptr %"object value104", 1
  %"car ptr106" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  %"cdr ptr107" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  store { i32, ptr } %"insert value object105", ptr %"car ptr106", align 8
  store { i32, ptr } %"insert value object95", ptr %"cdr ptr107", align 8
  %"insert car - cons108" = insertvalue { ptr, ptr } zeroinitializer, ptr %"car ptr106", 0
  %"insert cdr - cons109" = insertvalue { ptr, ptr } %"insert car - cons108", ptr %"cdr ptr107", 1
  %"object value110" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ ptr, ptr }, ptr null, i32 1) to i32))
  store { ptr, ptr } %"insert cdr - cons109", ptr %"object value110", align 8
  %"insert value object111" = insertvalue { i32, ptr } { i32 6, ptr null }, ptr %"object value110", 1
  %"car ptr112" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  %"cdr ptr113" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  store { i32, ptr } %"insert value object103", ptr %"car ptr112", align 8
  store { i32, ptr } %"insert value object101", ptr %"cdr ptr113", align 8
  %"insert car - cons114" = insertvalue { ptr, ptr } zeroinitializer, ptr %"car ptr112", 0
  %"insert cdr - cons115" = insertvalue { ptr, ptr } %"insert car - cons114", ptr %"cdr ptr113", 1
  %"object value116" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ ptr, ptr }, ptr null, i32 1) to i32))
  store { ptr, ptr } %"insert cdr - cons115", ptr %"object value116", align 8
  %"insert value object117" = insertvalue { i32, ptr } { i32 6, ptr null }, ptr %"object value116", 1
  %"object value118" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr (ptr, ptr null, i32 1) to i32))
  store ptr @-1, ptr %"object value118", align 8
  %"insert value object119" = insertvalue { i32, ptr } { i32 7, ptr null }, ptr %"object value118", 1
  %"object value120" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  store { i32, ptr } { i32 2, ptr @-1.171 }, ptr %"object value120", align 8
  %"insert value object121" = insertvalue { i32, ptr } { i32 4, ptr null }, ptr %"object value120", 1
  %"car ptr122" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  %"cdr ptr123" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  store { i32, ptr } %"insert value object121", ptr %"car ptr122", align 8
  store { i32, ptr } %"insert value object111", ptr %"cdr ptr123", align 8
  %"insert car - cons124" = insertvalue { ptr, ptr } zeroinitializer, ptr %"car ptr122", 0
  %"insert cdr - cons125" = insertvalue { ptr, ptr } %"insert car - cons124", ptr %"cdr ptr123", 1
  %"object value126" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ ptr, ptr }, ptr null, i32 1) to i32))
  store { ptr, ptr } %"insert cdr - cons125", ptr %"object value126", align 8
  %"insert value object127" = insertvalue { i32, ptr } { i32 6, ptr null }, ptr %"object value126", 1
  %"car ptr128" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  %"cdr ptr129" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  store { i32, ptr } %"insert value object119", ptr %"car ptr128", align 8
  store { i32, ptr } %"insert value object117", ptr %"cdr ptr129", align 8
  %"insert car - cons130" = insertvalue { ptr, ptr } zeroinitializer, ptr %"car ptr128", 0
  %"insert cdr - cons131" = insertvalue { ptr, ptr } %"insert car - cons130", ptr %"cdr ptr129", 1
  %"object value132" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ ptr, ptr }, ptr null, i32 1) to i32))
  store { ptr, ptr } %"insert cdr - cons131", ptr %"object value132", align 8
  %"insert value object133" = insertvalue { i32, ptr } { i32 6, ptr null }, ptr %"object value132", 1
  %"object value134" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr (ptr, ptr null, i32 1) to i32))
  store ptr @car, ptr %"object value134", align 8
  %"insert value object135" = insertvalue { i32, ptr } { i32 7, ptr null }, ptr %"object value134", 1
  %"object value136" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  store { i32, ptr } { i32 3, ptr @car.172 }, ptr %"object value136", align 8
  %"insert value object137" = insertvalue { i32, ptr } { i32 4, ptr null }, ptr %"object value136", 1
  %"car ptr138" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  %"cdr ptr139" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  store { i32, ptr } %"insert value object137", ptr %"car ptr138", align 8
  store { i32, ptr } %"insert value object127", ptr %"cdr ptr139", align 8
  %"insert car - cons140" = insertvalue { ptr, ptr } zeroinitializer, ptr %"car ptr138", 0
  %"insert cdr - cons141" = insertvalue { ptr, ptr } %"insert car - cons140", ptr %"cdr ptr139", 1
  %"object value142" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ ptr, ptr }, ptr null, i32 1) to i32))
  store { ptr, ptr } %"insert cdr - cons141", ptr %"object value142", align 8
  %"insert value object143" = insertvalue { i32, ptr } { i32 6, ptr null }, ptr %"object value142", 1
  %"car ptr144" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  %"cdr ptr145" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  store { i32, ptr } %"insert value object135", ptr %"car ptr144", align 8
  store { i32, ptr } %"insert value object133", ptr %"cdr ptr145", align 8
  %"insert car - cons146" = insertvalue { ptr, ptr } zeroinitializer, ptr %"car ptr144", 0
  %"insert cdr - cons147" = insertvalue { ptr, ptr } %"insert car - cons146", ptr %"cdr ptr145", 1
  %"object value148" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ ptr, ptr }, ptr null, i32 1) to i32))
  store { ptr, ptr } %"insert cdr - cons147", ptr %"object value148", align 8
  %"insert value object149" = insertvalue { i32, ptr } { i32 6, ptr null }, ptr %"object value148", 1
  %"object value150" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr (ptr, ptr null, i32 1) to i32))
  store ptr @cdr, ptr %"object value150", align 8
  %"insert value object151" = insertvalue { i32, ptr } { i32 7, ptr null }, ptr %"object value150", 1
  %"object value152" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  store { i32, ptr } { i32 3, ptr @cdr.173 }, ptr %"object value152", align 8
  %"insert value object153" = insertvalue { i32, ptr } { i32 4, ptr null }, ptr %"object value152", 1
  %"car ptr154" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  %"cdr ptr155" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  store { i32, ptr } %"insert value object153", ptr %"car ptr154", align 8
  store { i32, ptr } %"insert value object143", ptr %"cdr ptr155", align 8
  %"insert car - cons156" = insertvalue { ptr, ptr } zeroinitializer, ptr %"car ptr154", 0
  %"insert cdr - cons157" = insertvalue { ptr, ptr } %"insert car - cons156", ptr %"cdr ptr155", 1
  %"object value158" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ ptr, ptr }, ptr null, i32 1) to i32))
  store { ptr, ptr } %"insert cdr - cons157", ptr %"object value158", align 8
  %"insert value object159" = insertvalue { i32, ptr } { i32 6, ptr null }, ptr %"object value158", 1
  %"car ptr160" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  %"cdr ptr161" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  store { i32, ptr } %"insert value object151", ptr %"car ptr160", align 8
  store { i32, ptr } %"insert value object149", ptr %"cdr ptr161", align 8
  %"insert car - cons162" = insertvalue { ptr, ptr } zeroinitializer, ptr %"car ptr160", 0
  %"insert cdr - cons163" = insertvalue { ptr, ptr } %"insert car - cons162", ptr %"cdr ptr161", 1
  %"object value164" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ ptr, ptr }, ptr null, i32 1) to i32))
  store { ptr, ptr } %"insert cdr - cons163", ptr %"object value164", align 8
  %"insert value object165" = insertvalue { i32, ptr } { i32 6, ptr null }, ptr %"object value164", 1
  %"object value166" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr (ptr, ptr null, i32 1) to i32))
  store ptr @caar, ptr %"object value166", align 8
  %"insert value object167" = insertvalue { i32, ptr } { i32 7, ptr null }, ptr %"object value166", 1
  %"object value168" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  store { i32, ptr } { i32 4, ptr @caar.174 }, ptr %"object value168", align 8
  %"insert value object169" = insertvalue { i32, ptr } { i32 4, ptr null }, ptr %"object value168", 1
  %"car ptr170" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  %"cdr ptr171" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  store { i32, ptr } %"insert value object169", ptr %"car ptr170", align 8
  store { i32, ptr } %"insert value object159", ptr %"cdr ptr171", align 8
  %"insert car - cons172" = insertvalue { ptr, ptr } zeroinitializer, ptr %"car ptr170", 0
  %"insert cdr - cons173" = insertvalue { ptr, ptr } %"insert car - cons172", ptr %"cdr ptr171", 1
  %"object value174" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ ptr, ptr }, ptr null, i32 1) to i32))
  store { ptr, ptr } %"insert cdr - cons173", ptr %"object value174", align 8
  %"insert value object175" = insertvalue { i32, ptr } { i32 6, ptr null }, ptr %"object value174", 1
  %"car ptr176" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  %"cdr ptr177" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  store { i32, ptr } %"insert value object167", ptr %"car ptr176", align 8
  store { i32, ptr } %"insert value object165", ptr %"cdr ptr177", align 8
  %"insert car - cons178" = insertvalue { ptr, ptr } zeroinitializer, ptr %"car ptr176", 0
  %"insert cdr - cons179" = insertvalue { ptr, ptr } %"insert car - cons178", ptr %"cdr ptr177", 1
  %"object value180" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ ptr, ptr }, ptr null, i32 1) to i32))
  store { ptr, ptr } %"insert cdr - cons179", ptr %"object value180", align 8
  %"insert value object181" = insertvalue { i32, ptr } { i32 6, ptr null }, ptr %"object value180", 1
  %"object value182" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr (ptr, ptr null, i32 1) to i32))
  store ptr @cadr, ptr %"object value182", align 8
  %"insert value object183" = insertvalue { i32, ptr } { i32 7, ptr null }, ptr %"object value182", 1
  %"object value184" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  store { i32, ptr } { i32 4, ptr @cadr.175 }, ptr %"object value184", align 8
  %"insert value object185" = insertvalue { i32, ptr } { i32 4, ptr null }, ptr %"object value184", 1
  %"car ptr186" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  %"cdr ptr187" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  store { i32, ptr } %"insert value object185", ptr %"car ptr186", align 8
  store { i32, ptr } %"insert value object175", ptr %"cdr ptr187", align 8
  %"insert car - cons188" = insertvalue { ptr, ptr } zeroinitializer, ptr %"car ptr186", 0
  %"insert cdr - cons189" = insertvalue { ptr, ptr } %"insert car - cons188", ptr %"cdr ptr187", 1
  %"object value190" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ ptr, ptr }, ptr null, i32 1) to i32))
  store { ptr, ptr } %"insert cdr - cons189", ptr %"object value190", align 8
  %"insert value object191" = insertvalue { i32, ptr } { i32 6, ptr null }, ptr %"object value190", 1
  %"car ptr192" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  %"cdr ptr193" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  store { i32, ptr } %"insert value object183", ptr %"car ptr192", align 8
  store { i32, ptr } %"insert value object181", ptr %"cdr ptr193", align 8
  %"insert car - cons194" = insertvalue { ptr, ptr } zeroinitializer, ptr %"car ptr192", 0
  %"insert cdr - cons195" = insertvalue { ptr, ptr } %"insert car - cons194", ptr %"cdr ptr193", 1
  %"object value196" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ ptr, ptr }, ptr null, i32 1) to i32))
  store { ptr, ptr } %"insert cdr - cons195", ptr %"object value196", align 8
  %"insert value object197" = insertvalue { i32, ptr } { i32 6, ptr null }, ptr %"object value196", 1
  %"object value198" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr (ptr, ptr null, i32 1) to i32))
  store ptr @cdar, ptr %"object value198", align 8
  %"insert value object199" = insertvalue { i32, ptr } { i32 7, ptr null }, ptr %"object value198", 1
  %"object value200" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  store { i32, ptr } { i32 4, ptr @cdar.176 }, ptr %"object value200", align 8
  %"insert value object201" = insertvalue { i32, ptr } { i32 4, ptr null }, ptr %"object value200", 1
  %"car ptr202" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  %"cdr ptr203" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  store { i32, ptr } %"insert value object201", ptr %"car ptr202", align 8
  store { i32, ptr } %"insert value object191", ptr %"cdr ptr203", align 8
  %"insert car - cons204" = insertvalue { ptr, ptr } zeroinitializer, ptr %"car ptr202", 0
  %"insert cdr - cons205" = insertvalue { ptr, ptr } %"insert car - cons204", ptr %"cdr ptr203", 1
  %"object value206" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ ptr, ptr }, ptr null, i32 1) to i32))
  store { ptr, ptr } %"insert cdr - cons205", ptr %"object value206", align 8
  %"insert value object207" = insertvalue { i32, ptr } { i32 6, ptr null }, ptr %"object value206", 1
  %"car ptr208" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  %"cdr ptr209" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  store { i32, ptr } %"insert value object199", ptr %"car ptr208", align 8
  store { i32, ptr } %"insert value object197", ptr %"cdr ptr209", align 8
  %"insert car - cons210" = insertvalue { ptr, ptr } zeroinitializer, ptr %"car ptr208", 0
  %"insert cdr - cons211" = insertvalue { ptr, ptr } %"insert car - cons210", ptr %"cdr ptr209", 1
  %"object value212" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ ptr, ptr }, ptr null, i32 1) to i32))
  store { ptr, ptr } %"insert cdr - cons211", ptr %"object value212", align 8
  %"insert value object213" = insertvalue { i32, ptr } { i32 6, ptr null }, ptr %"object value212", 1
  %"object value214" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr (ptr, ptr null, i32 1) to i32))
  store ptr @cddr, ptr %"object value214", align 8
  %"insert value object215" = insertvalue { i32, ptr } { i32 7, ptr null }, ptr %"object value214", 1
  %"object value216" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  store { i32, ptr } { i32 4, ptr @cddr.177 }, ptr %"object value216", align 8
  %"insert value object217" = insertvalue { i32, ptr } { i32 4, ptr null }, ptr %"object value216", 1
  %"car ptr218" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  %"cdr ptr219" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  store { i32, ptr } %"insert value object217", ptr %"car ptr218", align 8
  store { i32, ptr } %"insert value object207", ptr %"cdr ptr219", align 8
  %"insert car - cons220" = insertvalue { ptr, ptr } zeroinitializer, ptr %"car ptr218", 0
  %"insert cdr - cons221" = insertvalue { ptr, ptr } %"insert car - cons220", ptr %"cdr ptr219", 1
  %"object value222" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ ptr, ptr }, ptr null, i32 1) to i32))
  store { ptr, ptr } %"insert cdr - cons221", ptr %"object value222", align 8
  %"insert value object223" = insertvalue { i32, ptr } { i32 6, ptr null }, ptr %"object value222", 1
  %"car ptr224" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  %"cdr ptr225" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  store { i32, ptr } %"insert value object215", ptr %"car ptr224", align 8
  store { i32, ptr } %"insert value object213", ptr %"cdr ptr225", align 8
  %"insert car - cons226" = insertvalue { ptr, ptr } zeroinitializer, ptr %"car ptr224", 0
  %"insert cdr - cons227" = insertvalue { ptr, ptr } %"insert car - cons226", ptr %"cdr ptr225", 1
  %"object value228" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ ptr, ptr }, ptr null, i32 1) to i32))
  store { ptr, ptr } %"insert cdr - cons227", ptr %"object value228", align 8
  %"insert value object229" = insertvalue { i32, ptr } { i32 6, ptr null }, ptr %"object value228", 1
  %"object value230" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr (ptr, ptr null, i32 1) to i32))
  store ptr @caaar, ptr %"object value230", align 8
  %"insert value object231" = insertvalue { i32, ptr } { i32 7, ptr null }, ptr %"object value230", 1
  %"object value232" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  store { i32, ptr } { i32 5, ptr @caaar.178 }, ptr %"object value232", align 8
  %"insert value object233" = insertvalue { i32, ptr } { i32 4, ptr null }, ptr %"object value232", 1
  %"car ptr234" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  %"cdr ptr235" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  store { i32, ptr } %"insert value object233", ptr %"car ptr234", align 8
  store { i32, ptr } %"insert value object223", ptr %"cdr ptr235", align 8
  %"insert car - cons236" = insertvalue { ptr, ptr } zeroinitializer, ptr %"car ptr234", 0
  %"insert cdr - cons237" = insertvalue { ptr, ptr } %"insert car - cons236", ptr %"cdr ptr235", 1
  %"object value238" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ ptr, ptr }, ptr null, i32 1) to i32))
  store { ptr, ptr } %"insert cdr - cons237", ptr %"object value238", align 8
  %"insert value object239" = insertvalue { i32, ptr } { i32 6, ptr null }, ptr %"object value238", 1
  %"car ptr240" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  %"cdr ptr241" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  store { i32, ptr } %"insert value object231", ptr %"car ptr240", align 8
  store { i32, ptr } %"insert value object229", ptr %"cdr ptr241", align 8
  %"insert car - cons242" = insertvalue { ptr, ptr } zeroinitializer, ptr %"car ptr240", 0
  %"insert cdr - cons243" = insertvalue { ptr, ptr } %"insert car - cons242", ptr %"cdr ptr241", 1
  %"object value244" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ ptr, ptr }, ptr null, i32 1) to i32))
  store { ptr, ptr } %"insert cdr - cons243", ptr %"object value244", align 8
  %"insert value object245" = insertvalue { i32, ptr } { i32 6, ptr null }, ptr %"object value244", 1
  %"object value246" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr (ptr, ptr null, i32 1) to i32))
  store ptr @caadr, ptr %"object value246", align 8
  %"insert value object247" = insertvalue { i32, ptr } { i32 7, ptr null }, ptr %"object value246", 1
  %"object value248" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  store { i32, ptr } { i32 5, ptr @caadr.179 }, ptr %"object value248", align 8
  %"insert value object249" = insertvalue { i32, ptr } { i32 4, ptr null }, ptr %"object value248", 1
  %"car ptr250" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  %"cdr ptr251" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  store { i32, ptr } %"insert value object249", ptr %"car ptr250", align 8
  store { i32, ptr } %"insert value object239", ptr %"cdr ptr251", align 8
  %"insert car - cons252" = insertvalue { ptr, ptr } zeroinitializer, ptr %"car ptr250", 0
  %"insert cdr - cons253" = insertvalue { ptr, ptr } %"insert car - cons252", ptr %"cdr ptr251", 1
  %"object value254" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ ptr, ptr }, ptr null, i32 1) to i32))
  store { ptr, ptr } %"insert cdr - cons253", ptr %"object value254", align 8
  %"insert value object255" = insertvalue { i32, ptr } { i32 6, ptr null }, ptr %"object value254", 1
  %"car ptr256" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  %"cdr ptr257" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  store { i32, ptr } %"insert value object247", ptr %"car ptr256", align 8
  store { i32, ptr } %"insert value object245", ptr %"cdr ptr257", align 8
  %"insert car - cons258" = insertvalue { ptr, ptr } zeroinitializer, ptr %"car ptr256", 0
  %"insert cdr - cons259" = insertvalue { ptr, ptr } %"insert car - cons258", ptr %"cdr ptr257", 1
  %"object value260" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ ptr, ptr }, ptr null, i32 1) to i32))
  store { ptr, ptr } %"insert cdr - cons259", ptr %"object value260", align 8
  %"insert value object261" = insertvalue { i32, ptr } { i32 6, ptr null }, ptr %"object value260", 1
  %"object value262" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr (ptr, ptr null, i32 1) to i32))
  store ptr @cadar, ptr %"object value262", align 8
  %"insert value object263" = insertvalue { i32, ptr } { i32 7, ptr null }, ptr %"object value262", 1
  %"object value264" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  store { i32, ptr } { i32 5, ptr @cadar.180 }, ptr %"object value264", align 8
  %"insert value object265" = insertvalue { i32, ptr } { i32 4, ptr null }, ptr %"object value264", 1
  %"car ptr266" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  %"cdr ptr267" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  store { i32, ptr } %"insert value object265", ptr %"car ptr266", align 8
  store { i32, ptr } %"insert value object255", ptr %"cdr ptr267", align 8
  %"insert car - cons268" = insertvalue { ptr, ptr } zeroinitializer, ptr %"car ptr266", 0
  %"insert cdr - cons269" = insertvalue { ptr, ptr } %"insert car - cons268", ptr %"cdr ptr267", 1
  %"object value270" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ ptr, ptr }, ptr null, i32 1) to i32))
  store { ptr, ptr } %"insert cdr - cons269", ptr %"object value270", align 8
  %"insert value object271" = insertvalue { i32, ptr } { i32 6, ptr null }, ptr %"object value270", 1
  %"car ptr272" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  %"cdr ptr273" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  store { i32, ptr } %"insert value object263", ptr %"car ptr272", align 8
  store { i32, ptr } %"insert value object261", ptr %"cdr ptr273", align 8
  %"insert car - cons274" = insertvalue { ptr, ptr } zeroinitializer, ptr %"car ptr272", 0
  %"insert cdr - cons275" = insertvalue { ptr, ptr } %"insert car - cons274", ptr %"cdr ptr273", 1
  %"object value276" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ ptr, ptr }, ptr null, i32 1) to i32))
  store { ptr, ptr } %"insert cdr - cons275", ptr %"object value276", align 8
  %"insert value object277" = insertvalue { i32, ptr } { i32 6, ptr null }, ptr %"object value276", 1
  %"object value278" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr (ptr, ptr null, i32 1) to i32))
  store ptr @caddr, ptr %"object value278", align 8
  %"insert value object279" = insertvalue { i32, ptr } { i32 7, ptr null }, ptr %"object value278", 1
  %"object value280" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  store { i32, ptr } { i32 5, ptr @caddr.181 }, ptr %"object value280", align 8
  %"insert value object281" = insertvalue { i32, ptr } { i32 4, ptr null }, ptr %"object value280", 1
  %"car ptr282" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  %"cdr ptr283" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  store { i32, ptr } %"insert value object281", ptr %"car ptr282", align 8
  store { i32, ptr } %"insert value object271", ptr %"cdr ptr283", align 8
  %"insert car - cons284" = insertvalue { ptr, ptr } zeroinitializer, ptr %"car ptr282", 0
  %"insert cdr - cons285" = insertvalue { ptr, ptr } %"insert car - cons284", ptr %"cdr ptr283", 1
  %"object value286" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ ptr, ptr }, ptr null, i32 1) to i32))
  store { ptr, ptr } %"insert cdr - cons285", ptr %"object value286", align 8
  %"insert value object287" = insertvalue { i32, ptr } { i32 6, ptr null }, ptr %"object value286", 1
  %"car ptr288" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  %"cdr ptr289" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  store { i32, ptr } %"insert value object279", ptr %"car ptr288", align 8
  store { i32, ptr } %"insert value object277", ptr %"cdr ptr289", align 8
  %"insert car - cons290" = insertvalue { ptr, ptr } zeroinitializer, ptr %"car ptr288", 0
  %"insert cdr - cons291" = insertvalue { ptr, ptr } %"insert car - cons290", ptr %"cdr ptr289", 1
  %"object value292" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ ptr, ptr }, ptr null, i32 1) to i32))
  store { ptr, ptr } %"insert cdr - cons291", ptr %"object value292", align 8
  %"insert value object293" = insertvalue { i32, ptr } { i32 6, ptr null }, ptr %"object value292", 1
  %"object value294" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr (ptr, ptr null, i32 1) to i32))
  store ptr @cdaar, ptr %"object value294", align 8
  %"insert value object295" = insertvalue { i32, ptr } { i32 7, ptr null }, ptr %"object value294", 1
  %"object value296" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  store { i32, ptr } { i32 5, ptr @cdaar.182 }, ptr %"object value296", align 8
  %"insert value object297" = insertvalue { i32, ptr } { i32 4, ptr null }, ptr %"object value296", 1
  %"car ptr298" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  %"cdr ptr299" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  store { i32, ptr } %"insert value object297", ptr %"car ptr298", align 8
  store { i32, ptr } %"insert value object287", ptr %"cdr ptr299", align 8
  %"insert car - cons300" = insertvalue { ptr, ptr } zeroinitializer, ptr %"car ptr298", 0
  %"insert cdr - cons301" = insertvalue { ptr, ptr } %"insert car - cons300", ptr %"cdr ptr299", 1
  %"object value302" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ ptr, ptr }, ptr null, i32 1) to i32))
  store { ptr, ptr } %"insert cdr - cons301", ptr %"object value302", align 8
  %"insert value object303" = insertvalue { i32, ptr } { i32 6, ptr null }, ptr %"object value302", 1
  %"car ptr304" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  %"cdr ptr305" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  store { i32, ptr } %"insert value object295", ptr %"car ptr304", align 8
  store { i32, ptr } %"insert value object293", ptr %"cdr ptr305", align 8
  %"insert car - cons306" = insertvalue { ptr, ptr } zeroinitializer, ptr %"car ptr304", 0
  %"insert cdr - cons307" = insertvalue { ptr, ptr } %"insert car - cons306", ptr %"cdr ptr305", 1
  %"object value308" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ ptr, ptr }, ptr null, i32 1) to i32))
  store { ptr, ptr } %"insert cdr - cons307", ptr %"object value308", align 8
  %"insert value object309" = insertvalue { i32, ptr } { i32 6, ptr null }, ptr %"object value308", 1
  %"object value310" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr (ptr, ptr null, i32 1) to i32))
  store ptr @cdadr, ptr %"object value310", align 8
  %"insert value object311" = insertvalue { i32, ptr } { i32 7, ptr null }, ptr %"object value310", 1
  %"object value312" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  store { i32, ptr } { i32 5, ptr @cdadr.183 }, ptr %"object value312", align 8
  %"insert value object313" = insertvalue { i32, ptr } { i32 4, ptr null }, ptr %"object value312", 1
  %"car ptr314" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  %"cdr ptr315" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  store { i32, ptr } %"insert value object313", ptr %"car ptr314", align 8
  store { i32, ptr } %"insert value object303", ptr %"cdr ptr315", align 8
  %"insert car - cons316" = insertvalue { ptr, ptr } zeroinitializer, ptr %"car ptr314", 0
  %"insert cdr - cons317" = insertvalue { ptr, ptr } %"insert car - cons316", ptr %"cdr ptr315", 1
  %"object value318" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ ptr, ptr }, ptr null, i32 1) to i32))
  store { ptr, ptr } %"insert cdr - cons317", ptr %"object value318", align 8
  %"insert value object319" = insertvalue { i32, ptr } { i32 6, ptr null }, ptr %"object value318", 1
  %"car ptr320" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  %"cdr ptr321" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  store { i32, ptr } %"insert value object311", ptr %"car ptr320", align 8
  store { i32, ptr } %"insert value object309", ptr %"cdr ptr321", align 8
  %"insert car - cons322" = insertvalue { ptr, ptr } zeroinitializer, ptr %"car ptr320", 0
  %"insert cdr - cons323" = insertvalue { ptr, ptr } %"insert car - cons322", ptr %"cdr ptr321", 1
  %"object value324" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ ptr, ptr }, ptr null, i32 1) to i32))
  store { ptr, ptr } %"insert cdr - cons323", ptr %"object value324", align 8
  %"insert value object325" = insertvalue { i32, ptr } { i32 6, ptr null }, ptr %"object value324", 1
  %"object value326" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr (ptr, ptr null, i32 1) to i32))
  store ptr @cddar, ptr %"object value326", align 8
  %"insert value object327" = insertvalue { i32, ptr } { i32 7, ptr null }, ptr %"object value326", 1
  %"object value328" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  store { i32, ptr } { i32 5, ptr @cddar.184 }, ptr %"object value328", align 8
  %"insert value object329" = insertvalue { i32, ptr } { i32 4, ptr null }, ptr %"object value328", 1
  %"car ptr330" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  %"cdr ptr331" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  store { i32, ptr } %"insert value object329", ptr %"car ptr330", align 8
  store { i32, ptr } %"insert value object319", ptr %"cdr ptr331", align 8
  %"insert car - cons332" = insertvalue { ptr, ptr } zeroinitializer, ptr %"car ptr330", 0
  %"insert cdr - cons333" = insertvalue { ptr, ptr } %"insert car - cons332", ptr %"cdr ptr331", 1
  %"object value334" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ ptr, ptr }, ptr null, i32 1) to i32))
  store { ptr, ptr } %"insert cdr - cons333", ptr %"object value334", align 8
  %"insert value object335" = insertvalue { i32, ptr } { i32 6, ptr null }, ptr %"object value334", 1
  %"car ptr336" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  %"cdr ptr337" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  store { i32, ptr } %"insert value object327", ptr %"car ptr336", align 8
  store { i32, ptr } %"insert value object325", ptr %"cdr ptr337", align 8
  %"insert car - cons338" = insertvalue { ptr, ptr } zeroinitializer, ptr %"car ptr336", 0
  %"insert cdr - cons339" = insertvalue { ptr, ptr } %"insert car - cons338", ptr %"cdr ptr337", 1
  %"object value340" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ ptr, ptr }, ptr null, i32 1) to i32))
  store { ptr, ptr } %"insert cdr - cons339", ptr %"object value340", align 8
  %"insert value object341" = insertvalue { i32, ptr } { i32 6, ptr null }, ptr %"object value340", 1
  %"object value342" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr (ptr, ptr null, i32 1) to i32))
  store ptr @cdddr, ptr %"object value342", align 8
  %"insert value object343" = insertvalue { i32, ptr } { i32 7, ptr null }, ptr %"object value342", 1
  %"object value344" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  store { i32, ptr } { i32 5, ptr @cdddr.185 }, ptr %"object value344", align 8
  %"insert value object345" = insertvalue { i32, ptr } { i32 4, ptr null }, ptr %"object value344", 1
  %"car ptr346" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  %"cdr ptr347" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  store { i32, ptr } %"insert value object345", ptr %"car ptr346", align 8
  store { i32, ptr } %"insert value object335", ptr %"cdr ptr347", align 8
  %"insert car - cons348" = insertvalue { ptr, ptr } zeroinitializer, ptr %"car ptr346", 0
  %"insert cdr - cons349" = insertvalue { ptr, ptr } %"insert car - cons348", ptr %"cdr ptr347", 1
  %"object value350" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ ptr, ptr }, ptr null, i32 1) to i32))
  store { ptr, ptr } %"insert cdr - cons349", ptr %"object value350", align 8
  %"insert value object351" = insertvalue { i32, ptr } { i32 6, ptr null }, ptr %"object value350", 1
  %"car ptr352" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  %"cdr ptr353" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  store { i32, ptr } %"insert value object343", ptr %"car ptr352", align 8
  store { i32, ptr } %"insert value object341", ptr %"cdr ptr353", align 8
  %"insert car - cons354" = insertvalue { ptr, ptr } zeroinitializer, ptr %"car ptr352", 0
  %"insert cdr - cons355" = insertvalue { ptr, ptr } %"insert car - cons354", ptr %"cdr ptr353", 1
  %"object value356" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ ptr, ptr }, ptr null, i32 1) to i32))
  store { ptr, ptr } %"insert cdr - cons355", ptr %"object value356", align 8
  %"insert value object357" = insertvalue { i32, ptr } { i32 6, ptr null }, ptr %"object value356", 1
  %"object value358" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr (ptr, ptr null, i32 1) to i32))
  store ptr @caaaar, ptr %"object value358", align 8
  %"insert value object359" = insertvalue { i32, ptr } { i32 7, ptr null }, ptr %"object value358", 1
  %"object value360" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  store { i32, ptr } { i32 6, ptr @caaaar.186 }, ptr %"object value360", align 8
  %"insert value object361" = insertvalue { i32, ptr } { i32 4, ptr null }, ptr %"object value360", 1
  %"car ptr362" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  %"cdr ptr363" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  store { i32, ptr } %"insert value object361", ptr %"car ptr362", align 8
  store { i32, ptr } %"insert value object351", ptr %"cdr ptr363", align 8
  %"insert car - cons364" = insertvalue { ptr, ptr } zeroinitializer, ptr %"car ptr362", 0
  %"insert cdr - cons365" = insertvalue { ptr, ptr } %"insert car - cons364", ptr %"cdr ptr363", 1
  %"object value366" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ ptr, ptr }, ptr null, i32 1) to i32))
  store { ptr, ptr } %"insert cdr - cons365", ptr %"object value366", align 8
  %"insert value object367" = insertvalue { i32, ptr } { i32 6, ptr null }, ptr %"object value366", 1
  %"car ptr368" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  %"cdr ptr369" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  store { i32, ptr } %"insert value object359", ptr %"car ptr368", align 8
  store { i32, ptr } %"insert value object357", ptr %"cdr ptr369", align 8
  %"insert car - cons370" = insertvalue { ptr, ptr } zeroinitializer, ptr %"car ptr368", 0
  %"insert cdr - cons371" = insertvalue { ptr, ptr } %"insert car - cons370", ptr %"cdr ptr369", 1
  %"object value372" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ ptr, ptr }, ptr null, i32 1) to i32))
  store { ptr, ptr } %"insert cdr - cons371", ptr %"object value372", align 8
  %"insert value object373" = insertvalue { i32, ptr } { i32 6, ptr null }, ptr %"object value372", 1
  %"object value374" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr (ptr, ptr null, i32 1) to i32))
  store ptr @caaadr, ptr %"object value374", align 8
  %"insert value object375" = insertvalue { i32, ptr } { i32 7, ptr null }, ptr %"object value374", 1
  %"object value376" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  store { i32, ptr } { i32 6, ptr @caaadr.187 }, ptr %"object value376", align 8
  %"insert value object377" = insertvalue { i32, ptr } { i32 4, ptr null }, ptr %"object value376", 1
  %"car ptr378" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  %"cdr ptr379" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  store { i32, ptr } %"insert value object377", ptr %"car ptr378", align 8
  store { i32, ptr } %"insert value object367", ptr %"cdr ptr379", align 8
  %"insert car - cons380" = insertvalue { ptr, ptr } zeroinitializer, ptr %"car ptr378", 0
  %"insert cdr - cons381" = insertvalue { ptr, ptr } %"insert car - cons380", ptr %"cdr ptr379", 1
  %"object value382" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ ptr, ptr }, ptr null, i32 1) to i32))
  store { ptr, ptr } %"insert cdr - cons381", ptr %"object value382", align 8
  %"insert value object383" = insertvalue { i32, ptr } { i32 6, ptr null }, ptr %"object value382", 1
  %"car ptr384" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  %"cdr ptr385" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  store { i32, ptr } %"insert value object375", ptr %"car ptr384", align 8
  store { i32, ptr } %"insert value object373", ptr %"cdr ptr385", align 8
  %"insert car - cons386" = insertvalue { ptr, ptr } zeroinitializer, ptr %"car ptr384", 0
  %"insert cdr - cons387" = insertvalue { ptr, ptr } %"insert car - cons386", ptr %"cdr ptr385", 1
  %"object value388" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ ptr, ptr }, ptr null, i32 1) to i32))
  store { ptr, ptr } %"insert cdr - cons387", ptr %"object value388", align 8
  %"insert value object389" = insertvalue { i32, ptr } { i32 6, ptr null }, ptr %"object value388", 1
  %"object value390" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr (ptr, ptr null, i32 1) to i32))
  store ptr @caadar, ptr %"object value390", align 8
  %"insert value object391" = insertvalue { i32, ptr } { i32 7, ptr null }, ptr %"object value390", 1
  %"object value392" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  store { i32, ptr } { i32 6, ptr @caadar.188 }, ptr %"object value392", align 8
  %"insert value object393" = insertvalue { i32, ptr } { i32 4, ptr null }, ptr %"object value392", 1
  %"car ptr394" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  %"cdr ptr395" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  store { i32, ptr } %"insert value object393", ptr %"car ptr394", align 8
  store { i32, ptr } %"insert value object383", ptr %"cdr ptr395", align 8
  %"insert car - cons396" = insertvalue { ptr, ptr } zeroinitializer, ptr %"car ptr394", 0
  %"insert cdr - cons397" = insertvalue { ptr, ptr } %"insert car - cons396", ptr %"cdr ptr395", 1
  %"object value398" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ ptr, ptr }, ptr null, i32 1) to i32))
  store { ptr, ptr } %"insert cdr - cons397", ptr %"object value398", align 8
  %"insert value object399" = insertvalue { i32, ptr } { i32 6, ptr null }, ptr %"object value398", 1
  %"car ptr400" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  %"cdr ptr401" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  store { i32, ptr } %"insert value object391", ptr %"car ptr400", align 8
  store { i32, ptr } %"insert value object389", ptr %"cdr ptr401", align 8
  %"insert car - cons402" = insertvalue { ptr, ptr } zeroinitializer, ptr %"car ptr400", 0
  %"insert cdr - cons403" = insertvalue { ptr, ptr } %"insert car - cons402", ptr %"cdr ptr401", 1
  %"object value404" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ ptr, ptr }, ptr null, i32 1) to i32))
  store { ptr, ptr } %"insert cdr - cons403", ptr %"object value404", align 8
  %"insert value object405" = insertvalue { i32, ptr } { i32 6, ptr null }, ptr %"object value404", 1
  %"object value406" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr (ptr, ptr null, i32 1) to i32))
  store ptr @caaddr, ptr %"object value406", align 8
  %"insert value object407" = insertvalue { i32, ptr } { i32 7, ptr null }, ptr %"object value406", 1
  %"object value408" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  store { i32, ptr } { i32 6, ptr @caaddr.189 }, ptr %"object value408", align 8
  %"insert value object409" = insertvalue { i32, ptr } { i32 4, ptr null }, ptr %"object value408", 1
  %"car ptr410" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  %"cdr ptr411" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  store { i32, ptr } %"insert value object409", ptr %"car ptr410", align 8
  store { i32, ptr } %"insert value object399", ptr %"cdr ptr411", align 8
  %"insert car - cons412" = insertvalue { ptr, ptr } zeroinitializer, ptr %"car ptr410", 0
  %"insert cdr - cons413" = insertvalue { ptr, ptr } %"insert car - cons412", ptr %"cdr ptr411", 1
  %"object value414" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ ptr, ptr }, ptr null, i32 1) to i32))
  store { ptr, ptr } %"insert cdr - cons413", ptr %"object value414", align 8
  %"insert value object415" = insertvalue { i32, ptr } { i32 6, ptr null }, ptr %"object value414", 1
  %"car ptr416" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  %"cdr ptr417" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  store { i32, ptr } %"insert value object407", ptr %"car ptr416", align 8
  store { i32, ptr } %"insert value object405", ptr %"cdr ptr417", align 8
  %"insert car - cons418" = insertvalue { ptr, ptr } zeroinitializer, ptr %"car ptr416", 0
  %"insert cdr - cons419" = insertvalue { ptr, ptr } %"insert car - cons418", ptr %"cdr ptr417", 1
  %"object value420" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ ptr, ptr }, ptr null, i32 1) to i32))
  store { ptr, ptr } %"insert cdr - cons419", ptr %"object value420", align 8
  %"insert value object421" = insertvalue { i32, ptr } { i32 6, ptr null }, ptr %"object value420", 1
  %"object value422" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr (ptr, ptr null, i32 1) to i32))
  store ptr @cadaar, ptr %"object value422", align 8
  %"insert value object423" = insertvalue { i32, ptr } { i32 7, ptr null }, ptr %"object value422", 1
  %"object value424" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  store { i32, ptr } { i32 6, ptr @cadaar.190 }, ptr %"object value424", align 8
  %"insert value object425" = insertvalue { i32, ptr } { i32 4, ptr null }, ptr %"object value424", 1
  %"car ptr426" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  %"cdr ptr427" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  store { i32, ptr } %"insert value object425", ptr %"car ptr426", align 8
  store { i32, ptr } %"insert value object415", ptr %"cdr ptr427", align 8
  %"insert car - cons428" = insertvalue { ptr, ptr } zeroinitializer, ptr %"car ptr426", 0
  %"insert cdr - cons429" = insertvalue { ptr, ptr } %"insert car - cons428", ptr %"cdr ptr427", 1
  %"object value430" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ ptr, ptr }, ptr null, i32 1) to i32))
  store { ptr, ptr } %"insert cdr - cons429", ptr %"object value430", align 8
  %"insert value object431" = insertvalue { i32, ptr } { i32 6, ptr null }, ptr %"object value430", 1
  %"car ptr432" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  %"cdr ptr433" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  store { i32, ptr } %"insert value object423", ptr %"car ptr432", align 8
  store { i32, ptr } %"insert value object421", ptr %"cdr ptr433", align 8
  %"insert car - cons434" = insertvalue { ptr, ptr } zeroinitializer, ptr %"car ptr432", 0
  %"insert cdr - cons435" = insertvalue { ptr, ptr } %"insert car - cons434", ptr %"cdr ptr433", 1
  %"object value436" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ ptr, ptr }, ptr null, i32 1) to i32))
  store { ptr, ptr } %"insert cdr - cons435", ptr %"object value436", align 8
  %"insert value object437" = insertvalue { i32, ptr } { i32 6, ptr null }, ptr %"object value436", 1
  %"object value438" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr (ptr, ptr null, i32 1) to i32))
  store ptr @cadadr, ptr %"object value438", align 8
  %"insert value object439" = insertvalue { i32, ptr } { i32 7, ptr null }, ptr %"object value438", 1
  %"object value440" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  store { i32, ptr } { i32 6, ptr @cadadr.191 }, ptr %"object value440", align 8
  %"insert value object441" = insertvalue { i32, ptr } { i32 4, ptr null }, ptr %"object value440", 1
  %"car ptr442" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  %"cdr ptr443" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  store { i32, ptr } %"insert value object441", ptr %"car ptr442", align 8
  store { i32, ptr } %"insert value object431", ptr %"cdr ptr443", align 8
  %"insert car - cons444" = insertvalue { ptr, ptr } zeroinitializer, ptr %"car ptr442", 0
  %"insert cdr - cons445" = insertvalue { ptr, ptr } %"insert car - cons444", ptr %"cdr ptr443", 1
  %"object value446" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ ptr, ptr }, ptr null, i32 1) to i32))
  store { ptr, ptr } %"insert cdr - cons445", ptr %"object value446", align 8
  %"insert value object447" = insertvalue { i32, ptr } { i32 6, ptr null }, ptr %"object value446", 1
  %"car ptr448" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  %"cdr ptr449" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  store { i32, ptr } %"insert value object439", ptr %"car ptr448", align 8
  store { i32, ptr } %"insert value object437", ptr %"cdr ptr449", align 8
  %"insert car - cons450" = insertvalue { ptr, ptr } zeroinitializer, ptr %"car ptr448", 0
  %"insert cdr - cons451" = insertvalue { ptr, ptr } %"insert car - cons450", ptr %"cdr ptr449", 1
  %"object value452" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ ptr, ptr }, ptr null, i32 1) to i32))
  store { ptr, ptr } %"insert cdr - cons451", ptr %"object value452", align 8
  %"insert value object453" = insertvalue { i32, ptr } { i32 6, ptr null }, ptr %"object value452", 1
  %"object value454" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr (ptr, ptr null, i32 1) to i32))
  store ptr @caddar, ptr %"object value454", align 8
  %"insert value object455" = insertvalue { i32, ptr } { i32 7, ptr null }, ptr %"object value454", 1
  %"object value456" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  store { i32, ptr } { i32 6, ptr @caddar.192 }, ptr %"object value456", align 8
  %"insert value object457" = insertvalue { i32, ptr } { i32 4, ptr null }, ptr %"object value456", 1
  %"car ptr458" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  %"cdr ptr459" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  store { i32, ptr } %"insert value object457", ptr %"car ptr458", align 8
  store { i32, ptr } %"insert value object447", ptr %"cdr ptr459", align 8
  %"insert car - cons460" = insertvalue { ptr, ptr } zeroinitializer, ptr %"car ptr458", 0
  %"insert cdr - cons461" = insertvalue { ptr, ptr } %"insert car - cons460", ptr %"cdr ptr459", 1
  %"object value462" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ ptr, ptr }, ptr null, i32 1) to i32))
  store { ptr, ptr } %"insert cdr - cons461", ptr %"object value462", align 8
  %"insert value object463" = insertvalue { i32, ptr } { i32 6, ptr null }, ptr %"object value462", 1
  %"car ptr464" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  %"cdr ptr465" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  store { i32, ptr } %"insert value object455", ptr %"car ptr464", align 8
  store { i32, ptr } %"insert value object453", ptr %"cdr ptr465", align 8
  %"insert car - cons466" = insertvalue { ptr, ptr } zeroinitializer, ptr %"car ptr464", 0
  %"insert cdr - cons467" = insertvalue { ptr, ptr } %"insert car - cons466", ptr %"cdr ptr465", 1
  %"object value468" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ ptr, ptr }, ptr null, i32 1) to i32))
  store { ptr, ptr } %"insert cdr - cons467", ptr %"object value468", align 8
  %"insert value object469" = insertvalue { i32, ptr } { i32 6, ptr null }, ptr %"object value468", 1
  %"object value470" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr (ptr, ptr null, i32 1) to i32))
  store ptr @cadddr, ptr %"object value470", align 8
  %"insert value object471" = insertvalue { i32, ptr } { i32 7, ptr null }, ptr %"object value470", 1
  %"object value472" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  store { i32, ptr } { i32 6, ptr @cadddr.193 }, ptr %"object value472", align 8
  %"insert value object473" = insertvalue { i32, ptr } { i32 4, ptr null }, ptr %"object value472", 1
  %"car ptr474" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  %"cdr ptr475" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  store { i32, ptr } %"insert value object473", ptr %"car ptr474", align 8
  store { i32, ptr } %"insert value object463", ptr %"cdr ptr475", align 8
  %"insert car - cons476" = insertvalue { ptr, ptr } zeroinitializer, ptr %"car ptr474", 0
  %"insert cdr - cons477" = insertvalue { ptr, ptr } %"insert car - cons476", ptr %"cdr ptr475", 1
  %"object value478" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ ptr, ptr }, ptr null, i32 1) to i32))
  store { ptr, ptr } %"insert cdr - cons477", ptr %"object value478", align 8
  %"insert value object479" = insertvalue { i32, ptr } { i32 6, ptr null }, ptr %"object value478", 1
  %"car ptr480" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  %"cdr ptr481" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  store { i32, ptr } %"insert value object471", ptr %"car ptr480", align 8
  store { i32, ptr } %"insert value object469", ptr %"cdr ptr481", align 8
  %"insert car - cons482" = insertvalue { ptr, ptr } zeroinitializer, ptr %"car ptr480", 0
  %"insert cdr - cons483" = insertvalue { ptr, ptr } %"insert car - cons482", ptr %"cdr ptr481", 1
  %"object value484" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ ptr, ptr }, ptr null, i32 1) to i32))
  store { ptr, ptr } %"insert cdr - cons483", ptr %"object value484", align 8
  %"insert value object485" = insertvalue { i32, ptr } { i32 6, ptr null }, ptr %"object value484", 1
  %"object value486" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr (ptr, ptr null, i32 1) to i32))
  store ptr @cdaaar, ptr %"object value486", align 8
  %"insert value object487" = insertvalue { i32, ptr } { i32 7, ptr null }, ptr %"object value486", 1
  %"object value488" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  store { i32, ptr } { i32 6, ptr @cdaaar.194 }, ptr %"object value488", align 8
  %"insert value object489" = insertvalue { i32, ptr } { i32 4, ptr null }, ptr %"object value488", 1
  %"car ptr490" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  %"cdr ptr491" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  store { i32, ptr } %"insert value object489", ptr %"car ptr490", align 8
  store { i32, ptr } %"insert value object479", ptr %"cdr ptr491", align 8
  %"insert car - cons492" = insertvalue { ptr, ptr } zeroinitializer, ptr %"car ptr490", 0
  %"insert cdr - cons493" = insertvalue { ptr, ptr } %"insert car - cons492", ptr %"cdr ptr491", 1
  %"object value494" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ ptr, ptr }, ptr null, i32 1) to i32))
  store { ptr, ptr } %"insert cdr - cons493", ptr %"object value494", align 8
  %"insert value object495" = insertvalue { i32, ptr } { i32 6, ptr null }, ptr %"object value494", 1
  %"car ptr496" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  %"cdr ptr497" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  store { i32, ptr } %"insert value object487", ptr %"car ptr496", align 8
  store { i32, ptr } %"insert value object485", ptr %"cdr ptr497", align 8
  %"insert car - cons498" = insertvalue { ptr, ptr } zeroinitializer, ptr %"car ptr496", 0
  %"insert cdr - cons499" = insertvalue { ptr, ptr } %"insert car - cons498", ptr %"cdr ptr497", 1
  %"object value500" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ ptr, ptr }, ptr null, i32 1) to i32))
  store { ptr, ptr } %"insert cdr - cons499", ptr %"object value500", align 8
  %"insert value object501" = insertvalue { i32, ptr } { i32 6, ptr null }, ptr %"object value500", 1
  %"object value502" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr (ptr, ptr null, i32 1) to i32))
  store ptr @cdaadr, ptr %"object value502", align 8
  %"insert value object503" = insertvalue { i32, ptr } { i32 7, ptr null }, ptr %"object value502", 1
  %"object value504" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  store { i32, ptr } { i32 6, ptr @cdaadr.195 }, ptr %"object value504", align 8
  %"insert value object505" = insertvalue { i32, ptr } { i32 4, ptr null }, ptr %"object value504", 1
  %"car ptr506" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  %"cdr ptr507" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  store { i32, ptr } %"insert value object505", ptr %"car ptr506", align 8
  store { i32, ptr } %"insert value object495", ptr %"cdr ptr507", align 8
  %"insert car - cons508" = insertvalue { ptr, ptr } zeroinitializer, ptr %"car ptr506", 0
  %"insert cdr - cons509" = insertvalue { ptr, ptr } %"insert car - cons508", ptr %"cdr ptr507", 1
  %"object value510" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ ptr, ptr }, ptr null, i32 1) to i32))
  store { ptr, ptr } %"insert cdr - cons509", ptr %"object value510", align 8
  %"insert value object511" = insertvalue { i32, ptr } { i32 6, ptr null }, ptr %"object value510", 1
  %"car ptr512" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  %"cdr ptr513" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  store { i32, ptr } %"insert value object503", ptr %"car ptr512", align 8
  store { i32, ptr } %"insert value object501", ptr %"cdr ptr513", align 8
  %"insert car - cons514" = insertvalue { ptr, ptr } zeroinitializer, ptr %"car ptr512", 0
  %"insert cdr - cons515" = insertvalue { ptr, ptr } %"insert car - cons514", ptr %"cdr ptr513", 1
  %"object value516" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ ptr, ptr }, ptr null, i32 1) to i32))
  store { ptr, ptr } %"insert cdr - cons515", ptr %"object value516", align 8
  %"insert value object517" = insertvalue { i32, ptr } { i32 6, ptr null }, ptr %"object value516", 1
  %"object value518" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr (ptr, ptr null, i32 1) to i32))
  store ptr @cdadar, ptr %"object value518", align 8
  %"insert value object519" = insertvalue { i32, ptr } { i32 7, ptr null }, ptr %"object value518", 1
  %"object value520" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  store { i32, ptr } { i32 6, ptr @cdadar.196 }, ptr %"object value520", align 8
  %"insert value object521" = insertvalue { i32, ptr } { i32 4, ptr null }, ptr %"object value520", 1
  %"car ptr522" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  %"cdr ptr523" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  store { i32, ptr } %"insert value object521", ptr %"car ptr522", align 8
  store { i32, ptr } %"insert value object511", ptr %"cdr ptr523", align 8
  %"insert car - cons524" = insertvalue { ptr, ptr } zeroinitializer, ptr %"car ptr522", 0
  %"insert cdr - cons525" = insertvalue { ptr, ptr } %"insert car - cons524", ptr %"cdr ptr523", 1
  %"object value526" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ ptr, ptr }, ptr null, i32 1) to i32))
  store { ptr, ptr } %"insert cdr - cons525", ptr %"object value526", align 8
  %"insert value object527" = insertvalue { i32, ptr } { i32 6, ptr null }, ptr %"object value526", 1
  %"car ptr528" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  %"cdr ptr529" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  store { i32, ptr } %"insert value object519", ptr %"car ptr528", align 8
  store { i32, ptr } %"insert value object517", ptr %"cdr ptr529", align 8
  %"insert car - cons530" = insertvalue { ptr, ptr } zeroinitializer, ptr %"car ptr528", 0
  %"insert cdr - cons531" = insertvalue { ptr, ptr } %"insert car - cons530", ptr %"cdr ptr529", 1
  %"object value532" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ ptr, ptr }, ptr null, i32 1) to i32))
  store { ptr, ptr } %"insert cdr - cons531", ptr %"object value532", align 8
  %"insert value object533" = insertvalue { i32, ptr } { i32 6, ptr null }, ptr %"object value532", 1
  %"object value534" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr (ptr, ptr null, i32 1) to i32))
  store ptr @cdaddr, ptr %"object value534", align 8
  %"insert value object535" = insertvalue { i32, ptr } { i32 7, ptr null }, ptr %"object value534", 1
  %"object value536" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  store { i32, ptr } { i32 6, ptr @cdaddr.197 }, ptr %"object value536", align 8
  %"insert value object537" = insertvalue { i32, ptr } { i32 4, ptr null }, ptr %"object value536", 1
  %"car ptr538" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  %"cdr ptr539" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  store { i32, ptr } %"insert value object537", ptr %"car ptr538", align 8
  store { i32, ptr } %"insert value object527", ptr %"cdr ptr539", align 8
  %"insert car - cons540" = insertvalue { ptr, ptr } zeroinitializer, ptr %"car ptr538", 0
  %"insert cdr - cons541" = insertvalue { ptr, ptr } %"insert car - cons540", ptr %"cdr ptr539", 1
  %"object value542" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ ptr, ptr }, ptr null, i32 1) to i32))
  store { ptr, ptr } %"insert cdr - cons541", ptr %"object value542", align 8
  %"insert value object543" = insertvalue { i32, ptr } { i32 6, ptr null }, ptr %"object value542", 1
  %"car ptr544" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  %"cdr ptr545" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  store { i32, ptr } %"insert value object535", ptr %"car ptr544", align 8
  store { i32, ptr } %"insert value object533", ptr %"cdr ptr545", align 8
  %"insert car - cons546" = insertvalue { ptr, ptr } zeroinitializer, ptr %"car ptr544", 0
  %"insert cdr - cons547" = insertvalue { ptr, ptr } %"insert car - cons546", ptr %"cdr ptr545", 1
  %"object value548" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ ptr, ptr }, ptr null, i32 1) to i32))
  store { ptr, ptr } %"insert cdr - cons547", ptr %"object value548", align 8
  %"insert value object549" = insertvalue { i32, ptr } { i32 6, ptr null }, ptr %"object value548", 1
  %"object value550" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr (ptr, ptr null, i32 1) to i32))
  store ptr @cddaar, ptr %"object value550", align 8
  %"insert value object551" = insertvalue { i32, ptr } { i32 7, ptr null }, ptr %"object value550", 1
  %"object value552" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  store { i32, ptr } { i32 6, ptr @cddaar.198 }, ptr %"object value552", align 8
  %"insert value object553" = insertvalue { i32, ptr } { i32 4, ptr null }, ptr %"object value552", 1
  %"car ptr554" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  %"cdr ptr555" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  store { i32, ptr } %"insert value object553", ptr %"car ptr554", align 8
  store { i32, ptr } %"insert value object543", ptr %"cdr ptr555", align 8
  %"insert car - cons556" = insertvalue { ptr, ptr } zeroinitializer, ptr %"car ptr554", 0
  %"insert cdr - cons557" = insertvalue { ptr, ptr } %"insert car - cons556", ptr %"cdr ptr555", 1
  %"object value558" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ ptr, ptr }, ptr null, i32 1) to i32))
  store { ptr, ptr } %"insert cdr - cons557", ptr %"object value558", align 8
  %"insert value object559" = insertvalue { i32, ptr } { i32 6, ptr null }, ptr %"object value558", 1
  %"car ptr560" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  %"cdr ptr561" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  store { i32, ptr } %"insert value object551", ptr %"car ptr560", align 8
  store { i32, ptr } %"insert value object549", ptr %"cdr ptr561", align 8
  %"insert car - cons562" = insertvalue { ptr, ptr } zeroinitializer, ptr %"car ptr560", 0
  %"insert cdr - cons563" = insertvalue { ptr, ptr } %"insert car - cons562", ptr %"cdr ptr561", 1
  %"object value564" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ ptr, ptr }, ptr null, i32 1) to i32))
  store { ptr, ptr } %"insert cdr - cons563", ptr %"object value564", align 8
  %"insert value object565" = insertvalue { i32, ptr } { i32 6, ptr null }, ptr %"object value564", 1
  %"object value566" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr (ptr, ptr null, i32 1) to i32))
  store ptr @cddadr, ptr %"object value566", align 8
  %"insert value object567" = insertvalue { i32, ptr } { i32 7, ptr null }, ptr %"object value566", 1
  %"object value568" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  store { i32, ptr } { i32 6, ptr @cddadr.199 }, ptr %"object value568", align 8
  %"insert value object569" = insertvalue { i32, ptr } { i32 4, ptr null }, ptr %"object value568", 1
  %"car ptr570" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  %"cdr ptr571" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  store { i32, ptr } %"insert value object569", ptr %"car ptr570", align 8
  store { i32, ptr } %"insert value object559", ptr %"cdr ptr571", align 8
  %"insert car - cons572" = insertvalue { ptr, ptr } zeroinitializer, ptr %"car ptr570", 0
  %"insert cdr - cons573" = insertvalue { ptr, ptr } %"insert car - cons572", ptr %"cdr ptr571", 1
  %"object value574" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ ptr, ptr }, ptr null, i32 1) to i32))
  store { ptr, ptr } %"insert cdr - cons573", ptr %"object value574", align 8
  %"insert value object575" = insertvalue { i32, ptr } { i32 6, ptr null }, ptr %"object value574", 1
  %"car ptr576" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  %"cdr ptr577" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  store { i32, ptr } %"insert value object567", ptr %"car ptr576", align 8
  store { i32, ptr } %"insert value object565", ptr %"cdr ptr577", align 8
  %"insert car - cons578" = insertvalue { ptr, ptr } zeroinitializer, ptr %"car ptr576", 0
  %"insert cdr - cons579" = insertvalue { ptr, ptr } %"insert car - cons578", ptr %"cdr ptr577", 1
  %"object value580" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ ptr, ptr }, ptr null, i32 1) to i32))
  store { ptr, ptr } %"insert cdr - cons579", ptr %"object value580", align 8
  %"insert value object581" = insertvalue { i32, ptr } { i32 6, ptr null }, ptr %"object value580", 1
  %"object value582" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr (ptr, ptr null, i32 1) to i32))
  store ptr @cdddar, ptr %"object value582", align 8
  %"insert value object583" = insertvalue { i32, ptr } { i32 7, ptr null }, ptr %"object value582", 1
  %"object value584" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  store { i32, ptr } { i32 6, ptr @cdddar.200 }, ptr %"object value584", align 8
  %"insert value object585" = insertvalue { i32, ptr } { i32 4, ptr null }, ptr %"object value584", 1
  %"car ptr586" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  %"cdr ptr587" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  store { i32, ptr } %"insert value object585", ptr %"car ptr586", align 8
  store { i32, ptr } %"insert value object575", ptr %"cdr ptr587", align 8
  %"insert car - cons588" = insertvalue { ptr, ptr } zeroinitializer, ptr %"car ptr586", 0
  %"insert cdr - cons589" = insertvalue { ptr, ptr } %"insert car - cons588", ptr %"cdr ptr587", 1
  %"object value590" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ ptr, ptr }, ptr null, i32 1) to i32))
  store { ptr, ptr } %"insert cdr - cons589", ptr %"object value590", align 8
  %"insert value object591" = insertvalue { i32, ptr } { i32 6, ptr null }, ptr %"object value590", 1
  %"car ptr592" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  %"cdr ptr593" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  store { i32, ptr } %"insert value object583", ptr %"car ptr592", align 8
  store { i32, ptr } %"insert value object581", ptr %"cdr ptr593", align 8
  %"insert car - cons594" = insertvalue { ptr, ptr } zeroinitializer, ptr %"car ptr592", 0
  %"insert cdr - cons595" = insertvalue { ptr, ptr } %"insert car - cons594", ptr %"cdr ptr593", 1
  %"object value596" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ ptr, ptr }, ptr null, i32 1) to i32))
  store { ptr, ptr } %"insert cdr - cons595", ptr %"object value596", align 8
  %"insert value object597" = insertvalue { i32, ptr } { i32 6, ptr null }, ptr %"object value596", 1
  %"object value598" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr (ptr, ptr null, i32 1) to i32))
  store ptr @cddddr, ptr %"object value598", align 8
  %"insert value object599" = insertvalue { i32, ptr } { i32 7, ptr null }, ptr %"object value598", 1
  %"object value600" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  store { i32, ptr } { i32 6, ptr @cddddr.201 }, ptr %"object value600", align 8
  %"insert value object601" = insertvalue { i32, ptr } { i32 4, ptr null }, ptr %"object value600", 1
  %"car ptr602" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  %"cdr ptr603" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  store { i32, ptr } %"insert value object601", ptr %"car ptr602", align 8
  store { i32, ptr } %"insert value object591", ptr %"cdr ptr603", align 8
  %"insert car - cons604" = insertvalue { ptr, ptr } zeroinitializer, ptr %"car ptr602", 0
  %"insert cdr - cons605" = insertvalue { ptr, ptr } %"insert car - cons604", ptr %"cdr ptr603", 1
  %"object value606" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ ptr, ptr }, ptr null, i32 1) to i32))
  store { ptr, ptr } %"insert cdr - cons605", ptr %"object value606", align 8
  %"insert value object607" = insertvalue { i32, ptr } { i32 6, ptr null }, ptr %"object value606", 1
  %"car ptr608" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  %"cdr ptr609" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  store { i32, ptr } %"insert value object599", ptr %"car ptr608", align 8
  store { i32, ptr } %"insert value object597", ptr %"cdr ptr609", align 8
  %"insert car - cons610" = insertvalue { ptr, ptr } zeroinitializer, ptr %"car ptr608", 0
  %"insert cdr - cons611" = insertvalue { ptr, ptr } %"insert car - cons610", ptr %"cdr ptr609", 1
  %"object value612" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ ptr, ptr }, ptr null, i32 1) to i32))
  store { ptr, ptr } %"insert cdr - cons611", ptr %"object value612", align 8
  %"insert value object613" = insertvalue { i32, ptr } { i32 6, ptr null }, ptr %"object value612", 1
  %"car ptr614" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  %"cdr ptr615" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  store { i32, ptr } %"insert value object9", ptr %"car ptr614", align 8
  store { i32, ptr } %"insert value object607", ptr %"cdr ptr615", align 8
  %"insert car - cons616" = insertvalue { ptr, ptr } zeroinitializer, ptr %"car ptr614", 0
  %"insert cdr - cons617" = insertvalue { ptr, ptr } %"insert car - cons616", ptr %"cdr ptr615", 1
  %"object value618" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ ptr, ptr }, ptr null, i32 1) to i32))
  store { ptr, ptr } %"insert cdr - cons617", ptr %"object value618", align 8
  %"insert value object619" = insertvalue { i32, ptr } { i32 6, ptr null }, ptr %"object value618", 1
  %"car ptr620" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  %"cdr ptr621" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  store { i32, ptr } %"insert value object7", ptr %"car ptr620", align 8
  store { i32, ptr } %"insert value object613", ptr %"cdr ptr621", align 8
  %"insert car - cons622" = insertvalue { ptr, ptr } zeroinitializer, ptr %"car ptr620", 0
  %"insert cdr - cons623" = insertvalue { ptr, ptr } %"insert car - cons622", ptr %"cdr ptr621", 1
  %"object value624" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ ptr, ptr }, ptr null, i32 1) to i32))
  store { ptr, ptr } %"insert cdr - cons623", ptr %"object value624", align 8
  %"insert value object625" = insertvalue { i32, ptr } { i32 6, ptr null }, ptr %"object value624", 1
  %"object value626" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  store { i32, ptr } { i32 3, ptr @nil }, ptr %"object value626", align 8
  %"insert value object627" = insertvalue { i32, ptr } { i32 4, ptr null }, ptr %"object value626", 1
  %"car ptr628" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  %"cdr ptr629" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  store { i32, ptr } %"insert value object627", ptr %"car ptr628", align 8
  store { i32, ptr } %"insert value object619", ptr %"cdr ptr629", align 8
  %"insert car - cons630" = insertvalue { ptr, ptr } zeroinitializer, ptr %"car ptr628", 0
  %"insert cdr - cons631" = insertvalue { ptr, ptr } %"insert car - cons630", ptr %"cdr ptr629", 1
  %"object value632" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ ptr, ptr }, ptr null, i32 1) to i32))
  store { ptr, ptr } %"insert cdr - cons631", ptr %"object value632", align 8
  %"insert value object633" = insertvalue { i32, ptr } { i32 6, ptr null }, ptr %"object value632", 1
  %"car ptr634" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  %"cdr ptr635" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  store { i32, ptr } zeroinitializer, ptr %"car ptr634", align 8
  store { i32, ptr } %"insert value object625", ptr %"cdr ptr635", align 8
  %"insert car - cons636" = insertvalue { ptr, ptr } zeroinitializer, ptr %"car ptr634", 0
  %"insert cdr - cons637" = insertvalue { ptr, ptr } %"insert car - cons636", ptr %"cdr ptr635", 1
  %"object value638" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ ptr, ptr }, ptr null, i32 1) to i32))
  store { ptr, ptr } %"insert cdr - cons637", ptr %"object value638", align 8
  %"insert value object639" = insertvalue { i32, ptr } { i32 6, ptr null }, ptr %"object value638", 1
  %"car ptr640" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  %"cdr ptr641" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  store { i32, ptr } %"insert value object633", ptr %"car ptr640", align 8
  store { i32, ptr } %"insert value object639", ptr %"cdr ptr641", align 8
  %"insert car - cons642" = insertvalue { ptr, ptr } zeroinitializer, ptr %"car ptr640", 0
  %"insert cdr - cons643" = insertvalue { ptr, ptr } %"insert car - cons642", ptr %"cdr ptr641", 1
  %"object value644" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ ptr, ptr }, ptr null, i32 1) to i32))
  store { ptr, ptr } %"insert cdr - cons643", ptr %"object value644", align 8
  %"insert value object645" = insertvalue { i32, ptr } { i32 6, ptr null }, ptr %"object value644", 1
  %"car ptr646" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  %"cdr ptr647" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  store { i32, ptr } %"insert value object645", ptr %"car ptr646", align 8
  store { i32, ptr } zeroinitializer, ptr %"cdr ptr647", align 8
  %"insert car - cons648" = insertvalue { ptr, ptr } zeroinitializer, ptr %"car ptr646", 0
  %"insert cdr - cons649" = insertvalue { ptr, ptr } %"insert car - cons648", ptr %"cdr ptr647", 1
  %"object value650" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ ptr, ptr }, ptr null, i32 1) to i32))
  store { ptr, ptr } %"insert cdr - cons649", ptr %"object value650", align 8
  %"insert value object651" = insertvalue { i32, ptr } { i32 6, ptr null }, ptr %"object value650", 1
  store { i32, ptr } %"insert value object651", ptr %env, align 8
  %"object value652" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr (ptr, ptr null, i32 1) to i32))
  store ptr blockaddress(@main, %entry1), ptr %"object value652", align 8
  %"insert value object653" = insertvalue { i32, ptr } { i32 5, ptr null }, ptr %"object value652", 1
  %"load register env" = load { i32, ptr }, ptr %env, align 8
  %"object value654" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  store { i32, ptr } { i32 18, ptr @compiled-procedure }, ptr %"object value654", align 8
  %"insert value object655" = insertvalue { i32, ptr } { i32 4, ptr null }, ptr %"object value654", 1
  %"object value656" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  store { i32, ptr } %"insert value object655", ptr %"object value656", align 8
  %"insert value object657" = insertvalue { i32, ptr } { i32 4, ptr null }, ptr %"object value656", 1
  %"insert into struct658" = insertvalue { { i32, ptr }, { i32, ptr } } zeroinitializer, { i32, ptr } %"insert value object653", 0
  %"insert into struct659" = insertvalue { { i32, ptr }, { i32, ptr } } %"insert into struct658", { i32, ptr } %"load register env", 1
  %"object value660" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ { i32, ptr }, { i32, ptr } }, ptr null, i32 1) to i32))
  store { { i32, ptr }, { i32, ptr } } %"insert into struct659", ptr %"object value660", align 8
  %"insert value object661" = insertvalue { i32, ptr } { i32 9, ptr null }, ptr %"object value660", 1
  store { i32, ptr } %"insert value object661", ptr %val, align 8
  br label %after-lambda2

error:                                            ; preds = %lookup-entry1828, %lookup-entry1622, %lookup-entry1415, %lookup-entry1209, %lookup-entry992, %lookup-entry869, %lookup-entry770, %lookup-entry, %add1
  %"error phi" = phi { ptr, i32 } [ { ptr @"error exit.37", i32 1 }, %add1 ], [ { ptr @"error exit.203", i32 1 }, %lookup-entry ], [ { ptr @"error exit.205", i32 1 }, %lookup-entry770 ], [ { ptr @"error exit.207", i32 1 }, %lookup-entry869 ], [ { ptr @"error exit.209", i32 1 }, %lookup-entry992 ], [ { ptr @"error exit.211", i32 1 }, %lookup-entry1209 ], [ { ptr @"error exit.214", i32 1 }, %lookup-entry1415 ], [ { ptr @"error exit.217", i32 1 }, %lookup-entry1622 ], [ { ptr @"error exit.220", i32 1 }, %lookup-entry1828 ]
  %error_msg = extractvalue { ptr, i32 } %"error phi", 0
  %error_code = extractvalue { ptr, i32 } %"error phi", 1
  %print = call i32 (ptr, ...) @printf(ptr %error_msg)
  call void @exit(i32 %error_code)
  unreachable

add1:                                             ; preds = %force63, %after-label66, %thunk65, %force60, %force52, %after-label55, %thunk54, %force49, %force41, %after-label44, %thunk43, %force38, %force30, %after-label33, %thunk32, %force27, %force24, %primitive-branch17, %compiled-branch18, %force15, %force7, %after-label10, %found775, %force4, %"extract-number:return"
  %"load argl1" = load { i32, ptr }, ptr %argl, align 8
  %"extract-cons:return" = extractvalue { i32, ptr } %"load argl1", 1
  %"extract-cons:" = load { ptr, ptr }, ptr %"extract-cons:return", align 8
  %"get car" = extractvalue { ptr, ptr } %"extract-cons:", 0
  %"load car" = load { i32, ptr }, ptr %"get car", align 8
  %get_type = extractvalue { i32, ptr } %"load car", 0
  %"extract-number:cmp-type" = icmp eq i32 %get_type, 2
  br i1 %"extract-number:cmp-type", label %"extract-number:return", label %error

"extract-number:return":                          ; preds = %add1
  %"extract-number:return2" = extractvalue { i32, ptr } %"load car", 1
  %"extract-number:" = load double, ptr %"extract-number:return2", align 8
  %"add 1" = fadd double %"extract-number:", 1.000000e+00
  %"object value" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr (double, ptr null, i32 1) to i32))
  store double %"add 1", ptr %"object value", align 8
  %"insert value object" = insertvalue { i32, ptr } { i32 2, ptr null }, ptr %"object value", 1
  store { i32, ptr } %"insert value object", ptr %val, align 8
  %"load register continue" = load { i32, ptr }, ptr %continue, align 8
  %"extract-label:return" = extractvalue { i32, ptr } %"load register continue", 1
  %"extract-label:" = load ptr, ptr %"extract-label:return", align 8
  indirectbr ptr %"extract-label:", [label %add1]

entry1:                                           ; preds = %force63, %after-label66, %thunk65, %force60, %force52, %after-label55, %thunk54, %force49, %force41, %after-label44, %thunk43, %force38, %force30, %after-label33, %thunk32, %force27, %force24, %primitive-branch17, %compiled-branch18, %force15, %force7, %after-label10, %found775, %force4, %next-block
  %"load register proc" = load { i32, ptr }, ptr %proc, align 8
  %"extract-lambda:return" = extractvalue { i32, ptr } %"load register proc", 1
  %"extract-lambda:" = load { { i32, ptr }, { i32, ptr } }, ptr %"extract-lambda:return", align 8
  %"proc env" = extractvalue { { i32, ptr }, { i32, ptr } } %"extract-lambda:", 1
  store { i32, ptr } %"proc env", ptr %env, align 8
  %"object value662" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  store { i32, ptr } { i32 3, ptr @"'0'" }, ptr %"object value662", align 8
  %"insert value object663" = insertvalue { i32, ptr } { i32 4, ptr null }, ptr %"object value662", 1
  %"load register argl" = load { i32, ptr }, ptr %argl, align 8
  %"load register env664" = load { i32, ptr }, ptr %env, align 8
  %"car ptr665" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  %"cdr ptr666" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  store { i32, ptr } %"insert value object663", ptr %"car ptr665", align 8
  store { i32, ptr } %"load register argl", ptr %"cdr ptr666", align 8
  %"insert car - cons667" = insertvalue { ptr, ptr } zeroinitializer, ptr %"car ptr665", 0
  %"insert cdr - cons668" = insertvalue { ptr, ptr } %"insert car - cons667", ptr %"cdr ptr666", 1
  %"object value669" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ ptr, ptr }, ptr null, i32 1) to i32))
  store { ptr, ptr } %"insert cdr - cons668", ptr %"object value669", align 8
  %"insert value object670" = insertvalue { i32, ptr } { i32 6, ptr null }, ptr %"object value669", 1
  %"car ptr671" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  %"cdr ptr672" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  store { i32, ptr } %"insert value object670", ptr %"car ptr671", align 8
  store { i32, ptr } %"load register env664", ptr %"cdr ptr672", align 8
  %"insert car - cons673" = insertvalue { ptr, ptr } zeroinitializer, ptr %"car ptr671", 0
  %"insert cdr - cons674" = insertvalue { ptr, ptr } %"insert car - cons673", ptr %"cdr ptr672", 1
  %"object value675" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ ptr, ptr }, ptr null, i32 1) to i32))
  store { ptr, ptr } %"insert cdr - cons674", ptr %"object value675", align 8
  %"insert value object676" = insertvalue { i32, ptr } { i32 6, ptr null }, ptr %"object value675", 1
  store { i32, ptr } %"insert value object676", ptr %env, align 8
  %"load stack" = load { { i32, ptr }, ptr }, ptr %stack, align 8
  %"previous stack" = alloca { { i32, ptr }, ptr }, align 8
  store { { i32, ptr }, ptr } %"load stack", ptr %"previous stack", align 8
  %"load register" = load { i32, ptr }, ptr %env, align 8
  %"save register" = insertvalue { { i32, ptr }, ptr } zeroinitializer, { i32, ptr } %"load register", 0
  %"save previous stack" = insertvalue { { i32, ptr }, ptr } %"save register", ptr %"previous stack", 1
  store { { i32, ptr }, ptr } %"save previous stack", ptr %stack, align 8
  %"load stack677" = load { { i32, ptr }, ptr }, ptr %stack, align 8
  %"previous stack678" = alloca { { i32, ptr }, ptr }, align 8
  store { { i32, ptr }, ptr } %"load stack677", ptr %"previous stack678", align 8
  %"load register679" = load { i32, ptr }, ptr %continue, align 8
  %"save register680" = insertvalue { { i32, ptr }, ptr } zeroinitializer, { i32, ptr } %"load register679", 0
  %"save previous stack681" = insertvalue { { i32, ptr }, ptr } %"save register680", ptr %"previous stack678", 1
  store { { i32, ptr }, ptr } %"save previous stack681", ptr %stack, align 8
  %"load stack682" = load { { i32, ptr }, ptr }, ptr %stack, align 8
  %"previous stack683" = alloca { { i32, ptr }, ptr }, align 8
  store { { i32, ptr }, ptr } %"load stack682", ptr %"previous stack683", align 8
  %"load register684" = load { i32, ptr }, ptr %env, align 8
  %"save register685" = insertvalue { { i32, ptr }, ptr } zeroinitializer, { i32, ptr } %"load register684", 0
  %"save previous stack686" = insertvalue { { i32, ptr }, ptr } %"save register685", ptr %"previous stack683", 1
  store { { i32, ptr }, ptr } %"save previous stack686", ptr %stack, align 8
  %"object value687" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  store { i32, ptr } { i32 5, ptr @print.202 }, ptr %"object value687", align 8
  %"insert value object688" = insertvalue { i32, ptr } { i32 4, ptr null }, ptr %"object value687", 1
  %"load register env689" = load { i32, ptr }, ptr %env, align 8
  %env690 = alloca { i32, ptr }, align 8
  store { i32, ptr } %"load register env689", ptr %env690, align 8
  br label %lookup-entry

actual-value3:                                    ; preds = %force63, %after-label66, %thunk65, %force60, %force52, %after-label55, %thunk54, %force49, %force41, %after-label44, %thunk43, %force38, %force30, %after-label33, %thunk32, %force27, %force24, %primitive-branch17, %compiled-branch18, %force15, %force7, %after-label10, %found775, %force4, %found
  %"load register thunk" = load { i32, ptr }, ptr %thunk, align 8
  %get_type730 = extractvalue { i32, ptr } %"load register thunk", 0
  %"is hempty731" = icmp eq i32 %get_type730, 8
  %"not thunk" = xor i1 %"is hempty731", true
  %"object value732" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr (i1, ptr null, i32 1) to i32))
  store i1 %"not thunk", ptr %"object value732", align 1
  %"insert value object733" = insertvalue { i32, ptr } { i32 1, ptr null }, ptr %"object value732", 1
  store { i32, ptr } %"insert value object733", ptr %flag, align 8
  %"load flag" = load { i32, ptr }, ptr %flag, align 8
  %get_type735 = extractvalue { i32, ptr } %"load flag", 0
  %"not bool check" = icmp ne i32 %get_type735, 1
  %"get object context" = extractvalue { i32, ptr } %"load flag", 1
  %"get bool value" = load i1, ptr %"get object context", align 1
  %"non bool or true" = or i1 %"not bool check", %"get bool value"
  br i1 %"non bool or true", label %done5, label %next-block734

force4:                                           ; preds = %force63, %after-label66, %thunk65, %force60, %force52, %after-label55, %thunk54, %force49, %force41, %after-label44, %thunk43, %force38, %force30, %after-label33, %thunk32, %force27, %force24, %primitive-branch17, %compiled-branch18, %force15, %force7, %after-label10, %found775, %force4, %next-block734
  %"object value736" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr (ptr, ptr null, i32 1) to i32))
  store ptr blockaddress(@main, %actual-value3), ptr %"object value736", align 8
  %"insert value object737" = insertvalue { i32, ptr } { i32 5, ptr null }, ptr %"object value736", 1
  store { i32, ptr } %"insert value object737", ptr %continue, align 8
  %"load register thunk738" = load { i32, ptr }, ptr %thunk, align 8
  %"extract-thunk:return" = extractvalue { i32, ptr } %"load register thunk738", 1
  %"extract-thunk:" = load { { i32, ptr }, { i32, ptr } }, ptr %"extract-thunk:return", align 8
  %"thunk entry" = extractvalue { { i32, ptr }, { i32, ptr } } %"extract-thunk:", 0
  store { i32, ptr } %"thunk entry", ptr %val, align 8
  %"load register val" = load { i32, ptr }, ptr %val, align 8
  %"extract-label:return739" = extractvalue { i32, ptr } %"load register val", 1
  %"extract-label:740" = load ptr, ptr %"extract-label:return739", align 8
  indirectbr ptr %"extract-label:740", [label %done28, label %after-call36, label %done16, label %after-call47, label %done8, label %compiled-branch35, label %force15, label %done53, label %compiled-branch57, label %after-label66, label %actual-value26, label %true-branch20, label %compiled-branch68, label %actual-value37, label %done50, label %actual-value6, label %add1, label %done61, label %actual-value23, label %primitive-branch34, label %thunk54, label %primitive-branch11, label %compiled-branch12, label %actual-value14, label %actual-value29, label %done31, label %force27, label %compiled-branch18, label %actual-value59, label %thunk32, label %compiled-branch46, label %force41, label %done42, label %after-call19, label %primitive-branch17, label %after-label10, label %actual-value51, label %after-label44, label %force52, label %thunk43, label %force4, label %primitive-branch45, label %force60, label %force7, label %force24, label %done39, label %actual-value40, label %primitive-branch67, label %thunk65, label %after-label55, label %force49, label %actual-value62, label %after-call13, label %after-if22, label %entry1, label %done64, label %primitive-branch56, label %after-label33, label %force30, label %done5, label %done25, label %thunk9, label %actual-value48, label %after-lambda2, label %false-branch21, label %actual-value3, label %after-call58, label %x, label %force63, label %after-call69, label %force38]

done5:                                            ; preds = %force63, %after-label66, %thunk65, %force60, %force52, %after-label55, %thunk54, %force49, %force41, %after-label44, %thunk43, %force38, %force30, %after-label33, %thunk32, %force27, %force24, %primitive-branch17, %compiled-branch18, %force15, %force7, %after-label10, %found775, %next-block741, %force4, %actual-value3
  %"load register thunk742" = load { i32, ptr }, ptr %thunk, align 8
  store { i32, ptr } %"load register thunk742", ptr %proc, align 8
  %stack743 = load { { i32, ptr }, ptr }, ptr %stack, align 8
  %"old stack" = extractvalue { { i32, ptr }, ptr } %stack743, 1
  %"current stack" = extractvalue { { i32, ptr }, ptr } %stack743, 0
  store { i32, ptr } %"current stack", ptr %env, align 8
  %"load previous stack" = load { { i32, ptr }, ptr }, ptr %"old stack", align 8
  store { { i32, ptr }, ptr } %"load previous stack", ptr %stack, align 8
  %"load register proc744" = load { i32, ptr }, ptr %proc, align 8
  %get_type745 = extractvalue { i32, ptr } %"load register proc744", 0
  %"is hempty746" = icmp eq i32 %get_type745, 7
  %"object value747" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr (i1, ptr null, i32 1) to i32))
  store i1 %"is hempty746", ptr %"object value747", align 1
  %"insert value object748" = insertvalue { i32, ptr } { i32 1, ptr null }, ptr %"object value747", 1
  store { i32, ptr } %"insert value object748", ptr %flag, align 8
  %"load flag749" = load { i32, ptr }, ptr %flag, align 8
  %get_type751 = extractvalue { i32, ptr } %"load flag749", 0
  %"not bool check752" = icmp ne i32 %get_type751, 1
  %"get object context753" = extractvalue { i32, ptr } %"load flag749", 1
  %"get bool value754" = load i1, ptr %"get object context753", align 1
  %"non bool or true755" = or i1 %"not bool check752", %"get bool value754"
  br i1 %"non bool or true755", label %primitive-branch11, label %next-block750

compiled-branch12:                                ; preds = %force63, %after-label66, %thunk65, %force60, %force52, %after-label55, %thunk54, %force49, %force41, %after-label44, %thunk43, %force38, %force30, %after-label33, %thunk32, %force27, %force24, %primitive-branch17, %compiled-branch18, %force15, %force7, %after-label10, %found775, %next-block750, %force4
  store { i32, ptr } zeroinitializer, ptr %argl, align 8
  %"object value756" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr (ptr, ptr null, i32 1) to i32))
  store ptr blockaddress(@main, %thunk9), ptr %"object value756", align 8
  %"insert value object757" = insertvalue { i32, ptr } { i32 5, ptr null }, ptr %"object value756", 1
  %"load register env758" = load { i32, ptr }, ptr %env, align 8
  %"insert into struct759" = insertvalue { { i32, ptr }, { i32, ptr } } zeroinitializer, { i32, ptr } %"insert value object757", 0
  %"insert into struct760" = insertvalue { { i32, ptr }, { i32, ptr } } %"insert into struct759", { i32, ptr } %"load register env758", 1
  %"object value761" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ { i32, ptr }, { i32, ptr } }, ptr null, i32 1) to i32))
  store { { i32, ptr }, { i32, ptr } } %"insert into struct760", ptr %"object value761", align 8
  %"insert value object762" = insertvalue { i32, ptr } { i32 8, ptr null }, ptr %"object value761", 1
  store { i32, ptr } %"insert value object762", ptr %val, align 8
  br label %after-label10

thunk9:                                           ; preds = %force63, %after-label66, %thunk65, %force60, %force52, %after-label55, %thunk54, %force49, %force41, %after-label44, %thunk43, %force38, %force30, %after-label33, %thunk32, %force27, %force24, %primitive-branch17, %compiled-branch18, %force15, %force7, %after-label10, %found775, %next-block763, %force4
  %"load register thunk764" = load { i32, ptr }, ptr %thunk, align 8
  %"extract-thunk:return765" = extractvalue { i32, ptr } %"load register thunk764", 1
  %"extract-thunk:766" = load { { i32, ptr }, { i32, ptr } }, ptr %"extract-thunk:return765", align 8
  %"thunk env" = extractvalue { { i32, ptr }, { i32, ptr } } %"extract-thunk:766", 1
  store { i32, ptr } %"thunk env", ptr %env, align 8
  %"object value767" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  store { i32, ptr } { i32 3, ptr @"'0'.204" }, ptr %"object value767", align 8
  %"insert value object768" = insertvalue { i32, ptr } { i32 4, ptr null }, ptr %"object value767", 1
  %"load register env769" = load { i32, ptr }, ptr %env, align 8
  %env777 = alloca { i32, ptr }, align 8
  store { i32, ptr } %"load register env769", ptr %env777, align 8
  br label %lookup-entry770

after-label10:                                    ; preds = %force63, %after-label66, %thunk65, %force60, %force52, %after-label55, %thunk54, %force49, %force41, %after-label44, %thunk43, %force38, %force30, %after-label33, %thunk32, %force27, %force24, %primitive-branch17, %compiled-branch18, %force15, %force7, %after-label10, %next-block838, %found775, %compiled-branch12, %force4
  %"load register val839" = load { i32, ptr }, ptr %val, align 8
  %"load register argl840" = load { i32, ptr }, ptr %argl, align 8
  %"car ptr841" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  %"cdr ptr842" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  store { i32, ptr } %"load register val839", ptr %"car ptr841", align 8
  store { i32, ptr } %"load register argl840", ptr %"cdr ptr842", align 8
  %"insert car - cons843" = insertvalue { ptr, ptr } zeroinitializer, ptr %"car ptr841", 0
  %"insert cdr - cons844" = insertvalue { ptr, ptr } %"insert car - cons843", ptr %"cdr ptr842", 1
  %"object value845" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ ptr, ptr }, ptr null, i32 1) to i32))
  store { ptr, ptr } %"insert cdr - cons844", ptr %"object value845", align 8
  %"insert value object846" = insertvalue { i32, ptr } { i32 6, ptr null }, ptr %"object value845", 1
  store { i32, ptr } %"insert value object846", ptr %argl, align 8
  %"object value847" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr (ptr, ptr null, i32 1) to i32))
  store ptr blockaddress(@main, %after-call13), ptr %"object value847", align 8
  %"insert value object848" = insertvalue { i32, ptr } { i32 5, ptr null }, ptr %"object value847", 1
  store { i32, ptr } %"insert value object848", ptr %continue, align 8
  %"load register proc849" = load { i32, ptr }, ptr %proc, align 8
  %"extract-lambda:return850" = extractvalue { i32, ptr } %"load register proc849", 1
  %"extract-lambda:851" = load { { i32, ptr }, { i32, ptr } }, ptr %"extract-lambda:return850", align 8
  %"proc entry" = extractvalue { { i32, ptr }, { i32, ptr } } %"extract-lambda:851", 0
  store { i32, ptr } %"proc entry", ptr %val, align 8
  %"load register val852" = load { i32, ptr }, ptr %val, align 8
  %"extract-label:return853" = extractvalue { i32, ptr } %"load register val852", 1
  %"extract-label:854" = load ptr, ptr %"extract-label:return853", align 8
  indirectbr ptr %"extract-label:854", [label %done28, label %after-call36, label %done16, label %after-call47, label %done8, label %compiled-branch35, label %force15, label %done53, label %compiled-branch57, label %after-label66, label %actual-value26, label %true-branch20, label %compiled-branch68, label %actual-value37, label %done50, label %actual-value6, label %add1, label %done61, label %actual-value23, label %primitive-branch34, label %thunk54, label %primitive-branch11, label %compiled-branch12, label %actual-value14, label %actual-value29, label %done31, label %force27, label %compiled-branch18, label %actual-value59, label %thunk32, label %compiled-branch46, label %force41, label %done42, label %after-call19, label %primitive-branch17, label %after-label10, label %actual-value51, label %after-label44, label %force52, label %thunk43, label %force4, label %primitive-branch45, label %force60, label %force7, label %force24, label %done39, label %actual-value40, label %primitive-branch67, label %thunk65, label %after-label55, label %force49, label %actual-value62, label %after-call13, label %after-if22, label %entry1, label %done64, label %primitive-branch56, label %after-label33, label %force30, label %done5, label %done25, label %thunk9, label %actual-value48, label %after-lambda2, label %false-branch21, label %actual-value3, label %after-call58, label %x, label %force63, label %after-call69, label %force38]

primitive-branch11:                               ; preds = %force63, %after-label66, %thunk65, %force60, %force52, %after-label55, %thunk54, %force49, %force41, %after-label44, %thunk43, %force38, %force30, %after-label33, %thunk32, %force27, %force24, %primitive-branch17, %compiled-branch18, %force15, %force7, %next-block855, %after-label10, %found775, %done5, %force4
  %"load stack856" = load { { i32, ptr }, ptr }, ptr %stack, align 8
  %"previous stack857" = alloca { { i32, ptr }, ptr }, align 8
  store { { i32, ptr }, ptr } %"load stack856", ptr %"previous stack857", align 8
  %"load register858" = load { i32, ptr }, ptr %proc, align 8
  %"save register859" = insertvalue { { i32, ptr }, ptr } zeroinitializer, { i32, ptr } %"load register858", 0
  %"save previous stack860" = insertvalue { { i32, ptr }, ptr } %"save register859", ptr %"previous stack857", 1
  store { { i32, ptr }, ptr } %"save previous stack860", ptr %stack, align 8
  store { i32, ptr } zeroinitializer, ptr %argl, align 8
  %"load stack861" = load { { i32, ptr }, ptr }, ptr %stack, align 8
  %"previous stack862" = alloca { { i32, ptr }, ptr }, align 8
  store { { i32, ptr }, ptr } %"load stack861", ptr %"previous stack862", align 8
  %"load register863" = load { i32, ptr }, ptr %argl, align 8
  %"save register864" = insertvalue { { i32, ptr }, ptr } zeroinitializer, { i32, ptr } %"load register863", 0
  %"save previous stack865" = insertvalue { { i32, ptr }, ptr } %"save register864", ptr %"previous stack862", 1
  store { { i32, ptr }, ptr } %"save previous stack865", ptr %stack, align 8
  %"object value866" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  store { i32, ptr } { i32 3, ptr @"'0'.206" }, ptr %"object value866", align 8
  %"insert value object867" = insertvalue { i32, ptr } { i32 4, ptr null }, ptr %"object value866", 1
  %"load register env868" = load { i32, ptr }, ptr %env, align 8
  %env876 = alloca { i32, ptr }, align 8
  store { i32, ptr } %"load register env868", ptr %env876, align 8
  br label %lookup-entry869

actual-value6:                                    ; preds = %force63, %after-label66, %thunk65, %force60, %force52, %after-label55, %thunk54, %force49, %force41, %after-label44, %thunk43, %force38, %force30, %after-label33, %thunk32, %force27, %force24, %primitive-branch17, %compiled-branch18, %force15, %force7, %found874, %after-label10, %found775, %force4
  %"load register thunk934" = load { i32, ptr }, ptr %thunk, align 8
  %get_type935 = extractvalue { i32, ptr } %"load register thunk934", 0
  %"is hempty936" = icmp eq i32 %get_type935, 8
  %"not thunk937" = xor i1 %"is hempty936", true
  %"object value938" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr (i1, ptr null, i32 1) to i32))
  store i1 %"not thunk937", ptr %"object value938", align 1
  %"insert value object939" = insertvalue { i32, ptr } { i32 1, ptr null }, ptr %"object value938", 1
  store { i32, ptr } %"insert value object939", ptr %flag, align 8
  %"load flag940" = load { i32, ptr }, ptr %flag, align 8
  %get_type942 = extractvalue { i32, ptr } %"load flag940", 0
  %"not bool check943" = icmp ne i32 %get_type942, 1
  %"get object context944" = extractvalue { i32, ptr } %"load flag940", 1
  %"get bool value945" = load i1, ptr %"get object context944", align 1
  %"non bool or true946" = or i1 %"not bool check943", %"get bool value945"
  br i1 %"non bool or true946", label %done8, label %next-block941

force7:                                           ; preds = %force63, %after-label66, %thunk65, %force60, %force52, %after-label55, %thunk54, %force49, %force41, %after-label44, %thunk43, %force38, %force30, %after-label33, %thunk32, %force27, %force24, %primitive-branch17, %compiled-branch18, %force15, %force7, %next-block941, %after-label10, %found775, %force4
  %"object value947" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr (ptr, ptr null, i32 1) to i32))
  store ptr blockaddress(@main, %actual-value6), ptr %"object value947", align 8
  %"insert value object948" = insertvalue { i32, ptr } { i32 5, ptr null }, ptr %"object value947", 1
  store { i32, ptr } %"insert value object948", ptr %continue, align 8
  %"load register thunk949" = load { i32, ptr }, ptr %thunk, align 8
  %"extract-thunk:return950" = extractvalue { i32, ptr } %"load register thunk949", 1
  %"extract-thunk:951" = load { { i32, ptr }, { i32, ptr } }, ptr %"extract-thunk:return950", align 8
  %"thunk entry952" = extractvalue { { i32, ptr }, { i32, ptr } } %"extract-thunk:951", 0
  store { i32, ptr } %"thunk entry952", ptr %val, align 8
  %"load register val953" = load { i32, ptr }, ptr %val, align 8
  %"extract-label:return954" = extractvalue { i32, ptr } %"load register val953", 1
  %"extract-label:955" = load ptr, ptr %"extract-label:return954", align 8
  indirectbr ptr %"extract-label:955", [label %done28, label %after-call36, label %done16, label %after-call47, label %done8, label %compiled-branch35, label %force15, label %done53, label %compiled-branch57, label %after-label66, label %actual-value26, label %true-branch20, label %compiled-branch68, label %actual-value37, label %done50, label %actual-value6, label %add1, label %done61, label %actual-value23, label %primitive-branch34, label %thunk54, label %primitive-branch11, label %compiled-branch12, label %actual-value14, label %actual-value29, label %done31, label %force27, label %compiled-branch18, label %actual-value59, label %thunk32, label %compiled-branch46, label %force41, label %done42, label %after-call19, label %primitive-branch17, label %after-label10, label %actual-value51, label %after-label44, label %force52, label %thunk43, label %force4, label %primitive-branch45, label %force60, label %force7, label %force24, label %done39, label %actual-value40, label %primitive-branch67, label %thunk65, label %after-label55, label %force49, label %actual-value62, label %after-call13, label %after-if22, label %entry1, label %done64, label %primitive-branch56, label %after-label33, label %force30, label %done5, label %done25, label %thunk9, label %actual-value48, label %after-lambda2, label %false-branch21, label %actual-value3, label %after-call58, label %x, label %force63, label %after-call69, label %force38]

done8:                                            ; preds = %force63, %after-label66, %thunk65, %force60, %force52, %after-label55, %thunk54, %force49, %force41, %after-label44, %thunk43, %force38, %force30, %after-label33, %thunk32, %force27, %force24, %primitive-branch17, %compiled-branch18, %force15, %next-block956, %force7, %actual-value6, %after-label10, %found775, %force4
  %"load register thunk957" = load { i32, ptr }, ptr %thunk, align 8
  store { i32, ptr } %"load register thunk957", ptr %val, align 8
  %stack958 = load { { i32, ptr }, ptr }, ptr %stack, align 8
  %"old stack959" = extractvalue { { i32, ptr }, ptr } %stack958, 1
  %"current stack960" = extractvalue { { i32, ptr }, ptr } %stack958, 0
  store { i32, ptr } %"current stack960", ptr %argl, align 8
  %"load previous stack961" = load { { i32, ptr }, ptr }, ptr %"old stack959", align 8
  store { { i32, ptr }, ptr } %"load previous stack961", ptr %stack, align 8
  %"load register val962" = load { i32, ptr }, ptr %val, align 8
  %"load register argl963" = load { i32, ptr }, ptr %argl, align 8
  %"car ptr964" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  %"cdr ptr965" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  store { i32, ptr } %"load register val962", ptr %"car ptr964", align 8
  store { i32, ptr } %"load register argl963", ptr %"cdr ptr965", align 8
  %"insert car - cons966" = insertvalue { ptr, ptr } zeroinitializer, ptr %"car ptr964", 0
  %"insert cdr - cons967" = insertvalue { ptr, ptr } %"insert car - cons966", ptr %"cdr ptr965", 1
  %"object value968" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ ptr, ptr }, ptr null, i32 1) to i32))
  store { ptr, ptr } %"insert cdr - cons967", ptr %"object value968", align 8
  %"insert value object969" = insertvalue { i32, ptr } { i32 6, ptr null }, ptr %"object value968", 1
  store { i32, ptr } %"insert value object969", ptr %argl, align 8
  %stack970 = load { { i32, ptr }, ptr }, ptr %stack, align 8
  %"old stack971" = extractvalue { { i32, ptr }, ptr } %stack970, 1
  %"current stack972" = extractvalue { { i32, ptr }, ptr } %stack970, 0
  store { i32, ptr } %"current stack972", ptr %proc, align 8
  %"load previous stack973" = load { { i32, ptr }, ptr }, ptr %"old stack971", align 8
  store { { i32, ptr }, ptr } %"load previous stack973", ptr %stack, align 8
  %"load register proc974" = load { i32, ptr }, ptr %proc, align 8
  %"load register argl975" = load { i32, ptr }, ptr %argl, align 8
  %"extract-primitive:return" = extractvalue { i32, ptr } %"load register proc974", 1
  %"extract-primitive:" = load ptr, ptr %"extract-primitive:return", align 8
  %"call primitive" = call { i32, ptr } %"extract-primitive:"({ i32, ptr } %"load register argl975")
  store { i32, ptr } %"call primitive", ptr %val, align 8
  br label %after-call13

after-call13:                                     ; preds = %force63, %after-label66, %thunk65, %force60, %force52, %after-label55, %thunk54, %force49, %force41, %after-label44, %thunk43, %force38, %force30, %after-label33, %thunk32, %force27, %force24, %primitive-branch17, %compiled-branch18, %force15, %done8, %force7, %after-label10, %found775, %force4
  %stack976 = load { { i32, ptr }, ptr }, ptr %stack, align 8
  %"old stack977" = extractvalue { { i32, ptr }, ptr } %stack976, 1
  %"current stack978" = extractvalue { { i32, ptr }, ptr } %stack976, 0
  store { i32, ptr } %"current stack978", ptr %continue, align 8
  %"load previous stack979" = load { { i32, ptr }, ptr }, ptr %"old stack977", align 8
  store { { i32, ptr }, ptr } %"load previous stack979", ptr %stack, align 8
  %stack980 = load { { i32, ptr }, ptr }, ptr %stack, align 8
  %"old stack981" = extractvalue { { i32, ptr }, ptr } %stack980, 1
  %"current stack982" = extractvalue { { i32, ptr }, ptr } %stack980, 0
  store { i32, ptr } %"current stack982", ptr %env, align 8
  %"load previous stack983" = load { { i32, ptr }, ptr }, ptr %"old stack981", align 8
  store { { i32, ptr }, ptr } %"load previous stack983", ptr %stack, align 8
  %"load stack984" = load { { i32, ptr }, ptr }, ptr %stack, align 8
  %"previous stack985" = alloca { { i32, ptr }, ptr }, align 8
  store { { i32, ptr }, ptr } %"load stack984", ptr %"previous stack985", align 8
  %"load register986" = load { i32, ptr }, ptr %continue, align 8
  %"save register987" = insertvalue { { i32, ptr }, ptr } zeroinitializer, { i32, ptr } %"load register986", 0
  %"save previous stack988" = insertvalue { { i32, ptr }, ptr } %"save register987", ptr %"previous stack985", 1
  store { { i32, ptr }, ptr } %"save previous stack988", ptr %stack, align 8
  %"object value989" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  store { i32, ptr } { i32 7, ptr @newline.208 }, ptr %"object value989", align 8
  %"insert value object990" = insertvalue { i32, ptr } { i32 4, ptr null }, ptr %"object value989", 1
  %"load register env991" = load { i32, ptr }, ptr %env, align 8
  %env999 = alloca { i32, ptr }, align 8
  store { i32, ptr } %"load register env991", ptr %env999, align 8
  br label %lookup-entry992

actual-value14:                                   ; preds = %force63, %after-label66, %thunk65, %force60, %force52, %after-label55, %thunk54, %force49, %force41, %after-label44, %thunk43, %force38, %force30, %after-label33, %thunk32, %force27, %force24, %primitive-branch17, %compiled-branch18, %force15, %found997, %force7, %after-label10, %found775, %force4
  %"load register thunk1057" = load { i32, ptr }, ptr %thunk, align 8
  %get_type1058 = extractvalue { i32, ptr } %"load register thunk1057", 0
  %"is hempty1059" = icmp eq i32 %get_type1058, 8
  %"not thunk1060" = xor i1 %"is hempty1059", true
  %"object value1061" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr (i1, ptr null, i32 1) to i32))
  store i1 %"not thunk1060", ptr %"object value1061", align 1
  %"insert value object1062" = insertvalue { i32, ptr } { i32 1, ptr null }, ptr %"object value1061", 1
  store { i32, ptr } %"insert value object1062", ptr %flag, align 8
  %"load flag1063" = load { i32, ptr }, ptr %flag, align 8
  %get_type1065 = extractvalue { i32, ptr } %"load flag1063", 0
  %"not bool check1066" = icmp ne i32 %get_type1065, 1
  %"get object context1067" = extractvalue { i32, ptr } %"load flag1063", 1
  %"get bool value1068" = load i1, ptr %"get object context1067", align 1
  %"non bool or true1069" = or i1 %"not bool check1066", %"get bool value1068"
  br i1 %"non bool or true1069", label %done16, label %next-block1064

force15:                                          ; preds = %force63, %after-label66, %thunk65, %force60, %force52, %after-label55, %thunk54, %force49, %force41, %after-label44, %thunk43, %force38, %force30, %after-label33, %thunk32, %force27, %force24, %primitive-branch17, %compiled-branch18, %force15, %next-block1064, %force7, %after-label10, %found775, %force4
  %"object value1070" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr (ptr, ptr null, i32 1) to i32))
  store ptr blockaddress(@main, %actual-value14), ptr %"object value1070", align 8
  %"insert value object1071" = insertvalue { i32, ptr } { i32 5, ptr null }, ptr %"object value1070", 1
  store { i32, ptr } %"insert value object1071", ptr %continue, align 8
  %"load register thunk1072" = load { i32, ptr }, ptr %thunk, align 8
  %"extract-thunk:return1073" = extractvalue { i32, ptr } %"load register thunk1072", 1
  %"extract-thunk:1074" = load { { i32, ptr }, { i32, ptr } }, ptr %"extract-thunk:return1073", align 8
  %"thunk entry1075" = extractvalue { { i32, ptr }, { i32, ptr } } %"extract-thunk:1074", 0
  store { i32, ptr } %"thunk entry1075", ptr %val, align 8
  %"load register val1076" = load { i32, ptr }, ptr %val, align 8
  %"extract-label:return1077" = extractvalue { i32, ptr } %"load register val1076", 1
  %"extract-label:1078" = load ptr, ptr %"extract-label:return1077", align 8
  indirectbr ptr %"extract-label:1078", [label %done28, label %after-call36, label %done16, label %after-call47, label %done8, label %compiled-branch35, label %force15, label %done53, label %compiled-branch57, label %after-label66, label %actual-value26, label %true-branch20, label %compiled-branch68, label %actual-value37, label %done50, label %actual-value6, label %add1, label %done61, label %actual-value23, label %primitive-branch34, label %thunk54, label %primitive-branch11, label %compiled-branch12, label %actual-value14, label %actual-value29, label %done31, label %force27, label %compiled-branch18, label %actual-value59, label %thunk32, label %compiled-branch46, label %force41, label %done42, label %after-call19, label %primitive-branch17, label %after-label10, label %actual-value51, label %after-label44, label %force52, label %thunk43, label %force4, label %primitive-branch45, label %force60, label %force7, label %force24, label %done39, label %actual-value40, label %primitive-branch67, label %thunk65, label %after-label55, label %force49, label %actual-value62, label %after-call13, label %after-if22, label %entry1, label %done64, label %primitive-branch56, label %after-label33, label %force30, label %done5, label %done25, label %thunk9, label %actual-value48, label %after-lambda2, label %false-branch21, label %actual-value3, label %after-call58, label %x, label %force63, label %after-call69, label %force38]

done16:                                           ; preds = %force63, %after-label66, %thunk65, %force60, %force52, %after-label55, %thunk54, %force49, %force41, %after-label44, %thunk43, %force38, %force30, %after-label33, %thunk32, %force27, %force24, %primitive-branch17, %compiled-branch18, %next-block1079, %force15, %actual-value14, %force7, %after-label10, %found775, %force4
  %"load register thunk1080" = load { i32, ptr }, ptr %thunk, align 8
  store { i32, ptr } %"load register thunk1080", ptr %proc, align 8
  %stack1081 = load { { i32, ptr }, ptr }, ptr %stack, align 8
  %"old stack1082" = extractvalue { { i32, ptr }, ptr } %stack1081, 1
  %"current stack1083" = extractvalue { { i32, ptr }, ptr } %stack1081, 0
  store { i32, ptr } %"current stack1083", ptr %continue, align 8
  %"load previous stack1084" = load { { i32, ptr }, ptr }, ptr %"old stack1082", align 8
  store { { i32, ptr }, ptr } %"load previous stack1084", ptr %stack, align 8
  %"load register proc1085" = load { i32, ptr }, ptr %proc, align 8
  %get_type1086 = extractvalue { i32, ptr } %"load register proc1085", 0
  %"is hempty1087" = icmp eq i32 %get_type1086, 7
  %"object value1088" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr (i1, ptr null, i32 1) to i32))
  store i1 %"is hempty1087", ptr %"object value1088", align 1
  %"insert value object1089" = insertvalue { i32, ptr } { i32 1, ptr null }, ptr %"object value1088", 1
  store { i32, ptr } %"insert value object1089", ptr %flag, align 8
  %"load flag1090" = load { i32, ptr }, ptr %flag, align 8
  %get_type1092 = extractvalue { i32, ptr } %"load flag1090", 0
  %"not bool check1093" = icmp ne i32 %get_type1092, 1
  %"get object context1094" = extractvalue { i32, ptr } %"load flag1090", 1
  %"get bool value1095" = load i1, ptr %"get object context1094", align 1
  %"non bool or true1096" = or i1 %"not bool check1093", %"get bool value1095"
  br i1 %"non bool or true1096", label %primitive-branch17, label %next-block1091

compiled-branch18:                                ; preds = %force63, %after-label66, %thunk65, %force60, %force52, %after-label55, %thunk54, %force49, %force41, %after-label44, %thunk43, %force38, %force30, %after-label33, %thunk32, %force27, %force24, %primitive-branch17, %compiled-branch18, %next-block1091, %force15, %force7, %after-label10, %found775, %force4
  store { i32, ptr } zeroinitializer, ptr %argl, align 8
  %"load register proc1097" = load { i32, ptr }, ptr %proc, align 8
  %"extract-lambda:return1098" = extractvalue { i32, ptr } %"load register proc1097", 1
  %"extract-lambda:1099" = load { { i32, ptr }, { i32, ptr } }, ptr %"extract-lambda:return1098", align 8
  %"proc entry1100" = extractvalue { { i32, ptr }, { i32, ptr } } %"extract-lambda:1099", 0
  store { i32, ptr } %"proc entry1100", ptr %val, align 8
  %"load register val1101" = load { i32, ptr }, ptr %val, align 8
  %"extract-label:return1102" = extractvalue { i32, ptr } %"load register val1101", 1
  %"extract-label:1103" = load ptr, ptr %"extract-label:return1102", align 8
  indirectbr ptr %"extract-label:1103", [label %done28, label %after-call36, label %done16, label %after-call47, label %done8, label %compiled-branch35, label %force15, label %done53, label %compiled-branch57, label %after-label66, label %actual-value26, label %true-branch20, label %compiled-branch68, label %actual-value37, label %done50, label %actual-value6, label %add1, label %done61, label %actual-value23, label %primitive-branch34, label %thunk54, label %primitive-branch11, label %compiled-branch12, label %actual-value14, label %actual-value29, label %done31, label %force27, label %compiled-branch18, label %actual-value59, label %thunk32, label %compiled-branch46, label %force41, label %done42, label %after-call19, label %primitive-branch17, label %after-label10, label %actual-value51, label %after-label44, label %force52, label %thunk43, label %force4, label %primitive-branch45, label %force60, label %force7, label %force24, label %done39, label %actual-value40, label %primitive-branch67, label %thunk65, label %after-label55, label %force49, label %actual-value62, label %after-call13, label %after-if22, label %entry1, label %done64, label %primitive-branch56, label %after-label33, label %force30, label %done5, label %done25, label %thunk9, label %actual-value48, label %after-lambda2, label %false-branch21, label %actual-value3, label %after-call58, label %x, label %force63, label %after-call69, label %force38]

primitive-branch17:                               ; preds = %force63, %after-label66, %thunk65, %force60, %force52, %after-label55, %thunk54, %force49, %force41, %after-label44, %thunk43, %force38, %force30, %after-label33, %thunk32, %force27, %force24, %primitive-branch17, %next-block1104, %compiled-branch18, %done16, %force15, %force7, %after-label10, %found775, %force4
  store { i32, ptr } zeroinitializer, ptr %argl, align 8
  %"load register proc1105" = load { i32, ptr }, ptr %proc, align 8
  %"load register argl1106" = load { i32, ptr }, ptr %argl, align 8
  %"extract-primitive:return1107" = extractvalue { i32, ptr } %"load register proc1105", 1
  %"extract-primitive:1108" = load ptr, ptr %"extract-primitive:return1107", align 8
  %"call primitive1109" = call { i32, ptr } %"extract-primitive:1108"({ i32, ptr } %"load register argl1106")
  store { i32, ptr } %"call primitive1109", ptr %val, align 8
  %"load register continue1110" = load { i32, ptr }, ptr %continue, align 8
  %"extract-label:return1111" = extractvalue { i32, ptr } %"load register continue1110", 1
  %"extract-label:1112" = load ptr, ptr %"extract-label:return1111", align 8
  indirectbr ptr %"extract-label:1112", [label %done28, label %after-call36, label %done16, label %after-call47, label %done8, label %compiled-branch35, label %force15, label %done53, label %compiled-branch57, label %after-label66, label %actual-value26, label %true-branch20, label %compiled-branch68, label %actual-value37, label %done50, label %actual-value6, label %add1, label %done61, label %actual-value23, label %primitive-branch34, label %thunk54, label %primitive-branch11, label %compiled-branch12, label %actual-value14, label %actual-value29, label %done31, label %force27, label %compiled-branch18, label %actual-value59, label %thunk32, label %compiled-branch46, label %force41, label %done42, label %after-call19, label %primitive-branch17, label %after-label10, label %actual-value51, label %after-label44, label %force52, label %thunk43, label %force4, label %primitive-branch45, label %force60, label %force7, label %force24, label %done39, label %actual-value40, label %primitive-branch67, label %thunk65, label %after-label55, label %force49, label %actual-value62, label %after-call13, label %after-if22, label %entry1, label %done64, label %primitive-branch56, label %after-label33, label %force30, label %done5, label %done25, label %thunk9, label %actual-value48, label %after-lambda2, label %false-branch21, label %actual-value3, label %after-call58, label %x, label %force63, label %after-call69, label %force38]

after-call19:                                     ; preds = %force63, %after-label66, %thunk65, %force60, %force52, %after-label55, %thunk54, %force49, %force41, %after-label44, %thunk43, %force38, %force30, %after-label33, %thunk32, %force27, %force24, %next-block1113, %primitive-branch17, %compiled-branch18, %force15, %force7, %after-label10, %found775, %force4
  br label %after-lambda2

after-lambda2:                                    ; preds = %force63, %after-label66, %thunk65, %force60, %force52, %after-label55, %thunk54, %force49, %force41, %after-label44, %thunk43, %force38, %force30, %after-label33, %thunk32, %force27, %force24, %after-call19, %primitive-branch17, %compiled-branch18, %force15, %force7, %after-label10, %found775, %force4, %entry
  %"object value1114" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  store { i32, ptr } { i32 7, ptr @println }, ptr %"object value1114", align 8
  %"insert value object1115" = insertvalue { i32, ptr } { i32 4, ptr null }, ptr %"object value1114", 1
  %"load register val1116" = load { i32, ptr }, ptr %val, align 8
  %"load register env1117" = load { i32, ptr }, ptr %env, align 8
  %"extract-cons:return1118" = extractvalue { i32, ptr } %"load register env1117", 1
  %"extract-cons:1119" = load { ptr, ptr }, ptr %"extract-cons:return1118", align 8
  %"get car1120" = extractvalue { ptr, ptr } %"extract-cons:1119", 0
  %"load car1121" = load { i32, ptr }, ptr %"get car1120", align 8
  %"extract-cons:return1122" = extractvalue { i32, ptr } %"load car1121", 1
  %"extract-cons:1123" = load { ptr, ptr }, ptr %"extract-cons:return1122", align 8
  %"get car1124" = extractvalue { ptr, ptr } %"extract-cons:1123", 0
  %"load car1125" = load { i32, ptr }, ptr %"get car1124", align 8
  %"car ptr1126" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  %"cdr ptr1127" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  store { i32, ptr } %"insert value object1115", ptr %"car ptr1126", align 8
  store { i32, ptr } %"load car1125", ptr %"cdr ptr1127", align 8
  %"insert car - cons1128" = insertvalue { ptr, ptr } zeroinitializer, ptr %"car ptr1126", 0
  %"insert cdr - cons1129" = insertvalue { ptr, ptr } %"insert car - cons1128", ptr %"cdr ptr1127", 1
  %"object value1130" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ ptr, ptr }, ptr null, i32 1) to i32))
  store { ptr, ptr } %"insert cdr - cons1129", ptr %"object value1130", align 8
  %"insert value object1131" = insertvalue { i32, ptr } { i32 6, ptr null }, ptr %"object value1130", 1
  %"extract-cons:return1132" = extractvalue { i32, ptr } %"load car1121", 1
  %"extract-cons:1133" = load { ptr, ptr }, ptr %"extract-cons:return1132", align 8
  %"get car1134" = extractvalue { ptr, ptr } %"extract-cons:1133", 0
  store { i32, ptr } %"insert value object1131", ptr %"get car1134", align 8
  %"extract-cons:return1135" = extractvalue { i32, ptr } %"load car1121", 1
  %"extract-cons:1136" = load { ptr, ptr }, ptr %"extract-cons:return1135", align 8
  %"get cdr1137" = extractvalue { ptr, ptr } %"extract-cons:1136", 1
  %"load cdr1138" = load { i32, ptr }, ptr %"get cdr1137", align 8
  %"car ptr1139" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  %"cdr ptr1140" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  store { i32, ptr } %"load register val1116", ptr %"car ptr1139", align 8
  store { i32, ptr } %"load cdr1138", ptr %"cdr ptr1140", align 8
  %"insert car - cons1141" = insertvalue { ptr, ptr } zeroinitializer, ptr %"car ptr1139", 0
  %"insert cdr - cons1142" = insertvalue { ptr, ptr } %"insert car - cons1141", ptr %"cdr ptr1140", 1
  %"object value1143" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ ptr, ptr }, ptr null, i32 1) to i32))
  store { ptr, ptr } %"insert cdr - cons1142", ptr %"object value1143", align 8
  %"insert value object1144" = insertvalue { i32, ptr } { i32 6, ptr null }, ptr %"object value1143", 1
  %"extract-cons:return1145" = extractvalue { i32, ptr } %"load car1121", 1
  %"extract-cons:1146" = load { ptr, ptr }, ptr %"extract-cons:return1145", align 8
  %"get cdr1147" = extractvalue { ptr, ptr } %"extract-cons:1146", 1
  store { i32, ptr } %"insert value object1144", ptr %"get cdr1147", align 8
  %"object value1148" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  store { i32, ptr } { i32 2, ptr @ok }, ptr %"object value1148", align 8
  %"insert value object1149" = insertvalue { i32, ptr } { i32 4, ptr null }, ptr %"object value1148", 1
  store { i32, ptr } %"insert value object1149", ptr %val, align 8
  %"load stack1150" = load { { i32, ptr }, ptr }, ptr %stack, align 8
  %"previous stack1151" = alloca { { i32, ptr }, ptr }, align 8
  store { { i32, ptr }, ptr } %"load stack1150", ptr %"previous stack1151", align 8
  %"load register1152" = load { i32, ptr }, ptr %env, align 8
  %"save register1153" = insertvalue { { i32, ptr }, ptr } zeroinitializer, { i32, ptr } %"load register1152", 0
  %"save previous stack1154" = insertvalue { { i32, ptr }, ptr } %"save register1153", ptr %"previous stack1151", 1
  store { { i32, ptr }, ptr } %"save previous stack1154", ptr %stack, align 8
  %"object value1155" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr (i1, ptr null, i32 1) to i32))
  store i1 true, ptr %"object value1155", align 1
  %"insert value object1156" = insertvalue { i32, ptr } { i32 1, ptr null }, ptr %"object value1155", 1
  store { i32, ptr } %"insert value object1156", ptr %thunk, align 8
  br label %actual-value23

actual-value23:                                   ; preds = %force63, %after-label66, %thunk65, %force60, %force52, %after-label55, %thunk54, %force49, %force41, %after-label44, %thunk43, %force38, %force30, %after-label33, %thunk32, %force27, %force24, %after-lambda2, %primitive-branch17, %compiled-branch18, %force15, %force7, %after-label10, %found775, %force4
  %"load register thunk1157" = load { i32, ptr }, ptr %thunk, align 8
  %get_type1158 = extractvalue { i32, ptr } %"load register thunk1157", 0
  %"is hempty1159" = icmp eq i32 %get_type1158, 8
  %"not thunk1160" = xor i1 %"is hempty1159", true
  %"object value1161" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr (i1, ptr null, i32 1) to i32))
  store i1 %"not thunk1160", ptr %"object value1161", align 1
  %"insert value object1162" = insertvalue { i32, ptr } { i32 1, ptr null }, ptr %"object value1161", 1
  store { i32, ptr } %"insert value object1162", ptr %flag, align 8
  %"load flag1163" = load { i32, ptr }, ptr %flag, align 8
  %get_type1165 = extractvalue { i32, ptr } %"load flag1163", 0
  %"not bool check1166" = icmp ne i32 %get_type1165, 1
  %"get object context1167" = extractvalue { i32, ptr } %"load flag1163", 1
  %"get bool value1168" = load i1, ptr %"get object context1167", align 1
  %"non bool or true1169" = or i1 %"not bool check1166", %"get bool value1168"
  br i1 %"non bool or true1169", label %done25, label %next-block1164

force24:                                          ; preds = %force63, %after-label66, %thunk65, %force60, %force52, %after-label55, %thunk54, %force49, %force41, %after-label44, %thunk43, %force38, %force30, %after-label33, %thunk32, %force27, %force24, %next-block1164, %primitive-branch17, %compiled-branch18, %force15, %force7, %after-label10, %found775, %force4
  %"object value1170" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr (ptr, ptr null, i32 1) to i32))
  store ptr blockaddress(@main, %actual-value23), ptr %"object value1170", align 8
  %"insert value object1171" = insertvalue { i32, ptr } { i32 5, ptr null }, ptr %"object value1170", 1
  store { i32, ptr } %"insert value object1171", ptr %continue, align 8
  %"load register thunk1172" = load { i32, ptr }, ptr %thunk, align 8
  %"extract-thunk:return1173" = extractvalue { i32, ptr } %"load register thunk1172", 1
  %"extract-thunk:1174" = load { { i32, ptr }, { i32, ptr } }, ptr %"extract-thunk:return1173", align 8
  %"thunk entry1175" = extractvalue { { i32, ptr }, { i32, ptr } } %"extract-thunk:1174", 0
  store { i32, ptr } %"thunk entry1175", ptr %val, align 8
  %"load register val1176" = load { i32, ptr }, ptr %val, align 8
  %"extract-label:return1177" = extractvalue { i32, ptr } %"load register val1176", 1
  %"extract-label:1178" = load ptr, ptr %"extract-label:return1177", align 8
  indirectbr ptr %"extract-label:1178", [label %done28, label %after-call36, label %done16, label %after-call47, label %done8, label %compiled-branch35, label %force15, label %done53, label %compiled-branch57, label %after-label66, label %actual-value26, label %true-branch20, label %compiled-branch68, label %actual-value37, label %done50, label %actual-value6, label %add1, label %done61, label %actual-value23, label %primitive-branch34, label %thunk54, label %primitive-branch11, label %compiled-branch12, label %actual-value14, label %actual-value29, label %done31, label %force27, label %compiled-branch18, label %actual-value59, label %thunk32, label %compiled-branch46, label %force41, label %done42, label %after-call19, label %primitive-branch17, label %after-label10, label %actual-value51, label %after-label44, label %force52, label %thunk43, label %force4, label %primitive-branch45, label %force60, label %force7, label %force24, label %done39, label %actual-value40, label %primitive-branch67, label %thunk65, label %after-label55, label %force49, label %actual-value62, label %after-call13, label %after-if22, label %entry1, label %done64, label %primitive-branch56, label %after-label33, label %force30, label %done5, label %done25, label %thunk9, label %actual-value48, label %after-lambda2, label %false-branch21, label %actual-value3, label %after-call58, label %x, label %force63, label %after-call69, label %force38]

done25:                                           ; preds = %force63, %after-label66, %thunk65, %force60, %force52, %after-label55, %thunk54, %force49, %force41, %after-label44, %thunk43, %force38, %force30, %after-label33, %thunk32, %force27, %next-block1179, %force24, %actual-value23, %primitive-branch17, %compiled-branch18, %force15, %force7, %after-label10, %found775, %force4
  %"load register thunk1180" = load { i32, ptr }, ptr %thunk, align 8
  store { i32, ptr } %"load register thunk1180", ptr %val, align 8
  %stack1181 = load { { i32, ptr }, ptr }, ptr %stack, align 8
  %"old stack1182" = extractvalue { { i32, ptr }, ptr } %stack1181, 1
  %"current stack1183" = extractvalue { { i32, ptr }, ptr } %stack1181, 0
  store { i32, ptr } %"current stack1183", ptr %env, align 8
  %"load previous stack1184" = load { { i32, ptr }, ptr }, ptr %"old stack1182", align 8
  store { { i32, ptr }, ptr } %"load previous stack1184", ptr %stack, align 8
  %"load register val1185" = load { i32, ptr }, ptr %val, align 8
  %get_type1186 = extractvalue { i32, ptr } %"load register val1185", 0
  %"not bool check1187" = icmp ne i32 %get_type1186, 1
  %"get object context1188" = extractvalue { i32, ptr } %"load register val1185", 1
  %"get bool value1189" = load i1, ptr %"get object context1188", align 1
  %"non bool or true1190" = or i1 %"not bool check1187", %"get bool value1189"
  %"not truthy" = xor i1 %"non bool or true1190", true
  %"object value1191" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr (i1, ptr null, i32 1) to i32))
  store i1 %"not truthy", ptr %"object value1191", align 1
  %"insert value object1192" = insertvalue { i32, ptr } { i32 1, ptr null }, ptr %"object value1191", 1
  store { i32, ptr } %"insert value object1192", ptr %flag, align 8
  %"load flag1193" = load { i32, ptr }, ptr %flag, align 8
  %get_type1195 = extractvalue { i32, ptr } %"load flag1193", 0
  %"not bool check1196" = icmp ne i32 %get_type1195, 1
  %"get object context1197" = extractvalue { i32, ptr } %"load flag1193", 1
  %"get bool value1198" = load i1, ptr %"get object context1197", align 1
  %"non bool or true1199" = or i1 %"not bool check1196", %"get bool value1198"
  br i1 %"non bool or true1199", label %false-branch21, label %next-block1194

true-branch20:                                    ; preds = %force63, %after-label66, %thunk65, %force60, %force52, %after-label55, %thunk54, %force49, %force41, %after-label44, %thunk43, %force38, %force30, %after-label33, %thunk32, %force27, %next-block1194, %force24, %primitive-branch17, %compiled-branch18, %force15, %force7, %after-label10, %found775, %force4
  br label %x

actual-value26:                                   ; preds = %force63, %after-label66, %thunk65, %force60, %force52, %after-label55, %thunk54, %force49, %force41, %after-label44, %thunk43, %force38, %force30, %after-label33, %thunk32, %force27, %found1214, %force24, %primitive-branch17, %compiled-branch18, %force15, %force7, %after-label10, %found775, %force4
  %"load register thunk1274" = load { i32, ptr }, ptr %thunk, align 8
  %get_type1275 = extractvalue { i32, ptr } %"load register thunk1274", 0
  %"is hempty1276" = icmp eq i32 %get_type1275, 8
  %"not thunk1277" = xor i1 %"is hempty1276", true
  %"object value1278" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr (i1, ptr null, i32 1) to i32))
  store i1 %"not thunk1277", ptr %"object value1278", align 1
  %"insert value object1279" = insertvalue { i32, ptr } { i32 1, ptr null }, ptr %"object value1278", 1
  store { i32, ptr } %"insert value object1279", ptr %flag, align 8
  %"load flag1280" = load { i32, ptr }, ptr %flag, align 8
  %get_type1282 = extractvalue { i32, ptr } %"load flag1280", 0
  %"not bool check1283" = icmp ne i32 %get_type1282, 1
  %"get object context1284" = extractvalue { i32, ptr } %"load flag1280", 1
  %"get bool value1285" = load i1, ptr %"get object context1284", align 1
  %"non bool or true1286" = or i1 %"not bool check1283", %"get bool value1285"
  br i1 %"non bool or true1286", label %done28, label %next-block1281

force27:                                          ; preds = %force63, %after-label66, %thunk65, %force60, %force52, %after-label55, %thunk54, %force49, %force41, %after-label44, %thunk43, %force38, %force30, %after-label33, %thunk32, %force27, %next-block1281, %force24, %primitive-branch17, %compiled-branch18, %force15, %force7, %after-label10, %found775, %force4
  %"object value1287" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr (ptr, ptr null, i32 1) to i32))
  store ptr blockaddress(@main, %actual-value26), ptr %"object value1287", align 8
  %"insert value object1288" = insertvalue { i32, ptr } { i32 5, ptr null }, ptr %"object value1287", 1
  store { i32, ptr } %"insert value object1288", ptr %continue, align 8
  %"load register thunk1289" = load { i32, ptr }, ptr %thunk, align 8
  %"extract-thunk:return1290" = extractvalue { i32, ptr } %"load register thunk1289", 1
  %"extract-thunk:1291" = load { { i32, ptr }, { i32, ptr } }, ptr %"extract-thunk:return1290", align 8
  %"thunk entry1292" = extractvalue { { i32, ptr }, { i32, ptr } } %"extract-thunk:1291", 0
  store { i32, ptr } %"thunk entry1292", ptr %val, align 8
  %"load register val1293" = load { i32, ptr }, ptr %val, align 8
  %"extract-label:return1294" = extractvalue { i32, ptr } %"load register val1293", 1
  %"extract-label:1295" = load ptr, ptr %"extract-label:return1294", align 8
  indirectbr ptr %"extract-label:1295", [label %done28, label %after-call36, label %done16, label %after-call47, label %done8, label %compiled-branch35, label %force15, label %done53, label %compiled-branch57, label %after-label66, label %actual-value26, label %true-branch20, label %compiled-branch68, label %actual-value37, label %done50, label %actual-value6, label %add1, label %done61, label %actual-value23, label %primitive-branch34, label %thunk54, label %primitive-branch11, label %compiled-branch12, label %actual-value14, label %actual-value29, label %done31, label %force27, label %compiled-branch18, label %actual-value59, label %thunk32, label %compiled-branch46, label %force41, label %done42, label %after-call19, label %primitive-branch17, label %after-label10, label %actual-value51, label %after-label44, label %force52, label %thunk43, label %force4, label %primitive-branch45, label %force60, label %force7, label %force24, label %done39, label %actual-value40, label %primitive-branch67, label %thunk65, label %after-label55, label %force49, label %actual-value62, label %after-call13, label %after-if22, label %entry1, label %done64, label %primitive-branch56, label %after-label33, label %force30, label %done5, label %done25, label %thunk9, label %actual-value48, label %after-lambda2, label %false-branch21, label %actual-value3, label %after-call58, label %x, label %force63, label %after-call69, label %force38]

done28:                                           ; preds = %force63, %after-label66, %thunk65, %force60, %force52, %after-label55, %thunk54, %force49, %force41, %after-label44, %thunk43, %force38, %force30, %after-label33, %thunk32, %next-block1296, %force27, %actual-value26, %force24, %primitive-branch17, %compiled-branch18, %force15, %force7, %after-label10, %found775, %force4
  %"load register thunk1297" = load { i32, ptr }, ptr %thunk, align 8
  store { i32, ptr } %"load register thunk1297", ptr %proc, align 8
  %stack1298 = load { { i32, ptr }, ptr }, ptr %stack, align 8
  %"old stack1299" = extractvalue { { i32, ptr }, ptr } %stack1298, 1
  %"current stack1300" = extractvalue { { i32, ptr }, ptr } %stack1298, 0
  store { i32, ptr } %"current stack1300", ptr %env, align 8
  %"load previous stack1301" = load { { i32, ptr }, ptr }, ptr %"old stack1299", align 8
  store { { i32, ptr }, ptr } %"load previous stack1301", ptr %stack, align 8
  %"load register proc1302" = load { i32, ptr }, ptr %proc, align 8
  %get_type1303 = extractvalue { i32, ptr } %"load register proc1302", 0
  %"is hempty1304" = icmp eq i32 %get_type1303, 7
  %"object value1305" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr (i1, ptr null, i32 1) to i32))
  store i1 %"is hempty1304", ptr %"object value1305", align 1
  %"insert value object1306" = insertvalue { i32, ptr } { i32 1, ptr null }, ptr %"object value1305", 1
  store { i32, ptr } %"insert value object1306", ptr %flag, align 8
  %"load flag1307" = load { i32, ptr }, ptr %flag, align 8
  %get_type1309 = extractvalue { i32, ptr } %"load flag1307", 0
  %"not bool check1310" = icmp ne i32 %get_type1309, 1
  %"get object context1311" = extractvalue { i32, ptr } %"load flag1307", 1
  %"get bool value1312" = load i1, ptr %"get object context1311", align 1
  %"non bool or true1313" = or i1 %"not bool check1310", %"get bool value1312"
  br i1 %"non bool or true1313", label %primitive-branch34, label %next-block1308

compiled-branch35:                                ; preds = %force63, %after-label66, %thunk65, %force60, %force52, %after-label55, %thunk54, %force49, %force41, %after-label44, %thunk43, %force38, %force30, %after-label33, %thunk32, %next-block1308, %force27, %force24, %primitive-branch17, %compiled-branch18, %force15, %force7, %after-label10, %found775, %force4
  store { i32, ptr } zeroinitializer, ptr %argl, align 8
  %"object value1314" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr (ptr, ptr null, i32 1) to i32))
  store ptr blockaddress(@main, %thunk32), ptr %"object value1314", align 8
  %"insert value object1315" = insertvalue { i32, ptr } { i32 5, ptr null }, ptr %"object value1314", 1
  %"load register env1316" = load { i32, ptr }, ptr %env, align 8
  %"insert into struct1317" = insertvalue { { i32, ptr }, { i32, ptr } } zeroinitializer, { i32, ptr } %"insert value object1315", 0
  %"insert into struct1318" = insertvalue { { i32, ptr }, { i32, ptr } } %"insert into struct1317", { i32, ptr } %"load register env1316", 1
  %"object value1319" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ { i32, ptr }, { i32, ptr } }, ptr null, i32 1) to i32))
  store { { i32, ptr }, { i32, ptr } } %"insert into struct1318", ptr %"object value1319", align 8
  %"insert value object1320" = insertvalue { i32, ptr } { i32 8, ptr null }, ptr %"object value1319", 1
  store { i32, ptr } %"insert value object1320", ptr %val, align 8
  br label %after-label33

thunk32:                                          ; preds = %force63, %after-label66, %thunk65, %force60, %force52, %after-label55, %thunk54, %force49, %force41, %after-label44, %thunk43, %force38, %force30, %after-label33, %thunk32, %next-block1321, %force27, %force24, %primitive-branch17, %compiled-branch18, %force15, %force7, %after-label10, %found775, %force4
  %"load register thunk1322" = load { i32, ptr }, ptr %thunk, align 8
  %"extract-thunk:return1323" = extractvalue { i32, ptr } %"load register thunk1322", 1
  %"extract-thunk:1324" = load { { i32, ptr }, { i32, ptr } }, ptr %"extract-thunk:return1323", align 8
  %"thunk env1325" = extractvalue { { i32, ptr }, { i32, ptr } } %"extract-thunk:1324", 1
  store { i32, ptr } %"thunk env1325", ptr %env, align 8
  %"object value1326" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  store { i32, ptr } { i32 2, ptr @gg }, ptr %"object value1326", align 8
  %"insert value object1327" = insertvalue { i32, ptr } { i32 3, ptr null }, ptr %"object value1326", 1
  store { i32, ptr } %"insert value object1327", ptr %thunk, align 8
  %"load register continue1328" = load { i32, ptr }, ptr %continue, align 8
  %"extract-label:return1329" = extractvalue { i32, ptr } %"load register continue1328", 1
  %"extract-label:1330" = load ptr, ptr %"extract-label:return1329", align 8
  indirectbr ptr %"extract-label:1330", [label %done28, label %after-call36, label %done16, label %after-call47, label %done8, label %compiled-branch35, label %force15, label %done53, label %compiled-branch57, label %after-label66, label %actual-value26, label %true-branch20, label %compiled-branch68, label %actual-value37, label %done50, label %actual-value6, label %add1, label %done61, label %actual-value23, label %primitive-branch34, label %thunk54, label %primitive-branch11, label %compiled-branch12, label %actual-value14, label %actual-value29, label %done31, label %force27, label %compiled-branch18, label %actual-value59, label %thunk32, label %compiled-branch46, label %force41, label %done42, label %after-call19, label %primitive-branch17, label %after-label10, label %actual-value51, label %after-label44, label %force52, label %thunk43, label %force4, label %primitive-branch45, label %force60, label %force7, label %force24, label %done39, label %actual-value40, label %primitive-branch67, label %thunk65, label %after-label55, label %force49, label %actual-value62, label %after-call13, label %after-if22, label %entry1, label %done64, label %primitive-branch56, label %after-label33, label %force30, label %done5, label %done25, label %thunk9, label %actual-value48, label %after-lambda2, label %false-branch21, label %actual-value3, label %after-call58, label %x, label %force63, label %after-call69, label %force38]

after-label33:                                    ; preds = %force63, %after-label66, %thunk65, %force60, %force52, %after-label55, %thunk54, %force49, %force41, %after-label44, %thunk43, %force38, %force30, %after-label33, %next-block1331, %thunk32, %compiled-branch35, %force27, %force24, %primitive-branch17, %compiled-branch18, %force15, %force7, %after-label10, %found775, %force4
  %"load register val1332" = load { i32, ptr }, ptr %val, align 8
  %"load register argl1333" = load { i32, ptr }, ptr %argl, align 8
  %"car ptr1334" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  %"cdr ptr1335" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  store { i32, ptr } %"load register val1332", ptr %"car ptr1334", align 8
  store { i32, ptr } %"load register argl1333", ptr %"cdr ptr1335", align 8
  %"insert car - cons1336" = insertvalue { ptr, ptr } zeroinitializer, ptr %"car ptr1334", 0
  %"insert cdr - cons1337" = insertvalue { ptr, ptr } %"insert car - cons1336", ptr %"cdr ptr1335", 1
  %"object value1338" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ ptr, ptr }, ptr null, i32 1) to i32))
  store { ptr, ptr } %"insert cdr - cons1337", ptr %"object value1338", align 8
  %"insert value object1339" = insertvalue { i32, ptr } { i32 6, ptr null }, ptr %"object value1338", 1
  store { i32, ptr } %"insert value object1339", ptr %argl, align 8
  %"object value1340" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr (ptr, ptr null, i32 1) to i32))
  store ptr blockaddress(@main, %after-call36), ptr %"object value1340", align 8
  %"insert value object1341" = insertvalue { i32, ptr } { i32 5, ptr null }, ptr %"object value1340", 1
  store { i32, ptr } %"insert value object1341", ptr %continue, align 8
  %"load register proc1342" = load { i32, ptr }, ptr %proc, align 8
  %"extract-lambda:return1343" = extractvalue { i32, ptr } %"load register proc1342", 1
  %"extract-lambda:1344" = load { { i32, ptr }, { i32, ptr } }, ptr %"extract-lambda:return1343", align 8
  %"proc entry1345" = extractvalue { { i32, ptr }, { i32, ptr } } %"extract-lambda:1344", 0
  store { i32, ptr } %"proc entry1345", ptr %val, align 8
  %"load register val1346" = load { i32, ptr }, ptr %val, align 8
  %"extract-label:return1347" = extractvalue { i32, ptr } %"load register val1346", 1
  %"extract-label:1348" = load ptr, ptr %"extract-label:return1347", align 8
  indirectbr ptr %"extract-label:1348", [label %done28, label %after-call36, label %done16, label %after-call47, label %done8, label %compiled-branch35, label %force15, label %done53, label %compiled-branch57, label %after-label66, label %actual-value26, label %true-branch20, label %compiled-branch68, label %actual-value37, label %done50, label %actual-value6, label %add1, label %done61, label %actual-value23, label %primitive-branch34, label %thunk54, label %primitive-branch11, label %compiled-branch12, label %actual-value14, label %actual-value29, label %done31, label %force27, label %compiled-branch18, label %actual-value59, label %thunk32, label %compiled-branch46, label %force41, label %done42, label %after-call19, label %primitive-branch17, label %after-label10, label %actual-value51, label %after-label44, label %force52, label %thunk43, label %force4, label %primitive-branch45, label %force60, label %force7, label %force24, label %done39, label %actual-value40, label %primitive-branch67, label %thunk65, label %after-label55, label %force49, label %actual-value62, label %after-call13, label %after-if22, label %entry1, label %done64, label %primitive-branch56, label %after-label33, label %force30, label %done5, label %done25, label %thunk9, label %actual-value48, label %after-lambda2, label %false-branch21, label %actual-value3, label %after-call58, label %x, label %force63, label %after-call69, label %force38]

primitive-branch34:                               ; preds = %force63, %after-label66, %thunk65, %force60, %force52, %after-label55, %thunk54, %force49, %force41, %after-label44, %thunk43, %force38, %force30, %next-block1349, %after-label33, %thunk32, %done28, %force27, %force24, %primitive-branch17, %compiled-branch18, %force15, %force7, %after-label10, %found775, %force4
  %"load stack1350" = load { { i32, ptr }, ptr }, ptr %stack, align 8
  %"previous stack1351" = alloca { { i32, ptr }, ptr }, align 8
  store { { i32, ptr }, ptr } %"load stack1350", ptr %"previous stack1351", align 8
  %"load register1352" = load { i32, ptr }, ptr %proc, align 8
  %"save register1353" = insertvalue { { i32, ptr }, ptr } zeroinitializer, { i32, ptr } %"load register1352", 0
  %"save previous stack1354" = insertvalue { { i32, ptr }, ptr } %"save register1353", ptr %"previous stack1351", 1
  store { { i32, ptr }, ptr } %"save previous stack1354", ptr %stack, align 8
  store { i32, ptr } zeroinitializer, ptr %argl, align 8
  %"load stack1355" = load { { i32, ptr }, ptr }, ptr %stack, align 8
  %"previous stack1356" = alloca { { i32, ptr }, ptr }, align 8
  store { { i32, ptr }, ptr } %"load stack1355", ptr %"previous stack1356", align 8
  %"load register1357" = load { i32, ptr }, ptr %argl, align 8
  %"save register1358" = insertvalue { { i32, ptr }, ptr } zeroinitializer, { i32, ptr } %"load register1357", 0
  %"save previous stack1359" = insertvalue { { i32, ptr }, ptr } %"save register1358", ptr %"previous stack1356", 1
  store { { i32, ptr }, ptr } %"save previous stack1359", ptr %stack, align 8
  %"object value1360" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  store { i32, ptr } { i32 2, ptr @gg.212 }, ptr %"object value1360", align 8
  %"insert value object1361" = insertvalue { i32, ptr } { i32 3, ptr null }, ptr %"object value1360", 1
  store { i32, ptr } %"insert value object1361", ptr %thunk, align 8
  br label %actual-value29

actual-value29:                                   ; preds = %force63, %after-label66, %thunk65, %force60, %force52, %after-label55, %thunk54, %force49, %force41, %after-label44, %thunk43, %force38, %force30, %primitive-branch34, %after-label33, %thunk32, %force27, %force24, %primitive-branch17, %compiled-branch18, %force15, %force7, %after-label10, %found775, %force4
  %"load register thunk1362" = load { i32, ptr }, ptr %thunk, align 8
  %get_type1363 = extractvalue { i32, ptr } %"load register thunk1362", 0
  %"is hempty1364" = icmp eq i32 %get_type1363, 8
  %"not thunk1365" = xor i1 %"is hempty1364", true
  %"object value1366" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr (i1, ptr null, i32 1) to i32))
  store i1 %"not thunk1365", ptr %"object value1366", align 1
  %"insert value object1367" = insertvalue { i32, ptr } { i32 1, ptr null }, ptr %"object value1366", 1
  store { i32, ptr } %"insert value object1367", ptr %flag, align 8
  %"load flag1368" = load { i32, ptr }, ptr %flag, align 8
  %get_type1370 = extractvalue { i32, ptr } %"load flag1368", 0
  %"not bool check1371" = icmp ne i32 %get_type1370, 1
  %"get object context1372" = extractvalue { i32, ptr } %"load flag1368", 1
  %"get bool value1373" = load i1, ptr %"get object context1372", align 1
  %"non bool or true1374" = or i1 %"not bool check1371", %"get bool value1373"
  br i1 %"non bool or true1374", label %done31, label %next-block1369

force30:                                          ; preds = %force63, %after-label66, %thunk65, %force60, %force52, %after-label55, %thunk54, %force49, %force41, %after-label44, %thunk43, %force38, %force30, %next-block1369, %after-label33, %thunk32, %force27, %force24, %primitive-branch17, %compiled-branch18, %force15, %force7, %after-label10, %found775, %force4
  %"object value1375" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr (ptr, ptr null, i32 1) to i32))
  store ptr blockaddress(@main, %actual-value29), ptr %"object value1375", align 8
  %"insert value object1376" = insertvalue { i32, ptr } { i32 5, ptr null }, ptr %"object value1375", 1
  store { i32, ptr } %"insert value object1376", ptr %continue, align 8
  %"load register thunk1377" = load { i32, ptr }, ptr %thunk, align 8
  %"extract-thunk:return1378" = extractvalue { i32, ptr } %"load register thunk1377", 1
  %"extract-thunk:1379" = load { { i32, ptr }, { i32, ptr } }, ptr %"extract-thunk:return1378", align 8
  %"thunk entry1380" = extractvalue { { i32, ptr }, { i32, ptr } } %"extract-thunk:1379", 0
  store { i32, ptr } %"thunk entry1380", ptr %val, align 8
  %"load register val1381" = load { i32, ptr }, ptr %val, align 8
  %"extract-label:return1382" = extractvalue { i32, ptr } %"load register val1381", 1
  %"extract-label:1383" = load ptr, ptr %"extract-label:return1382", align 8
  indirectbr ptr %"extract-label:1383", [label %done28, label %after-call36, label %done16, label %after-call47, label %done8, label %compiled-branch35, label %force15, label %done53, label %compiled-branch57, label %after-label66, label %actual-value26, label %true-branch20, label %compiled-branch68, label %actual-value37, label %done50, label %actual-value6, label %add1, label %done61, label %actual-value23, label %primitive-branch34, label %thunk54, label %primitive-branch11, label %compiled-branch12, label %actual-value14, label %actual-value29, label %done31, label %force27, label %compiled-branch18, label %actual-value59, label %thunk32, label %compiled-branch46, label %force41, label %done42, label %after-call19, label %primitive-branch17, label %after-label10, label %actual-value51, label %after-label44, label %force52, label %thunk43, label %force4, label %primitive-branch45, label %force60, label %force7, label %force24, label %done39, label %actual-value40, label %primitive-branch67, label %thunk65, label %after-label55, label %force49, label %actual-value62, label %after-call13, label %after-if22, label %entry1, label %done64, label %primitive-branch56, label %after-label33, label %force30, label %done5, label %done25, label %thunk9, label %actual-value48, label %after-lambda2, label %false-branch21, label %actual-value3, label %after-call58, label %x, label %force63, label %after-call69, label %force38]

done31:                                           ; preds = %force63, %after-label66, %thunk65, %force60, %force52, %after-label55, %thunk54, %force49, %force41, %after-label44, %thunk43, %force38, %next-block1384, %force30, %actual-value29, %after-label33, %thunk32, %force27, %force24, %primitive-branch17, %compiled-branch18, %force15, %force7, %after-label10, %found775, %force4
  %"load register thunk1385" = load { i32, ptr }, ptr %thunk, align 8
  store { i32, ptr } %"load register thunk1385", ptr %val, align 8
  %stack1386 = load { { i32, ptr }, ptr }, ptr %stack, align 8
  %"old stack1387" = extractvalue { { i32, ptr }, ptr } %stack1386, 1
  %"current stack1388" = extractvalue { { i32, ptr }, ptr } %stack1386, 0
  store { i32, ptr } %"current stack1388", ptr %argl, align 8
  %"load previous stack1389" = load { { i32, ptr }, ptr }, ptr %"old stack1387", align 8
  store { { i32, ptr }, ptr } %"load previous stack1389", ptr %stack, align 8
  %"load register val1390" = load { i32, ptr }, ptr %val, align 8
  %"load register argl1391" = load { i32, ptr }, ptr %argl, align 8
  %"car ptr1392" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  %"cdr ptr1393" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  store { i32, ptr } %"load register val1390", ptr %"car ptr1392", align 8
  store { i32, ptr } %"load register argl1391", ptr %"cdr ptr1393", align 8
  %"insert car - cons1394" = insertvalue { ptr, ptr } zeroinitializer, ptr %"car ptr1392", 0
  %"insert cdr - cons1395" = insertvalue { ptr, ptr } %"insert car - cons1394", ptr %"cdr ptr1393", 1
  %"object value1396" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ ptr, ptr }, ptr null, i32 1) to i32))
  store { ptr, ptr } %"insert cdr - cons1395", ptr %"object value1396", align 8
  %"insert value object1397" = insertvalue { i32, ptr } { i32 6, ptr null }, ptr %"object value1396", 1
  store { i32, ptr } %"insert value object1397", ptr %argl, align 8
  %stack1398 = load { { i32, ptr }, ptr }, ptr %stack, align 8
  %"old stack1399" = extractvalue { { i32, ptr }, ptr } %stack1398, 1
  %"current stack1400" = extractvalue { { i32, ptr }, ptr } %stack1398, 0
  store { i32, ptr } %"current stack1400", ptr %proc, align 8
  %"load previous stack1401" = load { { i32, ptr }, ptr }, ptr %"old stack1399", align 8
  store { { i32, ptr }, ptr } %"load previous stack1401", ptr %stack, align 8
  %"load register proc1402" = load { i32, ptr }, ptr %proc, align 8
  %"load register argl1403" = load { i32, ptr }, ptr %argl, align 8
  %"extract-primitive:return1404" = extractvalue { i32, ptr } %"load register proc1402", 1
  %"extract-primitive:1405" = load ptr, ptr %"extract-primitive:return1404", align 8
  %"call primitive1406" = call { i32, ptr } %"extract-primitive:1405"({ i32, ptr } %"load register argl1403")
  store { i32, ptr } %"call primitive1406", ptr %val, align 8
  br label %after-call36

after-call36:                                     ; preds = %force63, %after-label66, %thunk65, %force60, %force52, %after-label55, %thunk54, %force49, %force41, %after-label44, %thunk43, %force38, %done31, %force30, %after-label33, %thunk32, %force27, %force24, %primitive-branch17, %compiled-branch18, %force15, %force7, %after-label10, %found775, %force4
  br label %false-branch21

false-branch21:                                   ; preds = %force63, %after-label66, %thunk65, %force60, %force52, %after-label55, %thunk54, %force49, %force41, %after-label44, %thunk43, %force38, %after-call36, %force30, %after-label33, %thunk32, %force27, %done25, %force24, %primitive-branch17, %compiled-branch18, %force15, %force7, %after-label10, %found775, %force4
  %"load stack1407" = load { { i32, ptr }, ptr }, ptr %stack, align 8
  %"previous stack1408" = alloca { { i32, ptr }, ptr }, align 8
  store { { i32, ptr }, ptr } %"load stack1407", ptr %"previous stack1408", align 8
  %"load register1409" = load { i32, ptr }, ptr %env, align 8
  %"save register1410" = insertvalue { { i32, ptr }, ptr } zeroinitializer, { i32, ptr } %"load register1409", 0
  %"save previous stack1411" = insertvalue { { i32, ptr }, ptr } %"save register1410", ptr %"previous stack1408", 1
  store { { i32, ptr }, ptr } %"save previous stack1411", ptr %stack, align 8
  %"object value1412" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  store { i32, ptr } { i32 5, ptr @print.213 }, ptr %"object value1412", align 8
  %"insert value object1413" = insertvalue { i32, ptr } { i32 4, ptr null }, ptr %"object value1412", 1
  %"load register env1414" = load { i32, ptr }, ptr %env, align 8
  %env1422 = alloca { i32, ptr }, align 8
  store { i32, ptr } %"load register env1414", ptr %env1422, align 8
  br label %lookup-entry1415

actual-value37:                                   ; preds = %force63, %after-label66, %thunk65, %force60, %force52, %after-label55, %thunk54, %force49, %force41, %after-label44, %thunk43, %force38, %found1420, %force30, %after-label33, %thunk32, %force27, %force24, %primitive-branch17, %compiled-branch18, %force15, %force7, %after-label10, %found775, %force4
  %"load register thunk1480" = load { i32, ptr }, ptr %thunk, align 8
  %get_type1481 = extractvalue { i32, ptr } %"load register thunk1480", 0
  %"is hempty1482" = icmp eq i32 %get_type1481, 8
  %"not thunk1483" = xor i1 %"is hempty1482", true
  %"object value1484" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr (i1, ptr null, i32 1) to i32))
  store i1 %"not thunk1483", ptr %"object value1484", align 1
  %"insert value object1485" = insertvalue { i32, ptr } { i32 1, ptr null }, ptr %"object value1484", 1
  store { i32, ptr } %"insert value object1485", ptr %flag, align 8
  %"load flag1486" = load { i32, ptr }, ptr %flag, align 8
  %get_type1488 = extractvalue { i32, ptr } %"load flag1486", 0
  %"not bool check1489" = icmp ne i32 %get_type1488, 1
  %"get object context1490" = extractvalue { i32, ptr } %"load flag1486", 1
  %"get bool value1491" = load i1, ptr %"get object context1490", align 1
  %"non bool or true1492" = or i1 %"not bool check1489", %"get bool value1491"
  br i1 %"non bool or true1492", label %done39, label %next-block1487

force38:                                          ; preds = %force63, %after-label66, %thunk65, %force60, %force52, %after-label55, %thunk54, %force49, %force41, %after-label44, %thunk43, %force38, %next-block1487, %force30, %after-label33, %thunk32, %force27, %force24, %primitive-branch17, %compiled-branch18, %force15, %force7, %after-label10, %found775, %force4
  %"object value1493" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr (ptr, ptr null, i32 1) to i32))
  store ptr blockaddress(@main, %actual-value37), ptr %"object value1493", align 8
  %"insert value object1494" = insertvalue { i32, ptr } { i32 5, ptr null }, ptr %"object value1493", 1
  store { i32, ptr } %"insert value object1494", ptr %continue, align 8
  %"load register thunk1495" = load { i32, ptr }, ptr %thunk, align 8
  %"extract-thunk:return1496" = extractvalue { i32, ptr } %"load register thunk1495", 1
  %"extract-thunk:1497" = load { { i32, ptr }, { i32, ptr } }, ptr %"extract-thunk:return1496", align 8
  %"thunk entry1498" = extractvalue { { i32, ptr }, { i32, ptr } } %"extract-thunk:1497", 0
  store { i32, ptr } %"thunk entry1498", ptr %val, align 8
  %"load register val1499" = load { i32, ptr }, ptr %val, align 8
  %"extract-label:return1500" = extractvalue { i32, ptr } %"load register val1499", 1
  %"extract-label:1501" = load ptr, ptr %"extract-label:return1500", align 8
  indirectbr ptr %"extract-label:1501", [label %done28, label %after-call36, label %done16, label %after-call47, label %done8, label %compiled-branch35, label %force15, label %done53, label %compiled-branch57, label %after-label66, label %actual-value26, label %true-branch20, label %compiled-branch68, label %actual-value37, label %done50, label %actual-value6, label %add1, label %done61, label %actual-value23, label %primitive-branch34, label %thunk54, label %primitive-branch11, label %compiled-branch12, label %actual-value14, label %actual-value29, label %done31, label %force27, label %compiled-branch18, label %actual-value59, label %thunk32, label %compiled-branch46, label %force41, label %done42, label %after-call19, label %primitive-branch17, label %after-label10, label %actual-value51, label %after-label44, label %force52, label %thunk43, label %force4, label %primitive-branch45, label %force60, label %force7, label %force24, label %done39, label %actual-value40, label %primitive-branch67, label %thunk65, label %after-label55, label %force49, label %actual-value62, label %after-call13, label %after-if22, label %entry1, label %done64, label %primitive-branch56, label %after-label33, label %force30, label %done5, label %done25, label %thunk9, label %actual-value48, label %after-lambda2, label %false-branch21, label %actual-value3, label %after-call58, label %x, label %force63, label %after-call69, label %force38]

done39:                                           ; preds = %force63, %after-label66, %thunk65, %force60, %force52, %after-label55, %thunk54, %force49, %force41, %after-label44, %thunk43, %next-block1502, %force38, %actual-value37, %force30, %after-label33, %thunk32, %force27, %force24, %primitive-branch17, %compiled-branch18, %force15, %force7, %after-label10, %found775, %force4
  %"load register thunk1503" = load { i32, ptr }, ptr %thunk, align 8
  store { i32, ptr } %"load register thunk1503", ptr %proc, align 8
  %stack1504 = load { { i32, ptr }, ptr }, ptr %stack, align 8
  %"old stack1505" = extractvalue { { i32, ptr }, ptr } %stack1504, 1
  %"current stack1506" = extractvalue { { i32, ptr }, ptr } %stack1504, 0
  store { i32, ptr } %"current stack1506", ptr %env, align 8
  %"load previous stack1507" = load { { i32, ptr }, ptr }, ptr %"old stack1505", align 8
  store { { i32, ptr }, ptr } %"load previous stack1507", ptr %stack, align 8
  %"load register proc1508" = load { i32, ptr }, ptr %proc, align 8
  %get_type1509 = extractvalue { i32, ptr } %"load register proc1508", 0
  %"is hempty1510" = icmp eq i32 %get_type1509, 7
  %"object value1511" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr (i1, ptr null, i32 1) to i32))
  store i1 %"is hempty1510", ptr %"object value1511", align 1
  %"insert value object1512" = insertvalue { i32, ptr } { i32 1, ptr null }, ptr %"object value1511", 1
  store { i32, ptr } %"insert value object1512", ptr %flag, align 8
  %"load flag1513" = load { i32, ptr }, ptr %flag, align 8
  %get_type1515 = extractvalue { i32, ptr } %"load flag1513", 0
  %"not bool check1516" = icmp ne i32 %get_type1515, 1
  %"get object context1517" = extractvalue { i32, ptr } %"load flag1513", 1
  %"get bool value1518" = load i1, ptr %"get object context1517", align 1
  %"non bool or true1519" = or i1 %"not bool check1516", %"get bool value1518"
  br i1 %"non bool or true1519", label %primitive-branch45, label %next-block1514

compiled-branch46:                                ; preds = %force63, %after-label66, %thunk65, %force60, %force52, %after-label55, %thunk54, %force49, %force41, %after-label44, %thunk43, %next-block1514, %force38, %force30, %after-label33, %thunk32, %force27, %force24, %primitive-branch17, %compiled-branch18, %force15, %force7, %after-label10, %found775, %force4
  store { i32, ptr } zeroinitializer, ptr %argl, align 8
  %"object value1520" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr (ptr, ptr null, i32 1) to i32))
  store ptr blockaddress(@main, %thunk43), ptr %"object value1520", align 8
  %"insert value object1521" = insertvalue { i32, ptr } { i32 5, ptr null }, ptr %"object value1520", 1
  %"load register env1522" = load { i32, ptr }, ptr %env, align 8
  %"insert into struct1523" = insertvalue { { i32, ptr }, { i32, ptr } } zeroinitializer, { i32, ptr } %"insert value object1521", 0
  %"insert into struct1524" = insertvalue { { i32, ptr }, { i32, ptr } } %"insert into struct1523", { i32, ptr } %"load register env1522", 1
  %"object value1525" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ { i32, ptr }, { i32, ptr } }, ptr null, i32 1) to i32))
  store { { i32, ptr }, { i32, ptr } } %"insert into struct1524", ptr %"object value1525", align 8
  %"insert value object1526" = insertvalue { i32, ptr } { i32 8, ptr null }, ptr %"object value1525", 1
  store { i32, ptr } %"insert value object1526", ptr %val, align 8
  br label %after-label44

thunk43:                                          ; preds = %force63, %after-label66, %thunk65, %force60, %force52, %after-label55, %thunk54, %force49, %force41, %after-label44, %thunk43, %next-block1527, %force38, %force30, %after-label33, %thunk32, %force27, %force24, %primitive-branch17, %compiled-branch18, %force15, %force7, %after-label10, %found775, %force4
  %"load register thunk1528" = load { i32, ptr }, ptr %thunk, align 8
  %"extract-thunk:return1529" = extractvalue { i32, ptr } %"load register thunk1528", 1
  %"extract-thunk:1530" = load { { i32, ptr }, { i32, ptr } }, ptr %"extract-thunk:return1529", align 8
  %"thunk env1531" = extractvalue { { i32, ptr }, { i32, ptr } } %"extract-thunk:1530", 1
  store { i32, ptr } %"thunk env1531", ptr %env, align 8
  %"object value1532" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  store { i32, ptr } { i32 2, ptr @hi }, ptr %"object value1532", align 8
  %"insert value object1533" = insertvalue { i32, ptr } { i32 3, ptr null }, ptr %"object value1532", 1
  store { i32, ptr } %"insert value object1533", ptr %thunk, align 8
  %"load register continue1534" = load { i32, ptr }, ptr %continue, align 8
  %"extract-label:return1535" = extractvalue { i32, ptr } %"load register continue1534", 1
  %"extract-label:1536" = load ptr, ptr %"extract-label:return1535", align 8
  indirectbr ptr %"extract-label:1536", [label %done28, label %after-call36, label %done16, label %after-call47, label %done8, label %compiled-branch35, label %force15, label %done53, label %compiled-branch57, label %after-label66, label %actual-value26, label %true-branch20, label %compiled-branch68, label %actual-value37, label %done50, label %actual-value6, label %add1, label %done61, label %actual-value23, label %primitive-branch34, label %thunk54, label %primitive-branch11, label %compiled-branch12, label %actual-value14, label %actual-value29, label %done31, label %force27, label %compiled-branch18, label %actual-value59, label %thunk32, label %compiled-branch46, label %force41, label %done42, label %after-call19, label %primitive-branch17, label %after-label10, label %actual-value51, label %after-label44, label %force52, label %thunk43, label %force4, label %primitive-branch45, label %force60, label %force7, label %force24, label %done39, label %actual-value40, label %primitive-branch67, label %thunk65, label %after-label55, label %force49, label %actual-value62, label %after-call13, label %after-if22, label %entry1, label %done64, label %primitive-branch56, label %after-label33, label %force30, label %done5, label %done25, label %thunk9, label %actual-value48, label %after-lambda2, label %false-branch21, label %actual-value3, label %after-call58, label %x, label %force63, label %after-call69, label %force38]

after-label44:                                    ; preds = %force63, %after-label66, %thunk65, %force60, %force52, %after-label55, %thunk54, %force49, %force41, %after-label44, %next-block1537, %thunk43, %compiled-branch46, %force38, %force30, %after-label33, %thunk32, %force27, %force24, %primitive-branch17, %compiled-branch18, %force15, %force7, %after-label10, %found775, %force4
  %"load register val1538" = load { i32, ptr }, ptr %val, align 8
  %"load register argl1539" = load { i32, ptr }, ptr %argl, align 8
  %"car ptr1540" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  %"cdr ptr1541" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  store { i32, ptr } %"load register val1538", ptr %"car ptr1540", align 8
  store { i32, ptr } %"load register argl1539", ptr %"cdr ptr1541", align 8
  %"insert car - cons1542" = insertvalue { ptr, ptr } zeroinitializer, ptr %"car ptr1540", 0
  %"insert cdr - cons1543" = insertvalue { ptr, ptr } %"insert car - cons1542", ptr %"cdr ptr1541", 1
  %"object value1544" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ ptr, ptr }, ptr null, i32 1) to i32))
  store { ptr, ptr } %"insert cdr - cons1543", ptr %"object value1544", align 8
  %"insert value object1545" = insertvalue { i32, ptr } { i32 6, ptr null }, ptr %"object value1544", 1
  store { i32, ptr } %"insert value object1545", ptr %argl, align 8
  %"object value1546" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr (ptr, ptr null, i32 1) to i32))
  store ptr blockaddress(@main, %after-if22), ptr %"object value1546", align 8
  %"insert value object1547" = insertvalue { i32, ptr } { i32 5, ptr null }, ptr %"object value1546", 1
  store { i32, ptr } %"insert value object1547", ptr %continue, align 8
  %"load register proc1548" = load { i32, ptr }, ptr %proc, align 8
  %"extract-lambda:return1549" = extractvalue { i32, ptr } %"load register proc1548", 1
  %"extract-lambda:1550" = load { { i32, ptr }, { i32, ptr } }, ptr %"extract-lambda:return1549", align 8
  %"proc entry1551" = extractvalue { { i32, ptr }, { i32, ptr } } %"extract-lambda:1550", 0
  store { i32, ptr } %"proc entry1551", ptr %val, align 8
  %"load register val1552" = load { i32, ptr }, ptr %val, align 8
  %"extract-label:return1553" = extractvalue { i32, ptr } %"load register val1552", 1
  %"extract-label:1554" = load ptr, ptr %"extract-label:return1553", align 8
  indirectbr ptr %"extract-label:1554", [label %done28, label %after-call36, label %done16, label %after-call47, label %done8, label %compiled-branch35, label %force15, label %done53, label %compiled-branch57, label %after-label66, label %actual-value26, label %true-branch20, label %compiled-branch68, label %actual-value37, label %done50, label %actual-value6, label %add1, label %done61, label %actual-value23, label %primitive-branch34, label %thunk54, label %primitive-branch11, label %compiled-branch12, label %actual-value14, label %actual-value29, label %done31, label %force27, label %compiled-branch18, label %actual-value59, label %thunk32, label %compiled-branch46, label %force41, label %done42, label %after-call19, label %primitive-branch17, label %after-label10, label %actual-value51, label %after-label44, label %force52, label %thunk43, label %force4, label %primitive-branch45, label %force60, label %force7, label %force24, label %done39, label %actual-value40, label %primitive-branch67, label %thunk65, label %after-label55, label %force49, label %actual-value62, label %after-call13, label %after-if22, label %entry1, label %done64, label %primitive-branch56, label %after-label33, label %force30, label %done5, label %done25, label %thunk9, label %actual-value48, label %after-lambda2, label %false-branch21, label %actual-value3, label %after-call58, label %x, label %force63, label %after-call69, label %force38]

primitive-branch45:                               ; preds = %force63, %after-label66, %thunk65, %force60, %force52, %after-label55, %thunk54, %force49, %force41, %next-block1555, %after-label44, %thunk43, %done39, %force38, %force30, %after-label33, %thunk32, %force27, %force24, %primitive-branch17, %compiled-branch18, %force15, %force7, %after-label10, %found775, %force4
  %"load stack1556" = load { { i32, ptr }, ptr }, ptr %stack, align 8
  %"previous stack1557" = alloca { { i32, ptr }, ptr }, align 8
  store { { i32, ptr }, ptr } %"load stack1556", ptr %"previous stack1557", align 8
  %"load register1558" = load { i32, ptr }, ptr %proc, align 8
  %"save register1559" = insertvalue { { i32, ptr }, ptr } zeroinitializer, { i32, ptr } %"load register1558", 0
  %"save previous stack1560" = insertvalue { { i32, ptr }, ptr } %"save register1559", ptr %"previous stack1557", 1
  store { { i32, ptr }, ptr } %"save previous stack1560", ptr %stack, align 8
  store { i32, ptr } zeroinitializer, ptr %argl, align 8
  %"load stack1561" = load { { i32, ptr }, ptr }, ptr %stack, align 8
  %"previous stack1562" = alloca { { i32, ptr }, ptr }, align 8
  store { { i32, ptr }, ptr } %"load stack1561", ptr %"previous stack1562", align 8
  %"load register1563" = load { i32, ptr }, ptr %argl, align 8
  %"save register1564" = insertvalue { { i32, ptr }, ptr } zeroinitializer, { i32, ptr } %"load register1563", 0
  %"save previous stack1565" = insertvalue { { i32, ptr }, ptr } %"save register1564", ptr %"previous stack1562", 1
  store { { i32, ptr }, ptr } %"save previous stack1565", ptr %stack, align 8
  %"object value1566" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  store { i32, ptr } { i32 2, ptr @hi.215 }, ptr %"object value1566", align 8
  %"insert value object1567" = insertvalue { i32, ptr } { i32 3, ptr null }, ptr %"object value1566", 1
  store { i32, ptr } %"insert value object1567", ptr %thunk, align 8
  br label %actual-value40

actual-value40:                                   ; preds = %force63, %after-label66, %thunk65, %force60, %force52, %after-label55, %thunk54, %force49, %force41, %primitive-branch45, %after-label44, %thunk43, %force38, %force30, %after-label33, %thunk32, %force27, %force24, %primitive-branch17, %compiled-branch18, %force15, %force7, %after-label10, %found775, %force4
  %"load register thunk1568" = load { i32, ptr }, ptr %thunk, align 8
  %get_type1569 = extractvalue { i32, ptr } %"load register thunk1568", 0
  %"is hempty1570" = icmp eq i32 %get_type1569, 8
  %"not thunk1571" = xor i1 %"is hempty1570", true
  %"object value1572" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr (i1, ptr null, i32 1) to i32))
  store i1 %"not thunk1571", ptr %"object value1572", align 1
  %"insert value object1573" = insertvalue { i32, ptr } { i32 1, ptr null }, ptr %"object value1572", 1
  store { i32, ptr } %"insert value object1573", ptr %flag, align 8
  %"load flag1574" = load { i32, ptr }, ptr %flag, align 8
  %get_type1576 = extractvalue { i32, ptr } %"load flag1574", 0
  %"not bool check1577" = icmp ne i32 %get_type1576, 1
  %"get object context1578" = extractvalue { i32, ptr } %"load flag1574", 1
  %"get bool value1579" = load i1, ptr %"get object context1578", align 1
  %"non bool or true1580" = or i1 %"not bool check1577", %"get bool value1579"
  br i1 %"non bool or true1580", label %done42, label %next-block1575

force41:                                          ; preds = %force63, %after-label66, %thunk65, %force60, %force52, %after-label55, %thunk54, %force49, %force41, %next-block1575, %after-label44, %thunk43, %force38, %force30, %after-label33, %thunk32, %force27, %force24, %primitive-branch17, %compiled-branch18, %force15, %force7, %after-label10, %found775, %force4
  %"object value1581" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr (ptr, ptr null, i32 1) to i32))
  store ptr blockaddress(@main, %actual-value40), ptr %"object value1581", align 8
  %"insert value object1582" = insertvalue { i32, ptr } { i32 5, ptr null }, ptr %"object value1581", 1
  store { i32, ptr } %"insert value object1582", ptr %continue, align 8
  %"load register thunk1583" = load { i32, ptr }, ptr %thunk, align 8
  %"extract-thunk:return1584" = extractvalue { i32, ptr } %"load register thunk1583", 1
  %"extract-thunk:1585" = load { { i32, ptr }, { i32, ptr } }, ptr %"extract-thunk:return1584", align 8
  %"thunk entry1586" = extractvalue { { i32, ptr }, { i32, ptr } } %"extract-thunk:1585", 0
  store { i32, ptr } %"thunk entry1586", ptr %val, align 8
  %"load register val1587" = load { i32, ptr }, ptr %val, align 8
  %"extract-label:return1588" = extractvalue { i32, ptr } %"load register val1587", 1
  %"extract-label:1589" = load ptr, ptr %"extract-label:return1588", align 8
  indirectbr ptr %"extract-label:1589", [label %done28, label %after-call36, label %done16, label %after-call47, label %done8, label %compiled-branch35, label %force15, label %done53, label %compiled-branch57, label %after-label66, label %actual-value26, label %true-branch20, label %compiled-branch68, label %actual-value37, label %done50, label %actual-value6, label %add1, label %done61, label %actual-value23, label %primitive-branch34, label %thunk54, label %primitive-branch11, label %compiled-branch12, label %actual-value14, label %actual-value29, label %done31, label %force27, label %compiled-branch18, label %actual-value59, label %thunk32, label %compiled-branch46, label %force41, label %done42, label %after-call19, label %primitive-branch17, label %after-label10, label %actual-value51, label %after-label44, label %force52, label %thunk43, label %force4, label %primitive-branch45, label %force60, label %force7, label %force24, label %done39, label %actual-value40, label %primitive-branch67, label %thunk65, label %after-label55, label %force49, label %actual-value62, label %after-call13, label %after-if22, label %entry1, label %done64, label %primitive-branch56, label %after-label33, label %force30, label %done5, label %done25, label %thunk9, label %actual-value48, label %after-lambda2, label %false-branch21, label %actual-value3, label %after-call58, label %x, label %force63, label %after-call69, label %force38]

done42:                                           ; preds = %force63, %after-label66, %thunk65, %force60, %force52, %after-label55, %thunk54, %force49, %next-block1590, %force41, %actual-value40, %after-label44, %thunk43, %force38, %force30, %after-label33, %thunk32, %force27, %force24, %primitive-branch17, %compiled-branch18, %force15, %force7, %after-label10, %found775, %force4
  %"load register thunk1591" = load { i32, ptr }, ptr %thunk, align 8
  store { i32, ptr } %"load register thunk1591", ptr %val, align 8
  %stack1592 = load { { i32, ptr }, ptr }, ptr %stack, align 8
  %"old stack1593" = extractvalue { { i32, ptr }, ptr } %stack1592, 1
  %"current stack1594" = extractvalue { { i32, ptr }, ptr } %stack1592, 0
  store { i32, ptr } %"current stack1594", ptr %argl, align 8
  %"load previous stack1595" = load { { i32, ptr }, ptr }, ptr %"old stack1593", align 8
  store { { i32, ptr }, ptr } %"load previous stack1595", ptr %stack, align 8
  %"load register val1596" = load { i32, ptr }, ptr %val, align 8
  %"load register argl1597" = load { i32, ptr }, ptr %argl, align 8
  %"car ptr1598" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  %"cdr ptr1599" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  store { i32, ptr } %"load register val1596", ptr %"car ptr1598", align 8
  store { i32, ptr } %"load register argl1597", ptr %"cdr ptr1599", align 8
  %"insert car - cons1600" = insertvalue { ptr, ptr } zeroinitializer, ptr %"car ptr1598", 0
  %"insert cdr - cons1601" = insertvalue { ptr, ptr } %"insert car - cons1600", ptr %"cdr ptr1599", 1
  %"object value1602" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ ptr, ptr }, ptr null, i32 1) to i32))
  store { ptr, ptr } %"insert cdr - cons1601", ptr %"object value1602", align 8
  %"insert value object1603" = insertvalue { i32, ptr } { i32 6, ptr null }, ptr %"object value1602", 1
  store { i32, ptr } %"insert value object1603", ptr %argl, align 8
  %stack1604 = load { { i32, ptr }, ptr }, ptr %stack, align 8
  %"old stack1605" = extractvalue { { i32, ptr }, ptr } %stack1604, 1
  %"current stack1606" = extractvalue { { i32, ptr }, ptr } %stack1604, 0
  store { i32, ptr } %"current stack1606", ptr %proc, align 8
  %"load previous stack1607" = load { { i32, ptr }, ptr }, ptr %"old stack1605", align 8
  store { { i32, ptr }, ptr } %"load previous stack1607", ptr %stack, align 8
  %"load register proc1608" = load { i32, ptr }, ptr %proc, align 8
  %"load register argl1609" = load { i32, ptr }, ptr %argl, align 8
  %"extract-primitive:return1610" = extractvalue { i32, ptr } %"load register proc1608", 1
  %"extract-primitive:1611" = load ptr, ptr %"extract-primitive:return1610", align 8
  %"call primitive1612" = call { i32, ptr } %"extract-primitive:1611"({ i32, ptr } %"load register argl1609")
  store { i32, ptr } %"call primitive1612", ptr %val, align 8
  br label %after-if22

after-call47:                                     ; preds = %force63, %after-label66, %thunk65, %force60, %force52, %after-label55, %thunk54, %force49, %next-block1613, %force41, %after-label44, %thunk43, %force38, %force30, %after-label33, %thunk32, %force27, %force24, %primitive-branch17, %compiled-branch18, %force15, %force7, %after-label10, %found775, %force4
  br label %after-if22

after-if22:                                       ; preds = %force63, %after-label66, %thunk65, %force60, %force52, %after-label55, %thunk54, %force49, %after-call47, %done42, %force41, %after-label44, %thunk43, %force38, %force30, %after-label33, %thunk32, %force27, %force24, %primitive-branch17, %compiled-branch18, %force15, %force7, %after-label10, %found775, %force4
  %"load stack1614" = load { { i32, ptr }, ptr }, ptr %stack, align 8
  %"previous stack1615" = alloca { { i32, ptr }, ptr }, align 8
  store { { i32, ptr }, ptr } %"load stack1614", ptr %"previous stack1615", align 8
  %"load register1616" = load { i32, ptr }, ptr %env, align 8
  %"save register1617" = insertvalue { { i32, ptr }, ptr } zeroinitializer, { i32, ptr } %"load register1616", 0
  %"save previous stack1618" = insertvalue { { i32, ptr }, ptr } %"save register1617", ptr %"previous stack1615", 1
  store { { i32, ptr }, ptr } %"save previous stack1618", ptr %stack, align 8
  %"object value1619" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  store { i32, ptr } { i32 7, ptr @println.216 }, ptr %"object value1619", align 8
  %"insert value object1620" = insertvalue { i32, ptr } { i32 4, ptr null }, ptr %"object value1619", 1
  %"load register env1621" = load { i32, ptr }, ptr %env, align 8
  %env1629 = alloca { i32, ptr }, align 8
  store { i32, ptr } %"load register env1621", ptr %env1629, align 8
  br label %lookup-entry1622

actual-value48:                                   ; preds = %force63, %after-label66, %thunk65, %force60, %force52, %after-label55, %thunk54, %force49, %found1627, %force41, %after-label44, %thunk43, %force38, %force30, %after-label33, %thunk32, %force27, %force24, %primitive-branch17, %compiled-branch18, %force15, %force7, %after-label10, %found775, %force4
  %"load register thunk1687" = load { i32, ptr }, ptr %thunk, align 8
  %get_type1688 = extractvalue { i32, ptr } %"load register thunk1687", 0
  %"is hempty1689" = icmp eq i32 %get_type1688, 8
  %"not thunk1690" = xor i1 %"is hempty1689", true
  %"object value1691" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr (i1, ptr null, i32 1) to i32))
  store i1 %"not thunk1690", ptr %"object value1691", align 1
  %"insert value object1692" = insertvalue { i32, ptr } { i32 1, ptr null }, ptr %"object value1691", 1
  store { i32, ptr } %"insert value object1692", ptr %flag, align 8
  %"load flag1693" = load { i32, ptr }, ptr %flag, align 8
  %get_type1695 = extractvalue { i32, ptr } %"load flag1693", 0
  %"not bool check1696" = icmp ne i32 %get_type1695, 1
  %"get object context1697" = extractvalue { i32, ptr } %"load flag1693", 1
  %"get bool value1698" = load i1, ptr %"get object context1697", align 1
  %"non bool or true1699" = or i1 %"not bool check1696", %"get bool value1698"
  br i1 %"non bool or true1699", label %done50, label %next-block1694

force49:                                          ; preds = %force63, %after-label66, %thunk65, %force60, %force52, %after-label55, %thunk54, %force49, %next-block1694, %force41, %after-label44, %thunk43, %force38, %force30, %after-label33, %thunk32, %force27, %force24, %primitive-branch17, %compiled-branch18, %force15, %force7, %after-label10, %found775, %force4
  %"object value1700" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr (ptr, ptr null, i32 1) to i32))
  store ptr blockaddress(@main, %actual-value48), ptr %"object value1700", align 8
  %"insert value object1701" = insertvalue { i32, ptr } { i32 5, ptr null }, ptr %"object value1700", 1
  store { i32, ptr } %"insert value object1701", ptr %continue, align 8
  %"load register thunk1702" = load { i32, ptr }, ptr %thunk, align 8
  %"extract-thunk:return1703" = extractvalue { i32, ptr } %"load register thunk1702", 1
  %"extract-thunk:1704" = load { { i32, ptr }, { i32, ptr } }, ptr %"extract-thunk:return1703", align 8
  %"thunk entry1705" = extractvalue { { i32, ptr }, { i32, ptr } } %"extract-thunk:1704", 0
  store { i32, ptr } %"thunk entry1705", ptr %val, align 8
  %"load register val1706" = load { i32, ptr }, ptr %val, align 8
  %"extract-label:return1707" = extractvalue { i32, ptr } %"load register val1706", 1
  %"extract-label:1708" = load ptr, ptr %"extract-label:return1707", align 8
  indirectbr ptr %"extract-label:1708", [label %done28, label %after-call36, label %done16, label %after-call47, label %done8, label %compiled-branch35, label %force15, label %done53, label %compiled-branch57, label %after-label66, label %actual-value26, label %true-branch20, label %compiled-branch68, label %actual-value37, label %done50, label %actual-value6, label %add1, label %done61, label %actual-value23, label %primitive-branch34, label %thunk54, label %primitive-branch11, label %compiled-branch12, label %actual-value14, label %actual-value29, label %done31, label %force27, label %compiled-branch18, label %actual-value59, label %thunk32, label %compiled-branch46, label %force41, label %done42, label %after-call19, label %primitive-branch17, label %after-label10, label %actual-value51, label %after-label44, label %force52, label %thunk43, label %force4, label %primitive-branch45, label %force60, label %force7, label %force24, label %done39, label %actual-value40, label %primitive-branch67, label %thunk65, label %after-label55, label %force49, label %actual-value62, label %after-call13, label %after-if22, label %entry1, label %done64, label %primitive-branch56, label %after-label33, label %force30, label %done5, label %done25, label %thunk9, label %actual-value48, label %after-lambda2, label %false-branch21, label %actual-value3, label %after-call58, label %x, label %force63, label %after-call69, label %force38]

done50:                                           ; preds = %force63, %after-label66, %thunk65, %force60, %force52, %after-label55, %thunk54, %next-block1709, %force49, %actual-value48, %force41, %after-label44, %thunk43, %force38, %force30, %after-label33, %thunk32, %force27, %force24, %primitive-branch17, %compiled-branch18, %force15, %force7, %after-label10, %found775, %force4
  %"load register thunk1710" = load { i32, ptr }, ptr %thunk, align 8
  store { i32, ptr } %"load register thunk1710", ptr %proc, align 8
  %stack1711 = load { { i32, ptr }, ptr }, ptr %stack, align 8
  %"old stack1712" = extractvalue { { i32, ptr }, ptr } %stack1711, 1
  %"current stack1713" = extractvalue { { i32, ptr }, ptr } %stack1711, 0
  store { i32, ptr } %"current stack1713", ptr %env, align 8
  %"load previous stack1714" = load { { i32, ptr }, ptr }, ptr %"old stack1712", align 8
  store { { i32, ptr }, ptr } %"load previous stack1714", ptr %stack, align 8
  %"load register proc1715" = load { i32, ptr }, ptr %proc, align 8
  %get_type1716 = extractvalue { i32, ptr } %"load register proc1715", 0
  %"is hempty1717" = icmp eq i32 %get_type1716, 7
  %"object value1718" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr (i1, ptr null, i32 1) to i32))
  store i1 %"is hempty1717", ptr %"object value1718", align 1
  %"insert value object1719" = insertvalue { i32, ptr } { i32 1, ptr null }, ptr %"object value1718", 1
  store { i32, ptr } %"insert value object1719", ptr %flag, align 8
  %"load flag1720" = load { i32, ptr }, ptr %flag, align 8
  %get_type1722 = extractvalue { i32, ptr } %"load flag1720", 0
  %"not bool check1723" = icmp ne i32 %get_type1722, 1
  %"get object context1724" = extractvalue { i32, ptr } %"load flag1720", 1
  %"get bool value1725" = load i1, ptr %"get object context1724", align 1
  %"non bool or true1726" = or i1 %"not bool check1723", %"get bool value1725"
  br i1 %"non bool or true1726", label %primitive-branch56, label %next-block1721

compiled-branch57:                                ; preds = %force63, %after-label66, %thunk65, %force60, %force52, %after-label55, %thunk54, %next-block1721, %force49, %force41, %after-label44, %thunk43, %force38, %force30, %after-label33, %thunk32, %force27, %force24, %primitive-branch17, %compiled-branch18, %force15, %force7, %after-label10, %found775, %force4
  store { i32, ptr } zeroinitializer, ptr %argl, align 8
  %"object value1727" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr (ptr, ptr null, i32 1) to i32))
  store ptr blockaddress(@main, %thunk54), ptr %"object value1727", align 8
  %"insert value object1728" = insertvalue { i32, ptr } { i32 5, ptr null }, ptr %"object value1727", 1
  %"load register env1729" = load { i32, ptr }, ptr %env, align 8
  %"insert into struct1730" = insertvalue { { i32, ptr }, { i32, ptr } } zeroinitializer, { i32, ptr } %"insert value object1728", 0
  %"insert into struct1731" = insertvalue { { i32, ptr }, { i32, ptr } } %"insert into struct1730", { i32, ptr } %"load register env1729", 1
  %"object value1732" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ { i32, ptr }, { i32, ptr } }, ptr null, i32 1) to i32))
  store { { i32, ptr }, { i32, ptr } } %"insert into struct1731", ptr %"object value1732", align 8
  %"insert value object1733" = insertvalue { i32, ptr } { i32 8, ptr null }, ptr %"object value1732", 1
  store { i32, ptr } %"insert value object1733", ptr %val, align 8
  br label %after-label55

thunk54:                                          ; preds = %force63, %after-label66, %thunk65, %force60, %force52, %after-label55, %thunk54, %next-block1734, %force49, %force41, %after-label44, %thunk43, %force38, %force30, %after-label33, %thunk32, %force27, %force24, %primitive-branch17, %compiled-branch18, %force15, %force7, %after-label10, %found775, %force4
  %"load register thunk1735" = load { i32, ptr }, ptr %thunk, align 8
  %"extract-thunk:return1736" = extractvalue { i32, ptr } %"load register thunk1735", 1
  %"extract-thunk:1737" = load { { i32, ptr }, { i32, ptr } }, ptr %"extract-thunk:return1736", align 8
  %"thunk env1738" = extractvalue { { i32, ptr }, { i32, ptr } } %"extract-thunk:1737", 1
  store { i32, ptr } %"thunk env1738", ptr %env, align 8
  %"object value1739" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  store { i32, ptr } { i32 11, ptr @"not skipped" }, ptr %"object value1739", align 8
  %"insert value object1740" = insertvalue { i32, ptr } { i32 3, ptr null }, ptr %"object value1739", 1
  store { i32, ptr } %"insert value object1740", ptr %thunk, align 8
  %"load register continue1741" = load { i32, ptr }, ptr %continue, align 8
  %"extract-label:return1742" = extractvalue { i32, ptr } %"load register continue1741", 1
  %"extract-label:1743" = load ptr, ptr %"extract-label:return1742", align 8
  indirectbr ptr %"extract-label:1743", [label %done28, label %after-call36, label %done16, label %after-call47, label %done8, label %compiled-branch35, label %force15, label %done53, label %compiled-branch57, label %after-label66, label %actual-value26, label %true-branch20, label %compiled-branch68, label %actual-value37, label %done50, label %actual-value6, label %add1, label %done61, label %actual-value23, label %primitive-branch34, label %thunk54, label %primitive-branch11, label %compiled-branch12, label %actual-value14, label %actual-value29, label %done31, label %force27, label %compiled-branch18, label %actual-value59, label %thunk32, label %compiled-branch46, label %force41, label %done42, label %after-call19, label %primitive-branch17, label %after-label10, label %actual-value51, label %after-label44, label %force52, label %thunk43, label %force4, label %primitive-branch45, label %force60, label %force7, label %force24, label %done39, label %actual-value40, label %primitive-branch67, label %thunk65, label %after-label55, label %force49, label %actual-value62, label %after-call13, label %after-if22, label %entry1, label %done64, label %primitive-branch56, label %after-label33, label %force30, label %done5, label %done25, label %thunk9, label %actual-value48, label %after-lambda2, label %false-branch21, label %actual-value3, label %after-call58, label %x, label %force63, label %after-call69, label %force38]

after-label55:                                    ; preds = %force63, %after-label66, %thunk65, %force60, %force52, %after-label55, %next-block1744, %thunk54, %compiled-branch57, %force49, %force41, %after-label44, %thunk43, %force38, %force30, %after-label33, %thunk32, %force27, %force24, %primitive-branch17, %compiled-branch18, %force15, %force7, %after-label10, %found775, %force4
  %"load register val1745" = load { i32, ptr }, ptr %val, align 8
  %"load register argl1746" = load { i32, ptr }, ptr %argl, align 8
  %"car ptr1747" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  %"cdr ptr1748" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  store { i32, ptr } %"load register val1745", ptr %"car ptr1747", align 8
  store { i32, ptr } %"load register argl1746", ptr %"cdr ptr1748", align 8
  %"insert car - cons1749" = insertvalue { ptr, ptr } zeroinitializer, ptr %"car ptr1747", 0
  %"insert cdr - cons1750" = insertvalue { ptr, ptr } %"insert car - cons1749", ptr %"cdr ptr1748", 1
  %"object value1751" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ ptr, ptr }, ptr null, i32 1) to i32))
  store { ptr, ptr } %"insert cdr - cons1750", ptr %"object value1751", align 8
  %"insert value object1752" = insertvalue { i32, ptr } { i32 6, ptr null }, ptr %"object value1751", 1
  store { i32, ptr } %"insert value object1752", ptr %argl, align 8
  %"object value1753" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr (ptr, ptr null, i32 1) to i32))
  store ptr blockaddress(@main, %after-call58), ptr %"object value1753", align 8
  %"insert value object1754" = insertvalue { i32, ptr } { i32 5, ptr null }, ptr %"object value1753", 1
  store { i32, ptr } %"insert value object1754", ptr %continue, align 8
  %"load register proc1755" = load { i32, ptr }, ptr %proc, align 8
  %"extract-lambda:return1756" = extractvalue { i32, ptr } %"load register proc1755", 1
  %"extract-lambda:1757" = load { { i32, ptr }, { i32, ptr } }, ptr %"extract-lambda:return1756", align 8
  %"proc entry1758" = extractvalue { { i32, ptr }, { i32, ptr } } %"extract-lambda:1757", 0
  store { i32, ptr } %"proc entry1758", ptr %val, align 8
  %"load register val1759" = load { i32, ptr }, ptr %val, align 8
  %"extract-label:return1760" = extractvalue { i32, ptr } %"load register val1759", 1
  %"extract-label:1761" = load ptr, ptr %"extract-label:return1760", align 8
  indirectbr ptr %"extract-label:1761", [label %done28, label %after-call36, label %done16, label %after-call47, label %done8, label %compiled-branch35, label %force15, label %done53, label %compiled-branch57, label %after-label66, label %actual-value26, label %true-branch20, label %compiled-branch68, label %actual-value37, label %done50, label %actual-value6, label %add1, label %done61, label %actual-value23, label %primitive-branch34, label %thunk54, label %primitive-branch11, label %compiled-branch12, label %actual-value14, label %actual-value29, label %done31, label %force27, label %compiled-branch18, label %actual-value59, label %thunk32, label %compiled-branch46, label %force41, label %done42, label %after-call19, label %primitive-branch17, label %after-label10, label %actual-value51, label %after-label44, label %force52, label %thunk43, label %force4, label %primitive-branch45, label %force60, label %force7, label %force24, label %done39, label %actual-value40, label %primitive-branch67, label %thunk65, label %after-label55, label %force49, label %actual-value62, label %after-call13, label %after-if22, label %entry1, label %done64, label %primitive-branch56, label %after-label33, label %force30, label %done5, label %done25, label %thunk9, label %actual-value48, label %after-lambda2, label %false-branch21, label %actual-value3, label %after-call58, label %x, label %force63, label %after-call69, label %force38]

primitive-branch56:                               ; preds = %force63, %after-label66, %thunk65, %force60, %force52, %next-block1762, %after-label55, %thunk54, %done50, %force49, %force41, %after-label44, %thunk43, %force38, %force30, %after-label33, %thunk32, %force27, %force24, %primitive-branch17, %compiled-branch18, %force15, %force7, %after-label10, %found775, %force4
  %"load stack1763" = load { { i32, ptr }, ptr }, ptr %stack, align 8
  %"previous stack1764" = alloca { { i32, ptr }, ptr }, align 8
  store { { i32, ptr }, ptr } %"load stack1763", ptr %"previous stack1764", align 8
  %"load register1765" = load { i32, ptr }, ptr %proc, align 8
  %"save register1766" = insertvalue { { i32, ptr }, ptr } zeroinitializer, { i32, ptr } %"load register1765", 0
  %"save previous stack1767" = insertvalue { { i32, ptr }, ptr } %"save register1766", ptr %"previous stack1764", 1
  store { { i32, ptr }, ptr } %"save previous stack1767", ptr %stack, align 8
  store { i32, ptr } zeroinitializer, ptr %argl, align 8
  %"load stack1768" = load { { i32, ptr }, ptr }, ptr %stack, align 8
  %"previous stack1769" = alloca { { i32, ptr }, ptr }, align 8
  store { { i32, ptr }, ptr } %"load stack1768", ptr %"previous stack1769", align 8
  %"load register1770" = load { i32, ptr }, ptr %argl, align 8
  %"save register1771" = insertvalue { { i32, ptr }, ptr } zeroinitializer, { i32, ptr } %"load register1770", 0
  %"save previous stack1772" = insertvalue { { i32, ptr }, ptr } %"save register1771", ptr %"previous stack1769", 1
  store { { i32, ptr }, ptr } %"save previous stack1772", ptr %stack, align 8
  %"object value1773" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  store { i32, ptr } { i32 11, ptr @"not skipped.218" }, ptr %"object value1773", align 8
  %"insert value object1774" = insertvalue { i32, ptr } { i32 3, ptr null }, ptr %"object value1773", 1
  store { i32, ptr } %"insert value object1774", ptr %thunk, align 8
  br label %actual-value51

actual-value51:                                   ; preds = %force63, %after-label66, %thunk65, %force60, %force52, %primitive-branch56, %after-label55, %thunk54, %force49, %force41, %after-label44, %thunk43, %force38, %force30, %after-label33, %thunk32, %force27, %force24, %primitive-branch17, %compiled-branch18, %force15, %force7, %after-label10, %found775, %force4
  %"load register thunk1775" = load { i32, ptr }, ptr %thunk, align 8
  %get_type1776 = extractvalue { i32, ptr } %"load register thunk1775", 0
  %"is hempty1777" = icmp eq i32 %get_type1776, 8
  %"not thunk1778" = xor i1 %"is hempty1777", true
  %"object value1779" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr (i1, ptr null, i32 1) to i32))
  store i1 %"not thunk1778", ptr %"object value1779", align 1
  %"insert value object1780" = insertvalue { i32, ptr } { i32 1, ptr null }, ptr %"object value1779", 1
  store { i32, ptr } %"insert value object1780", ptr %flag, align 8
  %"load flag1781" = load { i32, ptr }, ptr %flag, align 8
  %get_type1783 = extractvalue { i32, ptr } %"load flag1781", 0
  %"not bool check1784" = icmp ne i32 %get_type1783, 1
  %"get object context1785" = extractvalue { i32, ptr } %"load flag1781", 1
  %"get bool value1786" = load i1, ptr %"get object context1785", align 1
  %"non bool or true1787" = or i1 %"not bool check1784", %"get bool value1786"
  br i1 %"non bool or true1787", label %done53, label %next-block1782

force52:                                          ; preds = %force63, %after-label66, %thunk65, %force60, %force52, %next-block1782, %after-label55, %thunk54, %force49, %force41, %after-label44, %thunk43, %force38, %force30, %after-label33, %thunk32, %force27, %force24, %primitive-branch17, %compiled-branch18, %force15, %force7, %after-label10, %found775, %force4
  %"object value1788" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr (ptr, ptr null, i32 1) to i32))
  store ptr blockaddress(@main, %actual-value51), ptr %"object value1788", align 8
  %"insert value object1789" = insertvalue { i32, ptr } { i32 5, ptr null }, ptr %"object value1788", 1
  store { i32, ptr } %"insert value object1789", ptr %continue, align 8
  %"load register thunk1790" = load { i32, ptr }, ptr %thunk, align 8
  %"extract-thunk:return1791" = extractvalue { i32, ptr } %"load register thunk1790", 1
  %"extract-thunk:1792" = load { { i32, ptr }, { i32, ptr } }, ptr %"extract-thunk:return1791", align 8
  %"thunk entry1793" = extractvalue { { i32, ptr }, { i32, ptr } } %"extract-thunk:1792", 0
  store { i32, ptr } %"thunk entry1793", ptr %val, align 8
  %"load register val1794" = load { i32, ptr }, ptr %val, align 8
  %"extract-label:return1795" = extractvalue { i32, ptr } %"load register val1794", 1
  %"extract-label:1796" = load ptr, ptr %"extract-label:return1795", align 8
  indirectbr ptr %"extract-label:1796", [label %done28, label %after-call36, label %done16, label %after-call47, label %done8, label %compiled-branch35, label %force15, label %done53, label %compiled-branch57, label %after-label66, label %actual-value26, label %true-branch20, label %compiled-branch68, label %actual-value37, label %done50, label %actual-value6, label %add1, label %done61, label %actual-value23, label %primitive-branch34, label %thunk54, label %primitive-branch11, label %compiled-branch12, label %actual-value14, label %actual-value29, label %done31, label %force27, label %compiled-branch18, label %actual-value59, label %thunk32, label %compiled-branch46, label %force41, label %done42, label %after-call19, label %primitive-branch17, label %after-label10, label %actual-value51, label %after-label44, label %force52, label %thunk43, label %force4, label %primitive-branch45, label %force60, label %force7, label %force24, label %done39, label %actual-value40, label %primitive-branch67, label %thunk65, label %after-label55, label %force49, label %actual-value62, label %after-call13, label %after-if22, label %entry1, label %done64, label %primitive-branch56, label %after-label33, label %force30, label %done5, label %done25, label %thunk9, label %actual-value48, label %after-lambda2, label %false-branch21, label %actual-value3, label %after-call58, label %x, label %force63, label %after-call69, label %force38]

done53:                                           ; preds = %force63, %after-label66, %thunk65, %force60, %next-block1797, %force52, %actual-value51, %after-label55, %thunk54, %force49, %force41, %after-label44, %thunk43, %force38, %force30, %after-label33, %thunk32, %force27, %force24, %primitive-branch17, %compiled-branch18, %force15, %force7, %after-label10, %found775, %force4
  %"load register thunk1798" = load { i32, ptr }, ptr %thunk, align 8
  store { i32, ptr } %"load register thunk1798", ptr %val, align 8
  %stack1799 = load { { i32, ptr }, ptr }, ptr %stack, align 8
  %"old stack1800" = extractvalue { { i32, ptr }, ptr } %stack1799, 1
  %"current stack1801" = extractvalue { { i32, ptr }, ptr } %stack1799, 0
  store { i32, ptr } %"current stack1801", ptr %argl, align 8
  %"load previous stack1802" = load { { i32, ptr }, ptr }, ptr %"old stack1800", align 8
  store { { i32, ptr }, ptr } %"load previous stack1802", ptr %stack, align 8
  %"load register val1803" = load { i32, ptr }, ptr %val, align 8
  %"load register argl1804" = load { i32, ptr }, ptr %argl, align 8
  %"car ptr1805" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  %"cdr ptr1806" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  store { i32, ptr } %"load register val1803", ptr %"car ptr1805", align 8
  store { i32, ptr } %"load register argl1804", ptr %"cdr ptr1806", align 8
  %"insert car - cons1807" = insertvalue { ptr, ptr } zeroinitializer, ptr %"car ptr1805", 0
  %"insert cdr - cons1808" = insertvalue { ptr, ptr } %"insert car - cons1807", ptr %"cdr ptr1806", 1
  %"object value1809" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ ptr, ptr }, ptr null, i32 1) to i32))
  store { ptr, ptr } %"insert cdr - cons1808", ptr %"object value1809", align 8
  %"insert value object1810" = insertvalue { i32, ptr } { i32 6, ptr null }, ptr %"object value1809", 1
  store { i32, ptr } %"insert value object1810", ptr %argl, align 8
  %stack1811 = load { { i32, ptr }, ptr }, ptr %stack, align 8
  %"old stack1812" = extractvalue { { i32, ptr }, ptr } %stack1811, 1
  %"current stack1813" = extractvalue { { i32, ptr }, ptr } %stack1811, 0
  store { i32, ptr } %"current stack1813", ptr %proc, align 8
  %"load previous stack1814" = load { { i32, ptr }, ptr }, ptr %"old stack1812", align 8
  store { { i32, ptr }, ptr } %"load previous stack1814", ptr %stack, align 8
  %"load register proc1815" = load { i32, ptr }, ptr %proc, align 8
  %"load register argl1816" = load { i32, ptr }, ptr %argl, align 8
  %"extract-primitive:return1817" = extractvalue { i32, ptr } %"load register proc1815", 1
  %"extract-primitive:1818" = load ptr, ptr %"extract-primitive:return1817", align 8
  %"call primitive1819" = call { i32, ptr } %"extract-primitive:1818"({ i32, ptr } %"load register argl1816")
  store { i32, ptr } %"call primitive1819", ptr %val, align 8
  br label %after-call58

after-call58:                                     ; preds = %force63, %after-label66, %thunk65, %force60, %done53, %force52, %after-label55, %thunk54, %force49, %force41, %after-label44, %thunk43, %force38, %force30, %after-label33, %thunk32, %force27, %force24, %primitive-branch17, %compiled-branch18, %force15, %force7, %after-label10, %found775, %force4
  br label %x

x:                                                ; preds = %force63, %after-label66, %thunk65, %force60, %after-call58, %force52, %after-label55, %thunk54, %force49, %force41, %after-label44, %thunk43, %force38, %force30, %after-label33, %thunk32, %force27, %true-branch20, %force24, %primitive-branch17, %compiled-branch18, %force15, %force7, %after-label10, %found775, %force4
  %"load stack1820" = load { { i32, ptr }, ptr }, ptr %stack, align 8
  %"previous stack1821" = alloca { { i32, ptr }, ptr }, align 8
  store { { i32, ptr }, ptr } %"load stack1820", ptr %"previous stack1821", align 8
  %"load register1822" = load { i32, ptr }, ptr %env, align 8
  %"save register1823" = insertvalue { { i32, ptr }, ptr } zeroinitializer, { i32, ptr } %"load register1822", 0
  %"save previous stack1824" = insertvalue { { i32, ptr }, ptr } %"save register1823", ptr %"previous stack1821", 1
  store { { i32, ptr }, ptr } %"save previous stack1824", ptr %stack, align 8
  %"object value1825" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  store { i32, ptr } { i32 7, ptr @println.219 }, ptr %"object value1825", align 8
  %"insert value object1826" = insertvalue { i32, ptr } { i32 4, ptr null }, ptr %"object value1825", 1
  %"load register env1827" = load { i32, ptr }, ptr %env, align 8
  %env1835 = alloca { i32, ptr }, align 8
  store { i32, ptr } %"load register env1827", ptr %env1835, align 8
  br label %lookup-entry1828

actual-value59:                                   ; preds = %force63, %after-label66, %thunk65, %force60, %found1833, %force52, %after-label55, %thunk54, %force49, %force41, %after-label44, %thunk43, %force38, %force30, %after-label33, %thunk32, %force27, %force24, %primitive-branch17, %compiled-branch18, %force15, %force7, %after-label10, %found775, %force4
  %"load register thunk1893" = load { i32, ptr }, ptr %thunk, align 8
  %get_type1894 = extractvalue { i32, ptr } %"load register thunk1893", 0
  %"is hempty1895" = icmp eq i32 %get_type1894, 8
  %"not thunk1896" = xor i1 %"is hempty1895", true
  %"object value1897" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr (i1, ptr null, i32 1) to i32))
  store i1 %"not thunk1896", ptr %"object value1897", align 1
  %"insert value object1898" = insertvalue { i32, ptr } { i32 1, ptr null }, ptr %"object value1897", 1
  store { i32, ptr } %"insert value object1898", ptr %flag, align 8
  %"load flag1899" = load { i32, ptr }, ptr %flag, align 8
  %get_type1901 = extractvalue { i32, ptr } %"load flag1899", 0
  %"not bool check1902" = icmp ne i32 %get_type1901, 1
  %"get object context1903" = extractvalue { i32, ptr } %"load flag1899", 1
  %"get bool value1904" = load i1, ptr %"get object context1903", align 1
  %"non bool or true1905" = or i1 %"not bool check1902", %"get bool value1904"
  br i1 %"non bool or true1905", label %done61, label %next-block1900

force60:                                          ; preds = %force63, %after-label66, %thunk65, %force60, %next-block1900, %force52, %after-label55, %thunk54, %force49, %force41, %after-label44, %thunk43, %force38, %force30, %after-label33, %thunk32, %force27, %force24, %primitive-branch17, %compiled-branch18, %force15, %force7, %after-label10, %found775, %force4
  %"object value1906" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr (ptr, ptr null, i32 1) to i32))
  store ptr blockaddress(@main, %actual-value59), ptr %"object value1906", align 8
  %"insert value object1907" = insertvalue { i32, ptr } { i32 5, ptr null }, ptr %"object value1906", 1
  store { i32, ptr } %"insert value object1907", ptr %continue, align 8
  %"load register thunk1908" = load { i32, ptr }, ptr %thunk, align 8
  %"extract-thunk:return1909" = extractvalue { i32, ptr } %"load register thunk1908", 1
  %"extract-thunk:1910" = load { { i32, ptr }, { i32, ptr } }, ptr %"extract-thunk:return1909", align 8
  %"thunk entry1911" = extractvalue { { i32, ptr }, { i32, ptr } } %"extract-thunk:1910", 0
  store { i32, ptr } %"thunk entry1911", ptr %val, align 8
  %"load register val1912" = load { i32, ptr }, ptr %val, align 8
  %"extract-label:return1913" = extractvalue { i32, ptr } %"load register val1912", 1
  %"extract-label:1914" = load ptr, ptr %"extract-label:return1913", align 8
  indirectbr ptr %"extract-label:1914", [label %done28, label %after-call36, label %done16, label %after-call47, label %done8, label %compiled-branch35, label %force15, label %done53, label %compiled-branch57, label %after-label66, label %actual-value26, label %true-branch20, label %compiled-branch68, label %actual-value37, label %done50, label %actual-value6, label %add1, label %done61, label %actual-value23, label %primitive-branch34, label %thunk54, label %primitive-branch11, label %compiled-branch12, label %actual-value14, label %actual-value29, label %done31, label %force27, label %compiled-branch18, label %actual-value59, label %thunk32, label %compiled-branch46, label %force41, label %done42, label %after-call19, label %primitive-branch17, label %after-label10, label %actual-value51, label %after-label44, label %force52, label %thunk43, label %force4, label %primitive-branch45, label %force60, label %force7, label %force24, label %done39, label %actual-value40, label %primitive-branch67, label %thunk65, label %after-label55, label %force49, label %actual-value62, label %after-call13, label %after-if22, label %entry1, label %done64, label %primitive-branch56, label %after-label33, label %force30, label %done5, label %done25, label %thunk9, label %actual-value48, label %after-lambda2, label %false-branch21, label %actual-value3, label %after-call58, label %x, label %force63, label %after-call69, label %force38]

done61:                                           ; preds = %force63, %after-label66, %thunk65, %next-block1915, %force60, %actual-value59, %force52, %after-label55, %thunk54, %force49, %force41, %after-label44, %thunk43, %force38, %force30, %after-label33, %thunk32, %force27, %force24, %primitive-branch17, %compiled-branch18, %force15, %force7, %after-label10, %found775, %force4
  %"load register thunk1916" = load { i32, ptr }, ptr %thunk, align 8
  store { i32, ptr } %"load register thunk1916", ptr %proc, align 8
  %stack1917 = load { { i32, ptr }, ptr }, ptr %stack, align 8
  %"old stack1918" = extractvalue { { i32, ptr }, ptr } %stack1917, 1
  %"current stack1919" = extractvalue { { i32, ptr }, ptr } %stack1917, 0
  store { i32, ptr } %"current stack1919", ptr %env, align 8
  %"load previous stack1920" = load { { i32, ptr }, ptr }, ptr %"old stack1918", align 8
  store { { i32, ptr }, ptr } %"load previous stack1920", ptr %stack, align 8
  %"load register proc1921" = load { i32, ptr }, ptr %proc, align 8
  %get_type1922 = extractvalue { i32, ptr } %"load register proc1921", 0
  %"is hempty1923" = icmp eq i32 %get_type1922, 7
  %"object value1924" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr (i1, ptr null, i32 1) to i32))
  store i1 %"is hempty1923", ptr %"object value1924", align 1
  %"insert value object1925" = insertvalue { i32, ptr } { i32 1, ptr null }, ptr %"object value1924", 1
  store { i32, ptr } %"insert value object1925", ptr %flag, align 8
  %"load flag1926" = load { i32, ptr }, ptr %flag, align 8
  %get_type1928 = extractvalue { i32, ptr } %"load flag1926", 0
  %"not bool check1929" = icmp ne i32 %get_type1928, 1
  %"get object context1930" = extractvalue { i32, ptr } %"load flag1926", 1
  %"get bool value1931" = load i1, ptr %"get object context1930", align 1
  %"non bool or true1932" = or i1 %"not bool check1929", %"get bool value1931"
  br i1 %"non bool or true1932", label %primitive-branch67, label %next-block1927

compiled-branch68:                                ; preds = %force63, %after-label66, %thunk65, %next-block1927, %force60, %force52, %after-label55, %thunk54, %force49, %force41, %after-label44, %thunk43, %force38, %force30, %after-label33, %thunk32, %force27, %force24, %primitive-branch17, %compiled-branch18, %force15, %force7, %after-label10, %found775, %force4
  store { i32, ptr } zeroinitializer, ptr %argl, align 8
  %"object value1933" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr (ptr, ptr null, i32 1) to i32))
  store ptr blockaddress(@main, %thunk65), ptr %"object value1933", align 8
  %"insert value object1934" = insertvalue { i32, ptr } { i32 5, ptr null }, ptr %"object value1933", 1
  %"load register env1935" = load { i32, ptr }, ptr %env, align 8
  %"insert into struct1936" = insertvalue { { i32, ptr }, { i32, ptr } } zeroinitializer, { i32, ptr } %"insert value object1934", 0
  %"insert into struct1937" = insertvalue { { i32, ptr }, { i32, ptr } } %"insert into struct1936", { i32, ptr } %"load register env1935", 1
  %"object value1938" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ { i32, ptr }, { i32, ptr } }, ptr null, i32 1) to i32))
  store { { i32, ptr }, { i32, ptr } } %"insert into struct1937", ptr %"object value1938", align 8
  %"insert value object1939" = insertvalue { i32, ptr } { i32 8, ptr null }, ptr %"object value1938", 1
  store { i32, ptr } %"insert value object1939", ptr %val, align 8
  br label %after-label66

thunk65:                                          ; preds = %force63, %after-label66, %thunk65, %next-block1940, %force60, %force52, %after-label55, %thunk54, %force49, %force41, %after-label44, %thunk43, %force38, %force30, %after-label33, %thunk32, %force27, %force24, %primitive-branch17, %compiled-branch18, %force15, %force7, %after-label10, %found775, %force4
  %"load register thunk1941" = load { i32, ptr }, ptr %thunk, align 8
  %"extract-thunk:return1942" = extractvalue { i32, ptr } %"load register thunk1941", 1
  %"extract-thunk:1943" = load { { i32, ptr }, { i32, ptr } }, ptr %"extract-thunk:return1942", align 8
  %"thunk env1944" = extractvalue { { i32, ptr }, { i32, ptr } } %"extract-thunk:1943", 1
  store { i32, ptr } %"thunk env1944", ptr %env, align 8
  %"object value1945" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  store { i32, ptr } { i32 2, ptr @aa }, ptr %"object value1945", align 8
  %"insert value object1946" = insertvalue { i32, ptr } { i32 3, ptr null }, ptr %"object value1945", 1
  store { i32, ptr } %"insert value object1946", ptr %thunk, align 8
  %"load register continue1947" = load { i32, ptr }, ptr %continue, align 8
  %"extract-label:return1948" = extractvalue { i32, ptr } %"load register continue1947", 1
  %"extract-label:1949" = load ptr, ptr %"extract-label:return1948", align 8
  indirectbr ptr %"extract-label:1949", [label %done28, label %after-call36, label %done16, label %after-call47, label %done8, label %compiled-branch35, label %force15, label %done53, label %compiled-branch57, label %after-label66, label %actual-value26, label %true-branch20, label %compiled-branch68, label %actual-value37, label %done50, label %actual-value6, label %add1, label %done61, label %actual-value23, label %primitive-branch34, label %thunk54, label %primitive-branch11, label %compiled-branch12, label %actual-value14, label %actual-value29, label %done31, label %force27, label %compiled-branch18, label %actual-value59, label %thunk32, label %compiled-branch46, label %force41, label %done42, label %after-call19, label %primitive-branch17, label %after-label10, label %actual-value51, label %after-label44, label %force52, label %thunk43, label %force4, label %primitive-branch45, label %force60, label %force7, label %force24, label %done39, label %actual-value40, label %primitive-branch67, label %thunk65, label %after-label55, label %force49, label %actual-value62, label %after-call13, label %after-if22, label %entry1, label %done64, label %primitive-branch56, label %after-label33, label %force30, label %done5, label %done25, label %thunk9, label %actual-value48, label %after-lambda2, label %false-branch21, label %actual-value3, label %after-call58, label %x, label %force63, label %after-call69, label %force38]

after-label66:                                    ; preds = %force63, %after-label66, %next-block1950, %thunk65, %compiled-branch68, %force60, %force52, %after-label55, %thunk54, %force49, %force41, %after-label44, %thunk43, %force38, %force30, %after-label33, %thunk32, %force27, %force24, %primitive-branch17, %compiled-branch18, %force15, %force7, %after-label10, %found775, %force4
  %"load register val1951" = load { i32, ptr }, ptr %val, align 8
  %"load register argl1952" = load { i32, ptr }, ptr %argl, align 8
  %"car ptr1953" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  %"cdr ptr1954" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  store { i32, ptr } %"load register val1951", ptr %"car ptr1953", align 8
  store { i32, ptr } %"load register argl1952", ptr %"cdr ptr1954", align 8
  %"insert car - cons1955" = insertvalue { ptr, ptr } zeroinitializer, ptr %"car ptr1953", 0
  %"insert cdr - cons1956" = insertvalue { ptr, ptr } %"insert car - cons1955", ptr %"cdr ptr1954", 1
  %"object value1957" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ ptr, ptr }, ptr null, i32 1) to i32))
  store { ptr, ptr } %"insert cdr - cons1956", ptr %"object value1957", align 8
  %"insert value object1958" = insertvalue { i32, ptr } { i32 6, ptr null }, ptr %"object value1957", 1
  store { i32, ptr } %"insert value object1958", ptr %argl, align 8
  %"object value1959" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr (ptr, ptr null, i32 1) to i32))
  store ptr blockaddress(@main, %after-call69), ptr %"object value1959", align 8
  %"insert value object1960" = insertvalue { i32, ptr } { i32 5, ptr null }, ptr %"object value1959", 1
  store { i32, ptr } %"insert value object1960", ptr %continue, align 8
  %"load register proc1961" = load { i32, ptr }, ptr %proc, align 8
  %"extract-lambda:return1962" = extractvalue { i32, ptr } %"load register proc1961", 1
  %"extract-lambda:1963" = load { { i32, ptr }, { i32, ptr } }, ptr %"extract-lambda:return1962", align 8
  %"proc entry1964" = extractvalue { { i32, ptr }, { i32, ptr } } %"extract-lambda:1963", 0
  store { i32, ptr } %"proc entry1964", ptr %val, align 8
  %"load register val1965" = load { i32, ptr }, ptr %val, align 8
  %"extract-label:return1966" = extractvalue { i32, ptr } %"load register val1965", 1
  %"extract-label:1967" = load ptr, ptr %"extract-label:return1966", align 8
  indirectbr ptr %"extract-label:1967", [label %done28, label %after-call36, label %done16, label %after-call47, label %done8, label %compiled-branch35, label %force15, label %done53, label %compiled-branch57, label %after-label66, label %actual-value26, label %true-branch20, label %compiled-branch68, label %actual-value37, label %done50, label %actual-value6, label %add1, label %done61, label %actual-value23, label %primitive-branch34, label %thunk54, label %primitive-branch11, label %compiled-branch12, label %actual-value14, label %actual-value29, label %done31, label %force27, label %compiled-branch18, label %actual-value59, label %thunk32, label %compiled-branch46, label %force41, label %done42, label %after-call19, label %primitive-branch17, label %after-label10, label %actual-value51, label %after-label44, label %force52, label %thunk43, label %force4, label %primitive-branch45, label %force60, label %force7, label %force24, label %done39, label %actual-value40, label %primitive-branch67, label %thunk65, label %after-label55, label %force49, label %actual-value62, label %after-call13, label %after-if22, label %entry1, label %done64, label %primitive-branch56, label %after-label33, label %force30, label %done5, label %done25, label %thunk9, label %actual-value48, label %after-lambda2, label %false-branch21, label %actual-value3, label %after-call58, label %x, label %force63, label %after-call69, label %force38]

primitive-branch67:                               ; preds = %force63, %next-block1968, %after-label66, %thunk65, %done61, %force60, %force52, %after-label55, %thunk54, %force49, %force41, %after-label44, %thunk43, %force38, %force30, %after-label33, %thunk32, %force27, %force24, %primitive-branch17, %compiled-branch18, %force15, %force7, %after-label10, %found775, %force4
  %"load stack1969" = load { { i32, ptr }, ptr }, ptr %stack, align 8
  %"previous stack1970" = alloca { { i32, ptr }, ptr }, align 8
  store { { i32, ptr }, ptr } %"load stack1969", ptr %"previous stack1970", align 8
  %"load register1971" = load { i32, ptr }, ptr %proc, align 8
  %"save register1972" = insertvalue { { i32, ptr }, ptr } zeroinitializer, { i32, ptr } %"load register1971", 0
  %"save previous stack1973" = insertvalue { { i32, ptr }, ptr } %"save register1972", ptr %"previous stack1970", 1
  store { { i32, ptr }, ptr } %"save previous stack1973", ptr %stack, align 8
  store { i32, ptr } zeroinitializer, ptr %argl, align 8
  %"load stack1974" = load { { i32, ptr }, ptr }, ptr %stack, align 8
  %"previous stack1975" = alloca { { i32, ptr }, ptr }, align 8
  store { { i32, ptr }, ptr } %"load stack1974", ptr %"previous stack1975", align 8
  %"load register1976" = load { i32, ptr }, ptr %argl, align 8
  %"save register1977" = insertvalue { { i32, ptr }, ptr } zeroinitializer, { i32, ptr } %"load register1976", 0
  %"save previous stack1978" = insertvalue { { i32, ptr }, ptr } %"save register1977", ptr %"previous stack1975", 1
  store { { i32, ptr }, ptr } %"save previous stack1978", ptr %stack, align 8
  %"object value1979" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  store { i32, ptr } { i32 2, ptr @aa.221 }, ptr %"object value1979", align 8
  %"insert value object1980" = insertvalue { i32, ptr } { i32 3, ptr null }, ptr %"object value1979", 1
  store { i32, ptr } %"insert value object1980", ptr %thunk, align 8
  br label %actual-value62

actual-value62:                                   ; preds = %force63, %primitive-branch67, %after-label66, %thunk65, %force60, %force52, %after-label55, %thunk54, %force49, %force41, %after-label44, %thunk43, %force38, %force30, %after-label33, %thunk32, %force27, %force24, %primitive-branch17, %compiled-branch18, %force15, %force7, %after-label10, %found775, %force4
  %"load register thunk1981" = load { i32, ptr }, ptr %thunk, align 8
  %get_type1982 = extractvalue { i32, ptr } %"load register thunk1981", 0
  %"is hempty1983" = icmp eq i32 %get_type1982, 8
  %"not thunk1984" = xor i1 %"is hempty1983", true
  %"object value1985" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr (i1, ptr null, i32 1) to i32))
  store i1 %"not thunk1984", ptr %"object value1985", align 1
  %"insert value object1986" = insertvalue { i32, ptr } { i32 1, ptr null }, ptr %"object value1985", 1
  store { i32, ptr } %"insert value object1986", ptr %flag, align 8
  %"load flag1987" = load { i32, ptr }, ptr %flag, align 8
  %get_type1989 = extractvalue { i32, ptr } %"load flag1987", 0
  %"not bool check1990" = icmp ne i32 %get_type1989, 1
  %"get object context1991" = extractvalue { i32, ptr } %"load flag1987", 1
  %"get bool value1992" = load i1, ptr %"get object context1991", align 1
  %"non bool or true1993" = or i1 %"not bool check1990", %"get bool value1992"
  br i1 %"non bool or true1993", label %done64, label %next-block1988

force63:                                          ; preds = %force63, %next-block1988, %after-label66, %thunk65, %force60, %force52, %after-label55, %thunk54, %force49, %force41, %after-label44, %thunk43, %force38, %force30, %after-label33, %thunk32, %force27, %force24, %primitive-branch17, %compiled-branch18, %force15, %force7, %after-label10, %found775, %force4
  %"object value1994" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr (ptr, ptr null, i32 1) to i32))
  store ptr blockaddress(@main, %actual-value62), ptr %"object value1994", align 8
  %"insert value object1995" = insertvalue { i32, ptr } { i32 5, ptr null }, ptr %"object value1994", 1
  store { i32, ptr } %"insert value object1995", ptr %continue, align 8
  %"load register thunk1996" = load { i32, ptr }, ptr %thunk, align 8
  %"extract-thunk:return1997" = extractvalue { i32, ptr } %"load register thunk1996", 1
  %"extract-thunk:1998" = load { { i32, ptr }, { i32, ptr } }, ptr %"extract-thunk:return1997", align 8
  %"thunk entry1999" = extractvalue { { i32, ptr }, { i32, ptr } } %"extract-thunk:1998", 0
  store { i32, ptr } %"thunk entry1999", ptr %val, align 8
  %"load register val2000" = load { i32, ptr }, ptr %val, align 8
  %"extract-label:return2001" = extractvalue { i32, ptr } %"load register val2000", 1
  %"extract-label:2002" = load ptr, ptr %"extract-label:return2001", align 8
  indirectbr ptr %"extract-label:2002", [label %done28, label %after-call36, label %done16, label %after-call47, label %done8, label %compiled-branch35, label %force15, label %done53, label %compiled-branch57, label %after-label66, label %actual-value26, label %true-branch20, label %compiled-branch68, label %actual-value37, label %done50, label %actual-value6, label %add1, label %done61, label %actual-value23, label %primitive-branch34, label %thunk54, label %primitive-branch11, label %compiled-branch12, label %actual-value14, label %actual-value29, label %done31, label %force27, label %compiled-branch18, label %actual-value59, label %thunk32, label %compiled-branch46, label %force41, label %done42, label %after-call19, label %primitive-branch17, label %after-label10, label %actual-value51, label %after-label44, label %force52, label %thunk43, label %force4, label %primitive-branch45, label %force60, label %force7, label %force24, label %done39, label %actual-value40, label %primitive-branch67, label %thunk65, label %after-label55, label %force49, label %actual-value62, label %after-call13, label %after-if22, label %entry1, label %done64, label %primitive-branch56, label %after-label33, label %force30, label %done5, label %done25, label %thunk9, label %actual-value48, label %after-lambda2, label %false-branch21, label %actual-value3, label %after-call58, label %x, label %force63, label %after-call69, label %force38]

done64:                                           ; preds = %next-block2003, %force63, %actual-value62, %after-label66, %thunk65, %force60, %force52, %after-label55, %thunk54, %force49, %force41, %after-label44, %thunk43, %force38, %force30, %after-label33, %thunk32, %force27, %force24, %primitive-branch17, %compiled-branch18, %force15, %force7, %after-label10, %found775, %force4
  %"load register thunk2004" = load { i32, ptr }, ptr %thunk, align 8
  store { i32, ptr } %"load register thunk2004", ptr %val, align 8
  %stack2005 = load { { i32, ptr }, ptr }, ptr %stack, align 8
  %"old stack2006" = extractvalue { { i32, ptr }, ptr } %stack2005, 1
  %"current stack2007" = extractvalue { { i32, ptr }, ptr } %stack2005, 0
  store { i32, ptr } %"current stack2007", ptr %argl, align 8
  %"load previous stack2008" = load { { i32, ptr }, ptr }, ptr %"old stack2006", align 8
  store { { i32, ptr }, ptr } %"load previous stack2008", ptr %stack, align 8
  %"load register val2009" = load { i32, ptr }, ptr %val, align 8
  %"load register argl2010" = load { i32, ptr }, ptr %argl, align 8
  %"car ptr2011" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  %"cdr ptr2012" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  store { i32, ptr } %"load register val2009", ptr %"car ptr2011", align 8
  store { i32, ptr } %"load register argl2010", ptr %"cdr ptr2012", align 8
  %"insert car - cons2013" = insertvalue { ptr, ptr } zeroinitializer, ptr %"car ptr2011", 0
  %"insert cdr - cons2014" = insertvalue { ptr, ptr } %"insert car - cons2013", ptr %"cdr ptr2012", 1
  %"object value2015" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ ptr, ptr }, ptr null, i32 1) to i32))
  store { ptr, ptr } %"insert cdr - cons2014", ptr %"object value2015", align 8
  %"insert value object2016" = insertvalue { i32, ptr } { i32 6, ptr null }, ptr %"object value2015", 1
  store { i32, ptr } %"insert value object2016", ptr %argl, align 8
  %stack2017 = load { { i32, ptr }, ptr }, ptr %stack, align 8
  %"old stack2018" = extractvalue { { i32, ptr }, ptr } %stack2017, 1
  %"current stack2019" = extractvalue { { i32, ptr }, ptr } %stack2017, 0
  store { i32, ptr } %"current stack2019", ptr %proc, align 8
  %"load previous stack2020" = load { { i32, ptr }, ptr }, ptr %"old stack2018", align 8
  store { { i32, ptr }, ptr } %"load previous stack2020", ptr %stack, align 8
  %"load register proc2021" = load { i32, ptr }, ptr %proc, align 8
  %"load register argl2022" = load { i32, ptr }, ptr %argl, align 8
  %"extract-primitive:return2023" = extractvalue { i32, ptr } %"load register proc2021", 1
  %"extract-primitive:2024" = load ptr, ptr %"extract-primitive:return2023", align 8
  %"call primitive2025" = call { i32, ptr } %"extract-primitive:2024"({ i32, ptr } %"load register argl2022")
  store { i32, ptr } %"call primitive2025", ptr %val, align 8
  br label %after-call69

after-call69:                                     ; preds = %done64, %force63, %after-label66, %thunk65, %force60, %force52, %after-label55, %thunk54, %force49, %force41, %after-label44, %thunk43, %force38, %force30, %after-label33, %thunk32, %force27, %force24, %primitive-branch17, %compiled-branch18, %force15, %force7, %after-label10, %found775, %force4
  ret i32 0

next-block:                                       ; No predecessors!
  br label %entry1

lookup-entry:                                     ; preds = %next-env, %entry1
  %"load env" = load { i32, ptr }, ptr %env690, align 8
  %get_type691 = extractvalue { i32, ptr } %"load env", 0
  %"is hempty" = icmp eq i32 %get_type691, 0
  br i1 %"is hempty", label %error, label %lookup

lookup:                                           ; preds = %lookup-entry
  %"extract-cons:return692" = extractvalue { i32, ptr } %"load env", 1
  %"extract-cons:693" = load { ptr, ptr }, ptr %"extract-cons:return692", align 8
  %"get car694" = extractvalue { ptr, ptr } %"extract-cons:693", 0
  %"load car695" = load { i32, ptr }, ptr %"get car694", align 8
  %vars = alloca { i32, ptr }, align 8
  %vals = alloca { i32, ptr }, align 8
  %"extract-cons:return696" = extractvalue { i32, ptr } %"load car695", 1
  %"extract-cons:697" = load { ptr, ptr }, ptr %"extract-cons:return696", align 8
  %"get car698" = extractvalue { ptr, ptr } %"extract-cons:697", 0
  %"load car699" = load { i32, ptr }, ptr %"get car698", align 8
  store { i32, ptr } %"load car699", ptr %vars, align 8
  %"extract-cons:return700" = extractvalue { i32, ptr } %"load car695", 1
  %"extract-cons:701" = load { ptr, ptr }, ptr %"extract-cons:return700", align 8
  %"get cdr" = extractvalue { ptr, ptr } %"extract-cons:701", 1
  %"load cdr" = load { i32, ptr }, ptr %"get cdr", align 8
  store { i32, ptr } %"load cdr", ptr %vals, align 8
  br label %scan

scan:                                             ; preds = %scan-next, %lookup
  %vars702 = load { i32, ptr }, ptr %vars, align 8
  %get_type703 = extractvalue { i32, ptr } %vars702, 0
  %"is hempty704" = icmp eq i32 %get_type703, 0
  br i1 %"is hempty704", label %next-env, label %check

next-env:                                         ; preds = %scan
  %"extract-cons:return705" = extractvalue { i32, ptr } %"load env", 1
  %"extract-cons:706" = load { ptr, ptr }, ptr %"extract-cons:return705", align 8
  %"get cdr707" = extractvalue { ptr, ptr } %"extract-cons:706", 1
  %"load cdr708" = load { i32, ptr }, ptr %"get cdr707", align 8
  store { i32, ptr } %"load cdr708", ptr %env690, align 8
  br label %lookup-entry

check:                                            ; preds = %scan
  %"load vars" = load { i32, ptr }, ptr %vars, align 8
  %"extract-cons:return709" = extractvalue { i32, ptr } %"load vars", 1
  %"extract-cons:710" = load { ptr, ptr }, ptr %"extract-cons:return709", align 8
  %"get car711" = extractvalue { ptr, ptr } %"extract-cons:710", 0
  %"load car712" = load { i32, ptr }, ptr %"get car711", align 8
  %"extract-symbol:return" = extractvalue { i32, ptr } %"insert value object688", 1
  %"extract-symbol:" = load { i32, ptr }, ptr %"extract-symbol:return", align 8
  %"extract-symbol:return713" = extractvalue { i32, ptr } %"load car712", 1
  %"extract-symbol:714" = load { i32, ptr }, ptr %"extract-symbol:return713", align 8
  %"get str length" = extractvalue { i32, ptr } %"extract-symbol:", 0
  %"get str length715" = extractvalue { i32, ptr } %"extract-symbol:714", 0
  %"get str" = extractvalue { i32, ptr } %"extract-symbol:", 1
  %"get str716" = extractvalue { i32, ptr } %"extract-symbol:714", 1
  %"len matches" = icmp eq i32 %"get str length", %"get str length715"
  %smaller = icmp slt i32 %"get str length", %"get str length715"
  %"str smallaest size" = select i1 %smaller, i32 %"get str length", i32 %"get str length715"
  %strcmp = call i32 @strncmp(ptr %"get str", ptr %"get str716", i32 %"str smallaest size")
  %"is same string" = icmp eq i32 %strcmp, 0
  %"eq?" = and i1 %"len matches", %"is same string"
  br i1 %"eq?", label %found, label %scan-next

found:                                            ; preds = %check
  %"load vals725" = load { i32, ptr }, ptr %vals, align 8
  %"extract-cons:return726" = extractvalue { i32, ptr } %"load vals725", 1
  %"extract-cons:727" = load { ptr, ptr }, ptr %"extract-cons:return726", align 8
  %"get car728" = extractvalue { ptr, ptr } %"extract-cons:727", 0
  %"load car729" = load { i32, ptr }, ptr %"get car728", align 8
  store { i32, ptr } %"load car729", ptr %thunk, align 8
  br label %actual-value3

scan-next:                                        ; preds = %check
  %"load vals" = load { i32, ptr }, ptr %vals, align 8
  %"extract-cons:return717" = extractvalue { i32, ptr } %"load vars", 1
  %"extract-cons:718" = load { ptr, ptr }, ptr %"extract-cons:return717", align 8
  %"get cdr719" = extractvalue { ptr, ptr } %"extract-cons:718", 1
  %"load cdr720" = load { i32, ptr }, ptr %"get cdr719", align 8
  store { i32, ptr } %"load cdr720", ptr %vars, align 8
  %"extract-cons:return721" = extractvalue { i32, ptr } %"load vals", 1
  %"extract-cons:722" = load { ptr, ptr }, ptr %"extract-cons:return721", align 8
  %"get cdr723" = extractvalue { ptr, ptr } %"extract-cons:722", 1
  %"load cdr724" = load { i32, ptr }, ptr %"get cdr723", align 8
  store { i32, ptr } %"load cdr724", ptr %vals, align 8
  br label %scan

next-block734:                                    ; preds = %actual-value3
  br label %force4

next-block741:                                    ; No predecessors!
  br label %done5

next-block750:                                    ; preds = %done5
  br label %compiled-branch12

next-block763:                                    ; No predecessors!
  br label %thunk9

lookup-entry770:                                  ; preds = %next-env773, %thunk9
  %"load env778" = load { i32, ptr }, ptr %env777, align 8
  %get_type779 = extractvalue { i32, ptr } %"load env778", 0
  %"is hempty780" = icmp eq i32 %get_type779, 0
  br i1 %"is hempty780", label %error, label %lookup771

lookup771:                                        ; preds = %lookup-entry770
  %"extract-cons:return781" = extractvalue { i32, ptr } %"load env778", 1
  %"extract-cons:782" = load { ptr, ptr }, ptr %"extract-cons:return781", align 8
  %"get car783" = extractvalue { ptr, ptr } %"extract-cons:782", 0
  %"load car784" = load { i32, ptr }, ptr %"get car783", align 8
  %vars785 = alloca { i32, ptr }, align 8
  %vals786 = alloca { i32, ptr }, align 8
  %"extract-cons:return787" = extractvalue { i32, ptr } %"load car784", 1
  %"extract-cons:788" = load { ptr, ptr }, ptr %"extract-cons:return787", align 8
  %"get car789" = extractvalue { ptr, ptr } %"extract-cons:788", 0
  %"load car790" = load { i32, ptr }, ptr %"get car789", align 8
  store { i32, ptr } %"load car790", ptr %vars785, align 8
  %"extract-cons:return791" = extractvalue { i32, ptr } %"load car784", 1
  %"extract-cons:792" = load { ptr, ptr }, ptr %"extract-cons:return791", align 8
  %"get cdr793" = extractvalue { ptr, ptr } %"extract-cons:792", 1
  %"load cdr794" = load { i32, ptr }, ptr %"get cdr793", align 8
  store { i32, ptr } %"load cdr794", ptr %vals786, align 8
  br label %scan772

scan772:                                          ; preds = %scan-next776, %lookup771
  %vars795 = load { i32, ptr }, ptr %vars785, align 8
  %get_type796 = extractvalue { i32, ptr } %vars795, 0
  %"is hempty797" = icmp eq i32 %get_type796, 0
  br i1 %"is hempty797", label %next-env773, label %check774

next-env773:                                      ; preds = %scan772
  %"extract-cons:return798" = extractvalue { i32, ptr } %"load env778", 1
  %"extract-cons:799" = load { ptr, ptr }, ptr %"extract-cons:return798", align 8
  %"get cdr800" = extractvalue { ptr, ptr } %"extract-cons:799", 1
  %"load cdr801" = load { i32, ptr }, ptr %"get cdr800", align 8
  store { i32, ptr } %"load cdr801", ptr %env777, align 8
  br label %lookup-entry770

check774:                                         ; preds = %scan772
  %"load vars802" = load { i32, ptr }, ptr %vars785, align 8
  %"extract-cons:return803" = extractvalue { i32, ptr } %"load vars802", 1
  %"extract-cons:804" = load { ptr, ptr }, ptr %"extract-cons:return803", align 8
  %"get car805" = extractvalue { ptr, ptr } %"extract-cons:804", 0
  %"load car806" = load { i32, ptr }, ptr %"get car805", align 8
  %"extract-symbol:return807" = extractvalue { i32, ptr } %"insert value object768", 1
  %"extract-symbol:808" = load { i32, ptr }, ptr %"extract-symbol:return807", align 8
  %"extract-symbol:return809" = extractvalue { i32, ptr } %"load car806", 1
  %"extract-symbol:810" = load { i32, ptr }, ptr %"extract-symbol:return809", align 8
  %"get str length811" = extractvalue { i32, ptr } %"extract-symbol:808", 0
  %"get str length812" = extractvalue { i32, ptr } %"extract-symbol:810", 0
  %"get str813" = extractvalue { i32, ptr } %"extract-symbol:808", 1
  %"get str814" = extractvalue { i32, ptr } %"extract-symbol:810", 1
  %"len matches815" = icmp eq i32 %"get str length811", %"get str length812"
  %smaller816 = icmp slt i32 %"get str length811", %"get str length812"
  %"str smallaest size817" = select i1 %smaller816, i32 %"get str length811", i32 %"get str length812"
  %strcmp818 = call i32 @strncmp(ptr %"get str813", ptr %"get str814", i32 %"str smallaest size817")
  %"is same string819" = icmp eq i32 %strcmp818, 0
  %"eq?820" = and i1 %"len matches815", %"is same string819"
  br i1 %"eq?820", label %found775, label %scan-next776

found775:                                         ; preds = %check774
  %"load vals830" = load { i32, ptr }, ptr %vals786, align 8
  %"extract-cons:return831" = extractvalue { i32, ptr } %"load vals830", 1
  %"extract-cons:832" = load { ptr, ptr }, ptr %"extract-cons:return831", align 8
  %"get car833" = extractvalue { ptr, ptr } %"extract-cons:832", 0
  %"load car834" = load { i32, ptr }, ptr %"get car833", align 8
  store { i32, ptr } %"load car834", ptr %thunk, align 8
  %"load register continue835" = load { i32, ptr }, ptr %continue, align 8
  %"extract-label:return836" = extractvalue { i32, ptr } %"load register continue835", 1
  %"extract-label:837" = load ptr, ptr %"extract-label:return836", align 8
  indirectbr ptr %"extract-label:837", [label %done28, label %after-call36, label %done16, label %after-call47, label %done8, label %compiled-branch35, label %force15, label %done53, label %compiled-branch57, label %after-label66, label %actual-value26, label %true-branch20, label %compiled-branch68, label %actual-value37, label %done50, label %actual-value6, label %add1, label %done61, label %actual-value23, label %primitive-branch34, label %thunk54, label %primitive-branch11, label %compiled-branch12, label %actual-value14, label %actual-value29, label %done31, label %force27, label %compiled-branch18, label %actual-value59, label %thunk32, label %compiled-branch46, label %force41, label %done42, label %after-call19, label %primitive-branch17, label %after-label10, label %actual-value51, label %after-label44, label %force52, label %thunk43, label %force4, label %primitive-branch45, label %force60, label %force7, label %force24, label %done39, label %actual-value40, label %primitive-branch67, label %thunk65, label %after-label55, label %force49, label %actual-value62, label %after-call13, label %after-if22, label %entry1, label %done64, label %primitive-branch56, label %after-label33, label %force30, label %done5, label %done25, label %thunk9, label %actual-value48, label %after-lambda2, label %false-branch21, label %actual-value3, label %after-call58, label %x, label %force63, label %after-call69, label %force38]

scan-next776:                                     ; preds = %check774
  %"load vals821" = load { i32, ptr }, ptr %vals786, align 8
  %"extract-cons:return822" = extractvalue { i32, ptr } %"load vars802", 1
  %"extract-cons:823" = load { ptr, ptr }, ptr %"extract-cons:return822", align 8
  %"get cdr824" = extractvalue { ptr, ptr } %"extract-cons:823", 1
  %"load cdr825" = load { i32, ptr }, ptr %"get cdr824", align 8
  store { i32, ptr } %"load cdr825", ptr %vars785, align 8
  %"extract-cons:return826" = extractvalue { i32, ptr } %"load vals821", 1
  %"extract-cons:827" = load { ptr, ptr }, ptr %"extract-cons:return826", align 8
  %"get cdr828" = extractvalue { ptr, ptr } %"extract-cons:827", 1
  %"load cdr829" = load { i32, ptr }, ptr %"get cdr828", align 8
  store { i32, ptr } %"load cdr829", ptr %vals786, align 8
  br label %scan772

next-block838:                                    ; No predecessors!
  br label %after-label10

next-block855:                                    ; No predecessors!
  br label %primitive-branch11

lookup-entry869:                                  ; preds = %next-env872, %primitive-branch11
  %"load env877" = load { i32, ptr }, ptr %env876, align 8
  %get_type878 = extractvalue { i32, ptr } %"load env877", 0
  %"is hempty879" = icmp eq i32 %get_type878, 0
  br i1 %"is hempty879", label %error, label %lookup870

lookup870:                                        ; preds = %lookup-entry869
  %"extract-cons:return880" = extractvalue { i32, ptr } %"load env877", 1
  %"extract-cons:881" = load { ptr, ptr }, ptr %"extract-cons:return880", align 8
  %"get car882" = extractvalue { ptr, ptr } %"extract-cons:881", 0
  %"load car883" = load { i32, ptr }, ptr %"get car882", align 8
  %vars884 = alloca { i32, ptr }, align 8
  %vals885 = alloca { i32, ptr }, align 8
  %"extract-cons:return886" = extractvalue { i32, ptr } %"load car883", 1
  %"extract-cons:887" = load { ptr, ptr }, ptr %"extract-cons:return886", align 8
  %"get car888" = extractvalue { ptr, ptr } %"extract-cons:887", 0
  %"load car889" = load { i32, ptr }, ptr %"get car888", align 8
  store { i32, ptr } %"load car889", ptr %vars884, align 8
  %"extract-cons:return890" = extractvalue { i32, ptr } %"load car883", 1
  %"extract-cons:891" = load { ptr, ptr }, ptr %"extract-cons:return890", align 8
  %"get cdr892" = extractvalue { ptr, ptr } %"extract-cons:891", 1
  %"load cdr893" = load { i32, ptr }, ptr %"get cdr892", align 8
  store { i32, ptr } %"load cdr893", ptr %vals885, align 8
  br label %scan871

scan871:                                          ; preds = %scan-next875, %lookup870
  %vars894 = load { i32, ptr }, ptr %vars884, align 8
  %get_type895 = extractvalue { i32, ptr } %vars894, 0
  %"is hempty896" = icmp eq i32 %get_type895, 0
  br i1 %"is hempty896", label %next-env872, label %check873

next-env872:                                      ; preds = %scan871
  %"extract-cons:return897" = extractvalue { i32, ptr } %"load env877", 1
  %"extract-cons:898" = load { ptr, ptr }, ptr %"extract-cons:return897", align 8
  %"get cdr899" = extractvalue { ptr, ptr } %"extract-cons:898", 1
  %"load cdr900" = load { i32, ptr }, ptr %"get cdr899", align 8
  store { i32, ptr } %"load cdr900", ptr %env876, align 8
  br label %lookup-entry869

check873:                                         ; preds = %scan871
  %"load vars901" = load { i32, ptr }, ptr %vars884, align 8
  %"extract-cons:return902" = extractvalue { i32, ptr } %"load vars901", 1
  %"extract-cons:903" = load { ptr, ptr }, ptr %"extract-cons:return902", align 8
  %"get car904" = extractvalue { ptr, ptr } %"extract-cons:903", 0
  %"load car905" = load { i32, ptr }, ptr %"get car904", align 8
  %"extract-symbol:return906" = extractvalue { i32, ptr } %"insert value object867", 1
  %"extract-symbol:907" = load { i32, ptr }, ptr %"extract-symbol:return906", align 8
  %"extract-symbol:return908" = extractvalue { i32, ptr } %"load car905", 1
  %"extract-symbol:909" = load { i32, ptr }, ptr %"extract-symbol:return908", align 8
  %"get str length910" = extractvalue { i32, ptr } %"extract-symbol:907", 0
  %"get str length911" = extractvalue { i32, ptr } %"extract-symbol:909", 0
  %"get str912" = extractvalue { i32, ptr } %"extract-symbol:907", 1
  %"get str913" = extractvalue { i32, ptr } %"extract-symbol:909", 1
  %"len matches914" = icmp eq i32 %"get str length910", %"get str length911"
  %smaller915 = icmp slt i32 %"get str length910", %"get str length911"
  %"str smallaest size916" = select i1 %smaller915, i32 %"get str length910", i32 %"get str length911"
  %strcmp917 = call i32 @strncmp(ptr %"get str912", ptr %"get str913", i32 %"str smallaest size916")
  %"is same string918" = icmp eq i32 %strcmp917, 0
  %"eq?919" = and i1 %"len matches914", %"is same string918"
  br i1 %"eq?919", label %found874, label %scan-next875

found874:                                         ; preds = %check873
  %"load vals929" = load { i32, ptr }, ptr %vals885, align 8
  %"extract-cons:return930" = extractvalue { i32, ptr } %"load vals929", 1
  %"extract-cons:931" = load { ptr, ptr }, ptr %"extract-cons:return930", align 8
  %"get car932" = extractvalue { ptr, ptr } %"extract-cons:931", 0
  %"load car933" = load { i32, ptr }, ptr %"get car932", align 8
  store { i32, ptr } %"load car933", ptr %thunk, align 8
  br label %actual-value6

scan-next875:                                     ; preds = %check873
  %"load vals920" = load { i32, ptr }, ptr %vals885, align 8
  %"extract-cons:return921" = extractvalue { i32, ptr } %"load vars901", 1
  %"extract-cons:922" = load { ptr, ptr }, ptr %"extract-cons:return921", align 8
  %"get cdr923" = extractvalue { ptr, ptr } %"extract-cons:922", 1
  %"load cdr924" = load { i32, ptr }, ptr %"get cdr923", align 8
  store { i32, ptr } %"load cdr924", ptr %vars884, align 8
  %"extract-cons:return925" = extractvalue { i32, ptr } %"load vals920", 1
  %"extract-cons:926" = load { ptr, ptr }, ptr %"extract-cons:return925", align 8
  %"get cdr927" = extractvalue { ptr, ptr } %"extract-cons:926", 1
  %"load cdr928" = load { i32, ptr }, ptr %"get cdr927", align 8
  store { i32, ptr } %"load cdr928", ptr %vals885, align 8
  br label %scan871

next-block941:                                    ; preds = %actual-value6
  br label %force7

next-block956:                                    ; No predecessors!
  br label %done8

lookup-entry992:                                  ; preds = %next-env995, %after-call13
  %"load env1000" = load { i32, ptr }, ptr %env999, align 8
  %get_type1001 = extractvalue { i32, ptr } %"load env1000", 0
  %"is hempty1002" = icmp eq i32 %get_type1001, 0
  br i1 %"is hempty1002", label %error, label %lookup993

lookup993:                                        ; preds = %lookup-entry992
  %"extract-cons:return1003" = extractvalue { i32, ptr } %"load env1000", 1
  %"extract-cons:1004" = load { ptr, ptr }, ptr %"extract-cons:return1003", align 8
  %"get car1005" = extractvalue { ptr, ptr } %"extract-cons:1004", 0
  %"load car1006" = load { i32, ptr }, ptr %"get car1005", align 8
  %vars1007 = alloca { i32, ptr }, align 8
  %vals1008 = alloca { i32, ptr }, align 8
  %"extract-cons:return1009" = extractvalue { i32, ptr } %"load car1006", 1
  %"extract-cons:1010" = load { ptr, ptr }, ptr %"extract-cons:return1009", align 8
  %"get car1011" = extractvalue { ptr, ptr } %"extract-cons:1010", 0
  %"load car1012" = load { i32, ptr }, ptr %"get car1011", align 8
  store { i32, ptr } %"load car1012", ptr %vars1007, align 8
  %"extract-cons:return1013" = extractvalue { i32, ptr } %"load car1006", 1
  %"extract-cons:1014" = load { ptr, ptr }, ptr %"extract-cons:return1013", align 8
  %"get cdr1015" = extractvalue { ptr, ptr } %"extract-cons:1014", 1
  %"load cdr1016" = load { i32, ptr }, ptr %"get cdr1015", align 8
  store { i32, ptr } %"load cdr1016", ptr %vals1008, align 8
  br label %scan994

scan994:                                          ; preds = %scan-next998, %lookup993
  %vars1017 = load { i32, ptr }, ptr %vars1007, align 8
  %get_type1018 = extractvalue { i32, ptr } %vars1017, 0
  %"is hempty1019" = icmp eq i32 %get_type1018, 0
  br i1 %"is hempty1019", label %next-env995, label %check996

next-env995:                                      ; preds = %scan994
  %"extract-cons:return1020" = extractvalue { i32, ptr } %"load env1000", 1
  %"extract-cons:1021" = load { ptr, ptr }, ptr %"extract-cons:return1020", align 8
  %"get cdr1022" = extractvalue { ptr, ptr } %"extract-cons:1021", 1
  %"load cdr1023" = load { i32, ptr }, ptr %"get cdr1022", align 8
  store { i32, ptr } %"load cdr1023", ptr %env999, align 8
  br label %lookup-entry992

check996:                                         ; preds = %scan994
  %"load vars1024" = load { i32, ptr }, ptr %vars1007, align 8
  %"extract-cons:return1025" = extractvalue { i32, ptr } %"load vars1024", 1
  %"extract-cons:1026" = load { ptr, ptr }, ptr %"extract-cons:return1025", align 8
  %"get car1027" = extractvalue { ptr, ptr } %"extract-cons:1026", 0
  %"load car1028" = load { i32, ptr }, ptr %"get car1027", align 8
  %"extract-symbol:return1029" = extractvalue { i32, ptr } %"insert value object990", 1
  %"extract-symbol:1030" = load { i32, ptr }, ptr %"extract-symbol:return1029", align 8
  %"extract-symbol:return1031" = extractvalue { i32, ptr } %"load car1028", 1
  %"extract-symbol:1032" = load { i32, ptr }, ptr %"extract-symbol:return1031", align 8
  %"get str length1033" = extractvalue { i32, ptr } %"extract-symbol:1030", 0
  %"get str length1034" = extractvalue { i32, ptr } %"extract-symbol:1032", 0
  %"get str1035" = extractvalue { i32, ptr } %"extract-symbol:1030", 1
  %"get str1036" = extractvalue { i32, ptr } %"extract-symbol:1032", 1
  %"len matches1037" = icmp eq i32 %"get str length1033", %"get str length1034"
  %smaller1038 = icmp slt i32 %"get str length1033", %"get str length1034"
  %"str smallaest size1039" = select i1 %smaller1038, i32 %"get str length1033", i32 %"get str length1034"
  %strcmp1040 = call i32 @strncmp(ptr %"get str1035", ptr %"get str1036", i32 %"str smallaest size1039")
  %"is same string1041" = icmp eq i32 %strcmp1040, 0
  %"eq?1042" = and i1 %"len matches1037", %"is same string1041"
  br i1 %"eq?1042", label %found997, label %scan-next998

found997:                                         ; preds = %check996
  %"load vals1052" = load { i32, ptr }, ptr %vals1008, align 8
  %"extract-cons:return1053" = extractvalue { i32, ptr } %"load vals1052", 1
  %"extract-cons:1054" = load { ptr, ptr }, ptr %"extract-cons:return1053", align 8
  %"get car1055" = extractvalue { ptr, ptr } %"extract-cons:1054", 0
  %"load car1056" = load { i32, ptr }, ptr %"get car1055", align 8
  store { i32, ptr } %"load car1056", ptr %thunk, align 8
  br label %actual-value14

scan-next998:                                     ; preds = %check996
  %"load vals1043" = load { i32, ptr }, ptr %vals1008, align 8
  %"extract-cons:return1044" = extractvalue { i32, ptr } %"load vars1024", 1
  %"extract-cons:1045" = load { ptr, ptr }, ptr %"extract-cons:return1044", align 8
  %"get cdr1046" = extractvalue { ptr, ptr } %"extract-cons:1045", 1
  %"load cdr1047" = load { i32, ptr }, ptr %"get cdr1046", align 8
  store { i32, ptr } %"load cdr1047", ptr %vars1007, align 8
  %"extract-cons:return1048" = extractvalue { i32, ptr } %"load vals1043", 1
  %"extract-cons:1049" = load { ptr, ptr }, ptr %"extract-cons:return1048", align 8
  %"get cdr1050" = extractvalue { ptr, ptr } %"extract-cons:1049", 1
  %"load cdr1051" = load { i32, ptr }, ptr %"get cdr1050", align 8
  store { i32, ptr } %"load cdr1051", ptr %vals1008, align 8
  br label %scan994

next-block1064:                                   ; preds = %actual-value14
  br label %force15

next-block1079:                                   ; No predecessors!
  br label %done16

next-block1091:                                   ; preds = %done16
  br label %compiled-branch18

next-block1104:                                   ; No predecessors!
  br label %primitive-branch17

next-block1113:                                   ; No predecessors!
  br label %after-call19

next-block1164:                                   ; preds = %actual-value23
  br label %force24

next-block1179:                                   ; No predecessors!
  br label %done25

next-block1194:                                   ; preds = %done25
  br label %true-branch20

next-block1200:                                   ; No predecessors!
  %"load stack1201" = load { { i32, ptr }, ptr }, ptr %stack, align 8
  %"previous stack1202" = alloca { { i32, ptr }, ptr }, align 8
  store { { i32, ptr }, ptr } %"load stack1201", ptr %"previous stack1202", align 8
  %"load register1203" = load { i32, ptr }, ptr %env, align 8
  %"save register1204" = insertvalue { { i32, ptr }, ptr } zeroinitializer, { i32, ptr } %"load register1203", 0
  %"save previous stack1205" = insertvalue { { i32, ptr }, ptr } %"save register1204", ptr %"previous stack1202", 1
  store { { i32, ptr }, ptr } %"save previous stack1205", ptr %stack, align 8
  %"object value1206" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  store { i32, ptr } { i32 7, ptr @println.210 }, ptr %"object value1206", align 8
  %"insert value object1207" = insertvalue { i32, ptr } { i32 4, ptr null }, ptr %"object value1206", 1
  %"load register env1208" = load { i32, ptr }, ptr %env, align 8
  %env1216 = alloca { i32, ptr }, align 8
  store { i32, ptr } %"load register env1208", ptr %env1216, align 8
  br label %lookup-entry1209

lookup-entry1209:                                 ; preds = %next-env1212, %next-block1200
  %"load env1217" = load { i32, ptr }, ptr %env1216, align 8
  %get_type1218 = extractvalue { i32, ptr } %"load env1217", 0
  %"is hempty1219" = icmp eq i32 %get_type1218, 0
  br i1 %"is hempty1219", label %error, label %lookup1210

lookup1210:                                       ; preds = %lookup-entry1209
  %"extract-cons:return1220" = extractvalue { i32, ptr } %"load env1217", 1
  %"extract-cons:1221" = load { ptr, ptr }, ptr %"extract-cons:return1220", align 8
  %"get car1222" = extractvalue { ptr, ptr } %"extract-cons:1221", 0
  %"load car1223" = load { i32, ptr }, ptr %"get car1222", align 8
  %vars1224 = alloca { i32, ptr }, align 8
  %vals1225 = alloca { i32, ptr }, align 8
  %"extract-cons:return1226" = extractvalue { i32, ptr } %"load car1223", 1
  %"extract-cons:1227" = load { ptr, ptr }, ptr %"extract-cons:return1226", align 8
  %"get car1228" = extractvalue { ptr, ptr } %"extract-cons:1227", 0
  %"load car1229" = load { i32, ptr }, ptr %"get car1228", align 8
  store { i32, ptr } %"load car1229", ptr %vars1224, align 8
  %"extract-cons:return1230" = extractvalue { i32, ptr } %"load car1223", 1
  %"extract-cons:1231" = load { ptr, ptr }, ptr %"extract-cons:return1230", align 8
  %"get cdr1232" = extractvalue { ptr, ptr } %"extract-cons:1231", 1
  %"load cdr1233" = load { i32, ptr }, ptr %"get cdr1232", align 8
  store { i32, ptr } %"load cdr1233", ptr %vals1225, align 8
  br label %scan1211

scan1211:                                         ; preds = %scan-next1215, %lookup1210
  %vars1234 = load { i32, ptr }, ptr %vars1224, align 8
  %get_type1235 = extractvalue { i32, ptr } %vars1234, 0
  %"is hempty1236" = icmp eq i32 %get_type1235, 0
  br i1 %"is hempty1236", label %next-env1212, label %check1213

next-env1212:                                     ; preds = %scan1211
  %"extract-cons:return1237" = extractvalue { i32, ptr } %"load env1217", 1
  %"extract-cons:1238" = load { ptr, ptr }, ptr %"extract-cons:return1237", align 8
  %"get cdr1239" = extractvalue { ptr, ptr } %"extract-cons:1238", 1
  %"load cdr1240" = load { i32, ptr }, ptr %"get cdr1239", align 8
  store { i32, ptr } %"load cdr1240", ptr %env1216, align 8
  br label %lookup-entry1209

check1213:                                        ; preds = %scan1211
  %"load vars1241" = load { i32, ptr }, ptr %vars1224, align 8
  %"extract-cons:return1242" = extractvalue { i32, ptr } %"load vars1241", 1
  %"extract-cons:1243" = load { ptr, ptr }, ptr %"extract-cons:return1242", align 8
  %"get car1244" = extractvalue { ptr, ptr } %"extract-cons:1243", 0
  %"load car1245" = load { i32, ptr }, ptr %"get car1244", align 8
  %"extract-symbol:return1246" = extractvalue { i32, ptr } %"insert value object1207", 1
  %"extract-symbol:1247" = load { i32, ptr }, ptr %"extract-symbol:return1246", align 8
  %"extract-symbol:return1248" = extractvalue { i32, ptr } %"load car1245", 1
  %"extract-symbol:1249" = load { i32, ptr }, ptr %"extract-symbol:return1248", align 8
  %"get str length1250" = extractvalue { i32, ptr } %"extract-symbol:1247", 0
  %"get str length1251" = extractvalue { i32, ptr } %"extract-symbol:1249", 0
  %"get str1252" = extractvalue { i32, ptr } %"extract-symbol:1247", 1
  %"get str1253" = extractvalue { i32, ptr } %"extract-symbol:1249", 1
  %"len matches1254" = icmp eq i32 %"get str length1250", %"get str length1251"
  %smaller1255 = icmp slt i32 %"get str length1250", %"get str length1251"
  %"str smallaest size1256" = select i1 %smaller1255, i32 %"get str length1250", i32 %"get str length1251"
  %strcmp1257 = call i32 @strncmp(ptr %"get str1252", ptr %"get str1253", i32 %"str smallaest size1256")
  %"is same string1258" = icmp eq i32 %strcmp1257, 0
  %"eq?1259" = and i1 %"len matches1254", %"is same string1258"
  br i1 %"eq?1259", label %found1214, label %scan-next1215

found1214:                                        ; preds = %check1213
  %"load vals1269" = load { i32, ptr }, ptr %vals1225, align 8
  %"extract-cons:return1270" = extractvalue { i32, ptr } %"load vals1269", 1
  %"extract-cons:1271" = load { ptr, ptr }, ptr %"extract-cons:return1270", align 8
  %"get car1272" = extractvalue { ptr, ptr } %"extract-cons:1271", 0
  %"load car1273" = load { i32, ptr }, ptr %"get car1272", align 8
  store { i32, ptr } %"load car1273", ptr %thunk, align 8
  br label %actual-value26

scan-next1215:                                    ; preds = %check1213
  %"load vals1260" = load { i32, ptr }, ptr %vals1225, align 8
  %"extract-cons:return1261" = extractvalue { i32, ptr } %"load vars1241", 1
  %"extract-cons:1262" = load { ptr, ptr }, ptr %"extract-cons:return1261", align 8
  %"get cdr1263" = extractvalue { ptr, ptr } %"extract-cons:1262", 1
  %"load cdr1264" = load { i32, ptr }, ptr %"get cdr1263", align 8
  store { i32, ptr } %"load cdr1264", ptr %vars1224, align 8
  %"extract-cons:return1265" = extractvalue { i32, ptr } %"load vals1260", 1
  %"extract-cons:1266" = load { ptr, ptr }, ptr %"extract-cons:return1265", align 8
  %"get cdr1267" = extractvalue { ptr, ptr } %"extract-cons:1266", 1
  %"load cdr1268" = load { i32, ptr }, ptr %"get cdr1267", align 8
  store { i32, ptr } %"load cdr1268", ptr %vals1225, align 8
  br label %scan1211

next-block1281:                                   ; preds = %actual-value26
  br label %force27

next-block1296:                                   ; No predecessors!
  br label %done28

next-block1308:                                   ; preds = %done28
  br label %compiled-branch35

next-block1321:                                   ; No predecessors!
  br label %thunk32

next-block1331:                                   ; No predecessors!
  br label %after-label33

next-block1349:                                   ; No predecessors!
  br label %primitive-branch34

next-block1369:                                   ; preds = %actual-value29
  br label %force30

next-block1384:                                   ; No predecessors!
  br label %done31

lookup-entry1415:                                 ; preds = %next-env1418, %false-branch21
  %"load env1423" = load { i32, ptr }, ptr %env1422, align 8
  %get_type1424 = extractvalue { i32, ptr } %"load env1423", 0
  %"is hempty1425" = icmp eq i32 %get_type1424, 0
  br i1 %"is hempty1425", label %error, label %lookup1416

lookup1416:                                       ; preds = %lookup-entry1415
  %"extract-cons:return1426" = extractvalue { i32, ptr } %"load env1423", 1
  %"extract-cons:1427" = load { ptr, ptr }, ptr %"extract-cons:return1426", align 8
  %"get car1428" = extractvalue { ptr, ptr } %"extract-cons:1427", 0
  %"load car1429" = load { i32, ptr }, ptr %"get car1428", align 8
  %vars1430 = alloca { i32, ptr }, align 8
  %vals1431 = alloca { i32, ptr }, align 8
  %"extract-cons:return1432" = extractvalue { i32, ptr } %"load car1429", 1
  %"extract-cons:1433" = load { ptr, ptr }, ptr %"extract-cons:return1432", align 8
  %"get car1434" = extractvalue { ptr, ptr } %"extract-cons:1433", 0
  %"load car1435" = load { i32, ptr }, ptr %"get car1434", align 8
  store { i32, ptr } %"load car1435", ptr %vars1430, align 8
  %"extract-cons:return1436" = extractvalue { i32, ptr } %"load car1429", 1
  %"extract-cons:1437" = load { ptr, ptr }, ptr %"extract-cons:return1436", align 8
  %"get cdr1438" = extractvalue { ptr, ptr } %"extract-cons:1437", 1
  %"load cdr1439" = load { i32, ptr }, ptr %"get cdr1438", align 8
  store { i32, ptr } %"load cdr1439", ptr %vals1431, align 8
  br label %scan1417

scan1417:                                         ; preds = %scan-next1421, %lookup1416
  %vars1440 = load { i32, ptr }, ptr %vars1430, align 8
  %get_type1441 = extractvalue { i32, ptr } %vars1440, 0
  %"is hempty1442" = icmp eq i32 %get_type1441, 0
  br i1 %"is hempty1442", label %next-env1418, label %check1419

next-env1418:                                     ; preds = %scan1417
  %"extract-cons:return1443" = extractvalue { i32, ptr } %"load env1423", 1
  %"extract-cons:1444" = load { ptr, ptr }, ptr %"extract-cons:return1443", align 8
  %"get cdr1445" = extractvalue { ptr, ptr } %"extract-cons:1444", 1
  %"load cdr1446" = load { i32, ptr }, ptr %"get cdr1445", align 8
  store { i32, ptr } %"load cdr1446", ptr %env1422, align 8
  br label %lookup-entry1415

check1419:                                        ; preds = %scan1417
  %"load vars1447" = load { i32, ptr }, ptr %vars1430, align 8
  %"extract-cons:return1448" = extractvalue { i32, ptr } %"load vars1447", 1
  %"extract-cons:1449" = load { ptr, ptr }, ptr %"extract-cons:return1448", align 8
  %"get car1450" = extractvalue { ptr, ptr } %"extract-cons:1449", 0
  %"load car1451" = load { i32, ptr }, ptr %"get car1450", align 8
  %"extract-symbol:return1452" = extractvalue { i32, ptr } %"insert value object1413", 1
  %"extract-symbol:1453" = load { i32, ptr }, ptr %"extract-symbol:return1452", align 8
  %"extract-symbol:return1454" = extractvalue { i32, ptr } %"load car1451", 1
  %"extract-symbol:1455" = load { i32, ptr }, ptr %"extract-symbol:return1454", align 8
  %"get str length1456" = extractvalue { i32, ptr } %"extract-symbol:1453", 0
  %"get str length1457" = extractvalue { i32, ptr } %"extract-symbol:1455", 0
  %"get str1458" = extractvalue { i32, ptr } %"extract-symbol:1453", 1
  %"get str1459" = extractvalue { i32, ptr } %"extract-symbol:1455", 1
  %"len matches1460" = icmp eq i32 %"get str length1456", %"get str length1457"
  %smaller1461 = icmp slt i32 %"get str length1456", %"get str length1457"
  %"str smallaest size1462" = select i1 %smaller1461, i32 %"get str length1456", i32 %"get str length1457"
  %strcmp1463 = call i32 @strncmp(ptr %"get str1458", ptr %"get str1459", i32 %"str smallaest size1462")
  %"is same string1464" = icmp eq i32 %strcmp1463, 0
  %"eq?1465" = and i1 %"len matches1460", %"is same string1464"
  br i1 %"eq?1465", label %found1420, label %scan-next1421

found1420:                                        ; preds = %check1419
  %"load vals1475" = load { i32, ptr }, ptr %vals1431, align 8
  %"extract-cons:return1476" = extractvalue { i32, ptr } %"load vals1475", 1
  %"extract-cons:1477" = load { ptr, ptr }, ptr %"extract-cons:return1476", align 8
  %"get car1478" = extractvalue { ptr, ptr } %"extract-cons:1477", 0
  %"load car1479" = load { i32, ptr }, ptr %"get car1478", align 8
  store { i32, ptr } %"load car1479", ptr %thunk, align 8
  br label %actual-value37

scan-next1421:                                    ; preds = %check1419
  %"load vals1466" = load { i32, ptr }, ptr %vals1431, align 8
  %"extract-cons:return1467" = extractvalue { i32, ptr } %"load vars1447", 1
  %"extract-cons:1468" = load { ptr, ptr }, ptr %"extract-cons:return1467", align 8
  %"get cdr1469" = extractvalue { ptr, ptr } %"extract-cons:1468", 1
  %"load cdr1470" = load { i32, ptr }, ptr %"get cdr1469", align 8
  store { i32, ptr } %"load cdr1470", ptr %vars1430, align 8
  %"extract-cons:return1471" = extractvalue { i32, ptr } %"load vals1466", 1
  %"extract-cons:1472" = load { ptr, ptr }, ptr %"extract-cons:return1471", align 8
  %"get cdr1473" = extractvalue { ptr, ptr } %"extract-cons:1472", 1
  %"load cdr1474" = load { i32, ptr }, ptr %"get cdr1473", align 8
  store { i32, ptr } %"load cdr1474", ptr %vals1431, align 8
  br label %scan1417

next-block1487:                                   ; preds = %actual-value37
  br label %force38

next-block1502:                                   ; No predecessors!
  br label %done39

next-block1514:                                   ; preds = %done39
  br label %compiled-branch46

next-block1527:                                   ; No predecessors!
  br label %thunk43

next-block1537:                                   ; No predecessors!
  br label %after-label44

next-block1555:                                   ; No predecessors!
  br label %primitive-branch45

next-block1575:                                   ; preds = %actual-value40
  br label %force41

next-block1590:                                   ; No predecessors!
  br label %done42

next-block1613:                                   ; No predecessors!
  br label %after-call47

lookup-entry1622:                                 ; preds = %next-env1625, %after-if22
  %"load env1630" = load { i32, ptr }, ptr %env1629, align 8
  %get_type1631 = extractvalue { i32, ptr } %"load env1630", 0
  %"is hempty1632" = icmp eq i32 %get_type1631, 0
  br i1 %"is hempty1632", label %error, label %lookup1623

lookup1623:                                       ; preds = %lookup-entry1622
  %"extract-cons:return1633" = extractvalue { i32, ptr } %"load env1630", 1
  %"extract-cons:1634" = load { ptr, ptr }, ptr %"extract-cons:return1633", align 8
  %"get car1635" = extractvalue { ptr, ptr } %"extract-cons:1634", 0
  %"load car1636" = load { i32, ptr }, ptr %"get car1635", align 8
  %vars1637 = alloca { i32, ptr }, align 8
  %vals1638 = alloca { i32, ptr }, align 8
  %"extract-cons:return1639" = extractvalue { i32, ptr } %"load car1636", 1
  %"extract-cons:1640" = load { ptr, ptr }, ptr %"extract-cons:return1639", align 8
  %"get car1641" = extractvalue { ptr, ptr } %"extract-cons:1640", 0
  %"load car1642" = load { i32, ptr }, ptr %"get car1641", align 8
  store { i32, ptr } %"load car1642", ptr %vars1637, align 8
  %"extract-cons:return1643" = extractvalue { i32, ptr } %"load car1636", 1
  %"extract-cons:1644" = load { ptr, ptr }, ptr %"extract-cons:return1643", align 8
  %"get cdr1645" = extractvalue { ptr, ptr } %"extract-cons:1644", 1
  %"load cdr1646" = load { i32, ptr }, ptr %"get cdr1645", align 8
  store { i32, ptr } %"load cdr1646", ptr %vals1638, align 8
  br label %scan1624

scan1624:                                         ; preds = %scan-next1628, %lookup1623
  %vars1647 = load { i32, ptr }, ptr %vars1637, align 8
  %get_type1648 = extractvalue { i32, ptr } %vars1647, 0
  %"is hempty1649" = icmp eq i32 %get_type1648, 0
  br i1 %"is hempty1649", label %next-env1625, label %check1626

next-env1625:                                     ; preds = %scan1624
  %"extract-cons:return1650" = extractvalue { i32, ptr } %"load env1630", 1
  %"extract-cons:1651" = load { ptr, ptr }, ptr %"extract-cons:return1650", align 8
  %"get cdr1652" = extractvalue { ptr, ptr } %"extract-cons:1651", 1
  %"load cdr1653" = load { i32, ptr }, ptr %"get cdr1652", align 8
  store { i32, ptr } %"load cdr1653", ptr %env1629, align 8
  br label %lookup-entry1622

check1626:                                        ; preds = %scan1624
  %"load vars1654" = load { i32, ptr }, ptr %vars1637, align 8
  %"extract-cons:return1655" = extractvalue { i32, ptr } %"load vars1654", 1
  %"extract-cons:1656" = load { ptr, ptr }, ptr %"extract-cons:return1655", align 8
  %"get car1657" = extractvalue { ptr, ptr } %"extract-cons:1656", 0
  %"load car1658" = load { i32, ptr }, ptr %"get car1657", align 8
  %"extract-symbol:return1659" = extractvalue { i32, ptr } %"insert value object1620", 1
  %"extract-symbol:1660" = load { i32, ptr }, ptr %"extract-symbol:return1659", align 8
  %"extract-symbol:return1661" = extractvalue { i32, ptr } %"load car1658", 1
  %"extract-symbol:1662" = load { i32, ptr }, ptr %"extract-symbol:return1661", align 8
  %"get str length1663" = extractvalue { i32, ptr } %"extract-symbol:1660", 0
  %"get str length1664" = extractvalue { i32, ptr } %"extract-symbol:1662", 0
  %"get str1665" = extractvalue { i32, ptr } %"extract-symbol:1660", 1
  %"get str1666" = extractvalue { i32, ptr } %"extract-symbol:1662", 1
  %"len matches1667" = icmp eq i32 %"get str length1663", %"get str length1664"
  %smaller1668 = icmp slt i32 %"get str length1663", %"get str length1664"
  %"str smallaest size1669" = select i1 %smaller1668, i32 %"get str length1663", i32 %"get str length1664"
  %strcmp1670 = call i32 @strncmp(ptr %"get str1665", ptr %"get str1666", i32 %"str smallaest size1669")
  %"is same string1671" = icmp eq i32 %strcmp1670, 0
  %"eq?1672" = and i1 %"len matches1667", %"is same string1671"
  br i1 %"eq?1672", label %found1627, label %scan-next1628

found1627:                                        ; preds = %check1626
  %"load vals1682" = load { i32, ptr }, ptr %vals1638, align 8
  %"extract-cons:return1683" = extractvalue { i32, ptr } %"load vals1682", 1
  %"extract-cons:1684" = load { ptr, ptr }, ptr %"extract-cons:return1683", align 8
  %"get car1685" = extractvalue { ptr, ptr } %"extract-cons:1684", 0
  %"load car1686" = load { i32, ptr }, ptr %"get car1685", align 8
  store { i32, ptr } %"load car1686", ptr %thunk, align 8
  br label %actual-value48

scan-next1628:                                    ; preds = %check1626
  %"load vals1673" = load { i32, ptr }, ptr %vals1638, align 8
  %"extract-cons:return1674" = extractvalue { i32, ptr } %"load vars1654", 1
  %"extract-cons:1675" = load { ptr, ptr }, ptr %"extract-cons:return1674", align 8
  %"get cdr1676" = extractvalue { ptr, ptr } %"extract-cons:1675", 1
  %"load cdr1677" = load { i32, ptr }, ptr %"get cdr1676", align 8
  store { i32, ptr } %"load cdr1677", ptr %vars1637, align 8
  %"extract-cons:return1678" = extractvalue { i32, ptr } %"load vals1673", 1
  %"extract-cons:1679" = load { ptr, ptr }, ptr %"extract-cons:return1678", align 8
  %"get cdr1680" = extractvalue { ptr, ptr } %"extract-cons:1679", 1
  %"load cdr1681" = load { i32, ptr }, ptr %"get cdr1680", align 8
  store { i32, ptr } %"load cdr1681", ptr %vals1638, align 8
  br label %scan1624

next-block1694:                                   ; preds = %actual-value48
  br label %force49

next-block1709:                                   ; No predecessors!
  br label %done50

next-block1721:                                   ; preds = %done50
  br label %compiled-branch57

next-block1734:                                   ; No predecessors!
  br label %thunk54

next-block1744:                                   ; No predecessors!
  br label %after-label55

next-block1762:                                   ; No predecessors!
  br label %primitive-branch56

next-block1782:                                   ; preds = %actual-value51
  br label %force52

next-block1797:                                   ; No predecessors!
  br label %done53

lookup-entry1828:                                 ; preds = %next-env1831, %x
  %"load env1836" = load { i32, ptr }, ptr %env1835, align 8
  %get_type1837 = extractvalue { i32, ptr } %"load env1836", 0
  %"is hempty1838" = icmp eq i32 %get_type1837, 0
  br i1 %"is hempty1838", label %error, label %lookup1829

lookup1829:                                       ; preds = %lookup-entry1828
  %"extract-cons:return1839" = extractvalue { i32, ptr } %"load env1836", 1
  %"extract-cons:1840" = load { ptr, ptr }, ptr %"extract-cons:return1839", align 8
  %"get car1841" = extractvalue { ptr, ptr } %"extract-cons:1840", 0
  %"load car1842" = load { i32, ptr }, ptr %"get car1841", align 8
  %vars1843 = alloca { i32, ptr }, align 8
  %vals1844 = alloca { i32, ptr }, align 8
  %"extract-cons:return1845" = extractvalue { i32, ptr } %"load car1842", 1
  %"extract-cons:1846" = load { ptr, ptr }, ptr %"extract-cons:return1845", align 8
  %"get car1847" = extractvalue { ptr, ptr } %"extract-cons:1846", 0
  %"load car1848" = load { i32, ptr }, ptr %"get car1847", align 8
  store { i32, ptr } %"load car1848", ptr %vars1843, align 8
  %"extract-cons:return1849" = extractvalue { i32, ptr } %"load car1842", 1
  %"extract-cons:1850" = load { ptr, ptr }, ptr %"extract-cons:return1849", align 8
  %"get cdr1851" = extractvalue { ptr, ptr } %"extract-cons:1850", 1
  %"load cdr1852" = load { i32, ptr }, ptr %"get cdr1851", align 8
  store { i32, ptr } %"load cdr1852", ptr %vals1844, align 8
  br label %scan1830

scan1830:                                         ; preds = %scan-next1834, %lookup1829
  %vars1853 = load { i32, ptr }, ptr %vars1843, align 8
  %get_type1854 = extractvalue { i32, ptr } %vars1853, 0
  %"is hempty1855" = icmp eq i32 %get_type1854, 0
  br i1 %"is hempty1855", label %next-env1831, label %check1832

next-env1831:                                     ; preds = %scan1830
  %"extract-cons:return1856" = extractvalue { i32, ptr } %"load env1836", 1
  %"extract-cons:1857" = load { ptr, ptr }, ptr %"extract-cons:return1856", align 8
  %"get cdr1858" = extractvalue { ptr, ptr } %"extract-cons:1857", 1
  %"load cdr1859" = load { i32, ptr }, ptr %"get cdr1858", align 8
  store { i32, ptr } %"load cdr1859", ptr %env1835, align 8
  br label %lookup-entry1828

check1832:                                        ; preds = %scan1830
  %"load vars1860" = load { i32, ptr }, ptr %vars1843, align 8
  %"extract-cons:return1861" = extractvalue { i32, ptr } %"load vars1860", 1
  %"extract-cons:1862" = load { ptr, ptr }, ptr %"extract-cons:return1861", align 8
  %"get car1863" = extractvalue { ptr, ptr } %"extract-cons:1862", 0
  %"load car1864" = load { i32, ptr }, ptr %"get car1863", align 8
  %"extract-symbol:return1865" = extractvalue { i32, ptr } %"insert value object1826", 1
  %"extract-symbol:1866" = load { i32, ptr }, ptr %"extract-symbol:return1865", align 8
  %"extract-symbol:return1867" = extractvalue { i32, ptr } %"load car1864", 1
  %"extract-symbol:1868" = load { i32, ptr }, ptr %"extract-symbol:return1867", align 8
  %"get str length1869" = extractvalue { i32, ptr } %"extract-symbol:1866", 0
  %"get str length1870" = extractvalue { i32, ptr } %"extract-symbol:1868", 0
  %"get str1871" = extractvalue { i32, ptr } %"extract-symbol:1866", 1
  %"get str1872" = extractvalue { i32, ptr } %"extract-symbol:1868", 1
  %"len matches1873" = icmp eq i32 %"get str length1869", %"get str length1870"
  %smaller1874 = icmp slt i32 %"get str length1869", %"get str length1870"
  %"str smallaest size1875" = select i1 %smaller1874, i32 %"get str length1869", i32 %"get str length1870"
  %strcmp1876 = call i32 @strncmp(ptr %"get str1871", ptr %"get str1872", i32 %"str smallaest size1875")
  %"is same string1877" = icmp eq i32 %strcmp1876, 0
  %"eq?1878" = and i1 %"len matches1873", %"is same string1877"
  br i1 %"eq?1878", label %found1833, label %scan-next1834

found1833:                                        ; preds = %check1832
  %"load vals1888" = load { i32, ptr }, ptr %vals1844, align 8
  %"extract-cons:return1889" = extractvalue { i32, ptr } %"load vals1888", 1
  %"extract-cons:1890" = load { ptr, ptr }, ptr %"extract-cons:return1889", align 8
  %"get car1891" = extractvalue { ptr, ptr } %"extract-cons:1890", 0
  %"load car1892" = load { i32, ptr }, ptr %"get car1891", align 8
  store { i32, ptr } %"load car1892", ptr %thunk, align 8
  br label %actual-value59

scan-next1834:                                    ; preds = %check1832
  %"load vals1879" = load { i32, ptr }, ptr %vals1844, align 8
  %"extract-cons:return1880" = extractvalue { i32, ptr } %"load vars1860", 1
  %"extract-cons:1881" = load { ptr, ptr }, ptr %"extract-cons:return1880", align 8
  %"get cdr1882" = extractvalue { ptr, ptr } %"extract-cons:1881", 1
  %"load cdr1883" = load { i32, ptr }, ptr %"get cdr1882", align 8
  store { i32, ptr } %"load cdr1883", ptr %vars1843, align 8
  %"extract-cons:return1884" = extractvalue { i32, ptr } %"load vals1879", 1
  %"extract-cons:1885" = load { ptr, ptr }, ptr %"extract-cons:return1884", align 8
  %"get cdr1886" = extractvalue { ptr, ptr } %"extract-cons:1885", 1
  %"load cdr1887" = load { i32, ptr }, ptr %"get cdr1886", align 8
  store { i32, ptr } %"load cdr1887", ptr %vals1844, align 8
  br label %scan1830

next-block1900:                                   ; preds = %actual-value59
  br label %force60

next-block1915:                                   ; No predecessors!
  br label %done61

next-block1927:                                   ; preds = %done61
  br label %compiled-branch68

next-block1940:                                   ; No predecessors!
  br label %thunk65

next-block1950:                                   ; No predecessors!
  br label %after-label66

next-block1968:                                   ; No predecessors!
  br label %primitive-branch67

next-block1988:                                   ; preds = %actual-value62
  br label %force63

next-block2003:                                   ; No predecessors!
  br label %done64
}

define void @print-obj({ i32, ptr } %0) {
entry:
  %get_type = extractvalue { i32, ptr } %0, 0
  switch i32 %get_type, label %error [
    i32 0, label %"print:empty"
    i32 1, label %"print:bool"
    i32 2, label %"print:number"
    i32 3, label %"print:string"
    i32 4, label %"print:symbol"
    i32 5, label %"print:label"
    i32 6, label %"print:cons"
    i32 7, label %"print:primitive"
    i32 8, label %"print:thunk"
    i32 9, label %"print:lambda"
  ]

error:                                            ; preds = %entry
  %"error phi" = phi { ptr, i32 } [ { ptr @"error exit", i32 1 }, %entry ]
  %error_msg = extractvalue { ptr, i32 } %"error phi", 0
  %error_code = extractvalue { ptr, i32 } %"error phi", 1
  %print = call i32 (ptr, ...) @printf(ptr %error_msg)
  call void @exit(i32 %error_code)
  unreachable

"print:empty":                                    ; preds = %entry
  %"printf debug" = call i32 (ptr, ...) @printf(ptr @printf-format)
  ret void

"print:bool":                                     ; preds = %entry
  %"extract-bool:return" = extractvalue { i32, ptr } %0, 1
  %"extract-bool:" = load i1, ptr %"extract-bool:return", align 1
  %"bool value" = select i1 %"extract-bool:", ptr @true, ptr @false
  %"printf debug1" = call i32 (ptr, ...) @printf(ptr %"bool value")
  ret void

"print:number":                                   ; preds = %entry
  %"extract-number:return" = extractvalue { i32, ptr } %0, 1
  %"extract-number:" = load double, ptr %"extract-number:return", align 8
  %"printf debug2" = call i32 (ptr, ...) @printf(ptr @printf-format.1, double %"extract-number:")
  ret void

"print:string":                                   ; preds = %entry
  %"extract-symbol:return" = extractvalue { i32, ptr } %0, 1
  %"extract-symbol:" = load { i32, ptr }, ptr %"extract-symbol:return", align 8
  %strlen = extractvalue { i32, ptr } %"extract-symbol:", 0
  %strlen3 = extractvalue { i32, ptr } %"extract-symbol:", 1
  %"printf debug4" = call i32 (ptr, ...) @printf(ptr @printf-format.2, i32 %strlen, ptr %strlen3)
  ret void

"print:symbol":                                   ; preds = %entry
  %"extract-symbol:return5" = extractvalue { i32, ptr } %0, 1
  %"extract-symbol:6" = load { i32, ptr }, ptr %"extract-symbol:return5", align 8
  %strlen7 = extractvalue { i32, ptr } %"extract-symbol:6", 0
  %strlen8 = extractvalue { i32, ptr } %"extract-symbol:6", 1
  %"printf debug9" = call i32 (ptr, ...) @printf(ptr @printf-format.3, i32 %strlen7, ptr %strlen8)
  ret void

"print:label":                                    ; preds = %entry
  %"printf debug10" = call i32 (ptr, ...) @printf(ptr @printf-format.4)
  ret void

"print:cons":                                     ; preds = %entry
  %"extract-cons:return" = extractvalue { i32, ptr } %0, 1
  %"extract-cons:" = load { ptr, ptr }, ptr %"extract-cons:return", align 8
  %"get car" = extractvalue { ptr, ptr } %"extract-cons:", 0
  %"load car" = load { i32, ptr }, ptr %"get car", align 8
  %"extract-cons:return11" = extractvalue { i32, ptr } %0, 1
  %"extract-cons:12" = load { ptr, ptr }, ptr %"extract-cons:return11", align 8
  %"get cdr" = extractvalue { ptr, ptr } %"extract-cons:12", 1
  %"load cdr" = load { i32, ptr }, ptr %"get cdr", align 8
  %"printf debug13" = call i32 (ptr, ...) @printf(ptr @printf-format.5)
  call void @print-obj({ i32, ptr } %"load car")
  %"printf debug14" = call i32 (ptr, ...) @printf(ptr @printf-format.6)
  call void @print-obj({ i32, ptr } %"load cdr")
  %"printf debug15" = call i32 (ptr, ...) @printf(ptr @printf-format.7)
  ret void

"print:primitive":                                ; preds = %entry
  %"printf debug16" = call i32 (ptr, ...) @printf(ptr @printf-format.8)
  ret void

"print:thunk":                                    ; preds = %entry
  %"printf debug17" = call i32 (ptr, ...) @printf(ptr @printf-format.9)
  ret void

"print:lambda":                                   ; preds = %entry
  %"printf debug18" = call i32 (ptr, ...) @printf(ptr @printf-format.10)
  ret void
}

define i1 @eq-obj({ i32, ptr } %0, { i32, ptr } %1) {
entry:
  %get_type = extractvalue { i32, ptr } %0, 0
  %get_type1 = extractvalue { i32, ptr } %1, 0
  %"same type?" = icmp eq i32 %get_type, %get_type1
  br i1 %"same type?", label %"same type", label %"not same type"

error:                                            ; preds = %primitive, %lambda, %thunk, %label, %"same type"
  %"error phi" = phi { ptr, i32 } [ { ptr @"error exit.11", i32 1 }, %"same type" ], [ { ptr @"error exit.13", i32 1 }, %label ], [ { ptr @"error exit.14", i32 1 }, %thunk ], [ { ptr @"error exit.15", i32 1 }, %lambda ], [ { ptr @"error exit.16", i32 1 }, %primitive ]
  %error_msg = extractvalue { ptr, i32 } %"error phi", 0
  %error_code = extractvalue { ptr, i32 } %"error phi", 1
  %print = call i32 (ptr, ...) @printf(ptr %error_msg)
  call void @exit(i32 %error_code)
  unreachable

"same type":                                      ; preds = %entry
  switch i32 %get_type, label %error [
    i32 0, label %hempty
    i32 1, label %bool
    i32 2, label %number
    i32 3, label %string
    i32 4, label %symbol
    i32 6, label %cons
    i32 9, label %lambda
    i32 7, label %primitive
    i32 5, label %label
    i32 8, label %thunk
  ]

"not same type":                                  ; preds = %entry
  ret i1 false

hempty:                                           ; preds = %"same type"
  %"printf debug" = call i32 (ptr, ...) @printf(ptr @printf-format.12)
  ret i1 true

string:                                           ; preds = %"same type"
  %"extract-string:return" = extractvalue { i32, ptr } %0, 1
  %"extract-string:" = load { i32, ptr }, ptr %"extract-string:return", align 8
  %"extract-string:return6" = extractvalue { i32, ptr } %1, 1
  %"extract-string:7" = load { i32, ptr }, ptr %"extract-string:return6", align 8
  %"get str length" = extractvalue { i32, ptr } %"extract-string:", 0
  %"get str length8" = extractvalue { i32, ptr } %"extract-string:7", 0
  %"get str" = extractvalue { i32, ptr } %"extract-string:", 1
  %"get str9" = extractvalue { i32, ptr } %"extract-string:7", 1
  %"len matches" = icmp eq i32 %"get str length", %"get str length8"
  %smaller = icmp slt i32 %"get str length", %"get str length8"
  %"str smallaest size" = select i1 %smaller, i32 %"get str length", i32 %"get str length8"
  %strcmp = call i32 @strncmp(ptr %"get str", ptr %"get str9", i32 %"str smallaest size")
  %"is same string" = icmp eq i32 %strcmp, 0
  %"eq?" = and i1 %"len matches", %"is same string"
  ret i1 %"eq?"

number:                                           ; preds = %"same type"
  %"extract-number:return" = extractvalue { i32, ptr } %0, 1
  %"extract-number:" = load double, ptr %"extract-number:return", align 8
  %"extract-number:return4" = extractvalue { i32, ptr } %1, 1
  %"extract-number:5" = load double, ptr %"extract-number:return4", align 8
  %"number compare" = fcmp oeq double %"extract-number:", %"extract-number:5"
  ret i1 %"number compare"

bool:                                             ; preds = %"same type"
  %"extract-bool:return" = extractvalue { i32, ptr } %0, 1
  %"extract-bool:" = load i1, ptr %"extract-bool:return", align 1
  %"extract-bool:return2" = extractvalue { i32, ptr } %1, 1
  %"extract-bool:3" = load i1, ptr %"extract-bool:return2", align 1
  %"bool compare" = icmp eq i1 %"extract-bool:", %"extract-bool:3"
  ret i1 %"bool compare"

symbol:                                           ; preds = %"same type"
  %"extract-symbol:return" = extractvalue { i32, ptr } %0, 1
  %"extract-symbol:" = load { i32, ptr }, ptr %"extract-symbol:return", align 8
  %"extract-symbol:return10" = extractvalue { i32, ptr } %1, 1
  %"extract-symbol:11" = load { i32, ptr }, ptr %"extract-symbol:return10", align 8
  %"get str length12" = extractvalue { i32, ptr } %"extract-symbol:", 0
  %"get str length13" = extractvalue { i32, ptr } %"extract-symbol:11", 0
  %"get str14" = extractvalue { i32, ptr } %"extract-symbol:", 1
  %"get str15" = extractvalue { i32, ptr } %"extract-symbol:11", 1
  %"len matches16" = icmp eq i32 %"get str length12", %"get str length13"
  %smaller17 = icmp slt i32 %"get str length12", %"get str length13"
  %"str smallaest size18" = select i1 %smaller17, i32 %"get str length12", i32 %"get str length13"
  %strcmp19 = call i32 @strncmp(ptr %"get str14", ptr %"get str15", i32 %"str smallaest size18")
  %"is same string20" = icmp eq i32 %strcmp19, 0
  %"eq?21" = and i1 %"len matches16", %"is same string20"
  ret i1 %"eq?21"

cons:                                             ; preds = %"same type"
  %"extract-cons:return" = extractvalue { i32, ptr } %0, 1
  %"extract-cons:" = load { ptr, ptr }, ptr %"extract-cons:return", align 8
  %"get car" = extractvalue { ptr, ptr } %"extract-cons:", 0
  %"load car" = load { i32, ptr }, ptr %"get car", align 8
  %"extract-cons:return22" = extractvalue { i32, ptr } %1, 1
  %"extract-cons:23" = load { ptr, ptr }, ptr %"extract-cons:return22", align 8
  %"get car24" = extractvalue { ptr, ptr } %"extract-cons:23", 0
  %"load car25" = load { i32, ptr }, ptr %"get car24", align 8
  %"extract-cons:return26" = extractvalue { i32, ptr } %0, 1
  %"extract-cons:27" = load { ptr, ptr }, ptr %"extract-cons:return26", align 8
  %"get cdr" = extractvalue { ptr, ptr } %"extract-cons:27", 1
  %"load cdr" = load { i32, ptr }, ptr %"get cdr", align 8
  %"extract-cons:return28" = extractvalue { i32, ptr } %1, 1
  %"extract-cons:29" = load { ptr, ptr }, ptr %"extract-cons:return28", align 8
  %"get cdr30" = extractvalue { ptr, ptr } %"extract-cons:29", 1
  %"load cdr31" = load { i32, ptr }, ptr %"get cdr30", align 8
  %"car eq" = call i1 @eq-obj({ i32, ptr } %"load car", { i32, ptr } %"load car25")
  %"cdr eq" = call i1 @eq-obj({ i32, ptr } %"load cdr", { i32, ptr } %"load cdr31")
  %"cons eq?" = and i1 %"car eq", %"cdr eq"
  ret i1 %"cons eq?"

thunk:                                            ; preds = %"same type"
  br label %error

primitive:                                        ; preds = %"same type"
  br label %error

label:                                            ; preds = %"same type"
  br label %error

lambda:                                           ; preds = %"same type"
  br label %error
}

define { i32, ptr } @newline({ i32, ptr } %0) {
entry:
  %"call newline" = call i32 (ptr, ...) @printf(ptr @"\0A")
  ret { i32, ptr } zeroinitializer
}

define { i32, ptr } @cons({ i32, ptr } %0) {
entry:
  %get_type = extractvalue { i32, ptr } %0, 0
  %"extract-cons:cmp-type" = icmp eq i32 %get_type, 6
  br i1 %"extract-cons:cmp-type", label %"extract-cons:return", label %error

error:                                            ; preds = %"extract-cons:return2", %"extract-cons:return", %entry
  %"error phi" = phi { ptr, i32 } [ { ptr @"error exit.17", i32 1 }, %entry ], [ { ptr @"error exit.18", i32 1 }, %"extract-cons:return" ], [ { ptr @"error exit.19", i32 1 }, %"extract-cons:return2" ]
  %error_msg = extractvalue { ptr, i32 } %"error phi", 0
  %error_code = extractvalue { ptr, i32 } %"error phi", 1
  %print = call i32 (ptr, ...) @printf(ptr %error_msg)
  call void @exit(i32 %error_code)
  unreachable

"extract-cons:return":                            ; preds = %entry
  %"extract-cons:return1" = extractvalue { i32, ptr } %0, 1
  %"extract-cons:" = load { ptr, ptr }, ptr %"extract-cons:return1", align 8
  %"get car" = extractvalue { ptr, ptr } %"extract-cons:", 0
  %"load car" = load { i32, ptr }, ptr %"get car", align 8
  %get_type3 = extractvalue { i32, ptr } %0, 0
  %"extract-cons:cmp-type4" = icmp eq i32 %get_type3, 6
  br i1 %"extract-cons:cmp-type4", label %"extract-cons:return2", label %error

"extract-cons:return2":                           ; preds = %"extract-cons:return"
  %"extract-cons:return5" = extractvalue { i32, ptr } %0, 1
  %"extract-cons:6" = load { ptr, ptr }, ptr %"extract-cons:return5", align 8
  %"get cdr" = extractvalue { ptr, ptr } %"extract-cons:6", 1
  %"load cdr" = load { i32, ptr }, ptr %"get cdr", align 8
  %get_type8 = extractvalue { i32, ptr } %"load cdr", 0
  %"extract-cons:cmp-type9" = icmp eq i32 %get_type8, 6
  br i1 %"extract-cons:cmp-type9", label %"extract-cons:return7", label %error

"extract-cons:return7":                           ; preds = %"extract-cons:return2"
  %"extract-cons:return10" = extractvalue { i32, ptr } %"load cdr", 1
  %"extract-cons:11" = load { ptr, ptr }, ptr %"extract-cons:return10", align 8
  %"get car12" = extractvalue { ptr, ptr } %"extract-cons:11", 0
  %"load car13" = load { i32, ptr }, ptr %"get car12", align 8
  %"car ptr" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  %"cdr ptr" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ i32, ptr }, ptr null, i32 1) to i32))
  store { i32, ptr } %"load car", ptr %"car ptr", align 8
  store { i32, ptr } %"load car13", ptr %"cdr ptr", align 8
  %"insert car - cons" = insertvalue { ptr, ptr } zeroinitializer, ptr %"car ptr", 0
  %"insert cdr - cons" = insertvalue { ptr, ptr } %"insert car - cons", ptr %"cdr ptr", 1
  %"object value" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr ({ ptr, ptr }, ptr null, i32 1) to i32))
  store { ptr, ptr } %"insert cdr - cons", ptr %"object value", align 8
  %"insert value object" = insertvalue { i32, ptr } { i32 6, ptr null }, ptr %"object value", 1
  ret { i32, ptr } %"insert value object"
}

declare noalias ptr @malloc(i32)

define { i32, ptr } @eq({ i32, ptr } %0) {
entry:
  %get_type = extractvalue { i32, ptr } %0, 0
  %"extract-cons:cmp-type" = icmp eq i32 %get_type, 6
  br i1 %"extract-cons:cmp-type", label %"extract-cons:return", label %error

error:                                            ; preds = %"extract-cons:return2", %"extract-cons:return", %entry
  %"error phi" = phi { ptr, i32 } [ { ptr @"error exit.20", i32 1 }, %entry ], [ { ptr @"error exit.21", i32 1 }, %"extract-cons:return" ], [ { ptr @"error exit.22", i32 1 }, %"extract-cons:return2" ]
  %error_msg = extractvalue { ptr, i32 } %"error phi", 0
  %error_code = extractvalue { ptr, i32 } %"error phi", 1
  %print = call i32 (ptr, ...) @printf(ptr %error_msg)
  call void @exit(i32 %error_code)
  unreachable

"extract-cons:return":                            ; preds = %entry
  %"extract-cons:return1" = extractvalue { i32, ptr } %0, 1
  %"extract-cons:" = load { ptr, ptr }, ptr %"extract-cons:return1", align 8
  %"get car" = extractvalue { ptr, ptr } %"extract-cons:", 0
  %"load car" = load { i32, ptr }, ptr %"get car", align 8
  %get_type3 = extractvalue { i32, ptr } %0, 0
  %"extract-cons:cmp-type4" = icmp eq i32 %get_type3, 6
  br i1 %"extract-cons:cmp-type4", label %"extract-cons:return2", label %error

"extract-cons:return2":                           ; preds = %"extract-cons:return"
  %"extract-cons:return5" = extractvalue { i32, ptr } %0, 1
  %"extract-cons:6" = load { ptr, ptr }, ptr %"extract-cons:return5", align 8
  %"get cdr" = extractvalue { ptr, ptr } %"extract-cons:6", 1
  %"load cdr" = load { i32, ptr }, ptr %"get cdr", align 8
  %get_type8 = extractvalue { i32, ptr } %"load cdr", 0
  %"extract-cons:cmp-type9" = icmp eq i32 %get_type8, 6
  br i1 %"extract-cons:cmp-type9", label %"extract-cons:return7", label %error

"extract-cons:return7":                           ; preds = %"extract-cons:return2"
  %"extract-cons:return10" = extractvalue { i32, ptr } %"load cdr", 1
  %"extract-cons:11" = load { ptr, ptr }, ptr %"extract-cons:return10", align 8
  %"get car12" = extractvalue { ptr, ptr } %"extract-cons:11", 0
  %"load car13" = load { i32, ptr }, ptr %"get car12", align 8
  %"compare objects" = call i1 @eq-obj({ i32, ptr } %"load car", { i32, ptr } %"load car13")
  %"object value" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr (i1, ptr null, i32 1) to i32))
  store i1 %"compare objects", ptr %"object value", align 1
  %"insert value object" = insertvalue { i32, ptr } { i32 1, ptr null }, ptr %"object value", 1
  ret { i32, ptr } %"insert value object"
}

define { i32, ptr } @"set-car!"({ i32, ptr } %0) {
entry:
  %get_type = extractvalue { i32, ptr } %0, 0
  %"extract-cons:cmp-type" = icmp eq i32 %get_type, 6
  br i1 %"extract-cons:cmp-type", label %"extract-cons:return", label %error

error:                                            ; preds = %"extract-cons:return7", %"extract-cons:return2", %"extract-cons:return", %entry
  %"error phi" = phi { ptr, i32 } [ { ptr @"error exit.23", i32 1 }, %entry ], [ { ptr @"error exit.24", i32 1 }, %"extract-cons:return" ], [ { ptr @"error exit.25", i32 1 }, %"extract-cons:return2" ], [ { ptr @"error exit.26", i32 1 }, %"extract-cons:return7" ]
  %error_msg = extractvalue { ptr, i32 } %"error phi", 0
  %error_code = extractvalue { ptr, i32 } %"error phi", 1
  %print = call i32 (ptr, ...) @printf(ptr %error_msg)
  call void @exit(i32 %error_code)
  unreachable

"extract-cons:return":                            ; preds = %entry
  %"extract-cons:return1" = extractvalue { i32, ptr } %0, 1
  %"extract-cons:" = load { ptr, ptr }, ptr %"extract-cons:return1", align 8
  %"get car" = extractvalue { ptr, ptr } %"extract-cons:", 0
  %"load car" = load { i32, ptr }, ptr %"get car", align 8
  %get_type3 = extractvalue { i32, ptr } %0, 0
  %"extract-cons:cmp-type4" = icmp eq i32 %get_type3, 6
  br i1 %"extract-cons:cmp-type4", label %"extract-cons:return2", label %error

"extract-cons:return2":                           ; preds = %"extract-cons:return"
  %"extract-cons:return5" = extractvalue { i32, ptr } %0, 1
  %"extract-cons:6" = load { ptr, ptr }, ptr %"extract-cons:return5", align 8
  %"get cdr" = extractvalue { ptr, ptr } %"extract-cons:6", 1
  %"load cdr" = load { i32, ptr }, ptr %"get cdr", align 8
  %get_type8 = extractvalue { i32, ptr } %"load cdr", 0
  %"extract-cons:cmp-type9" = icmp eq i32 %get_type8, 6
  br i1 %"extract-cons:cmp-type9", label %"extract-cons:return7", label %error

"extract-cons:return7":                           ; preds = %"extract-cons:return2"
  %"extract-cons:return10" = extractvalue { i32, ptr } %"load cdr", 1
  %"extract-cons:11" = load { ptr, ptr }, ptr %"extract-cons:return10", align 8
  %"get car12" = extractvalue { ptr, ptr } %"extract-cons:11", 0
  %"load car13" = load { i32, ptr }, ptr %"get car12", align 8
  %get_type15 = extractvalue { i32, ptr } %"load car", 0
  %"extract-cons:cmp-type16" = icmp eq i32 %get_type15, 6
  br i1 %"extract-cons:cmp-type16", label %"extract-cons:return14", label %error

"extract-cons:return14":                          ; preds = %"extract-cons:return7"
  %"extract-cons:return17" = extractvalue { i32, ptr } %"load car", 1
  %"extract-cons:18" = load { ptr, ptr }, ptr %"extract-cons:return17", align 8
  %"get car19" = extractvalue { ptr, ptr } %"extract-cons:18", 0
  store { i32, ptr } %"load car13", ptr %"get car19", align 8
  ret { i32, ptr } zeroinitializer
}

define { i32, ptr } @"set-cdr!"({ i32, ptr } %0) {
entry:
  %get_type = extractvalue { i32, ptr } %0, 0
  %"extract-cons:cmp-type" = icmp eq i32 %get_type, 6
  br i1 %"extract-cons:cmp-type", label %"extract-cons:return", label %error

error:                                            ; preds = %"extract-cons:return7", %"extract-cons:return2", %"extract-cons:return", %entry
  %"error phi" = phi { ptr, i32 } [ { ptr @"error exit.27", i32 1 }, %entry ], [ { ptr @"error exit.28", i32 1 }, %"extract-cons:return" ], [ { ptr @"error exit.29", i32 1 }, %"extract-cons:return2" ], [ { ptr @"error exit.30", i32 1 }, %"extract-cons:return7" ]
  %error_msg = extractvalue { ptr, i32 } %"error phi", 0
  %error_code = extractvalue { ptr, i32 } %"error phi", 1
  %print = call i32 (ptr, ...) @printf(ptr %error_msg)
  call void @exit(i32 %error_code)
  unreachable

"extract-cons:return":                            ; preds = %entry
  %"extract-cons:return1" = extractvalue { i32, ptr } %0, 1
  %"extract-cons:" = load { ptr, ptr }, ptr %"extract-cons:return1", align 8
  %"get car" = extractvalue { ptr, ptr } %"extract-cons:", 0
  %"load car" = load { i32, ptr }, ptr %"get car", align 8
  %get_type3 = extractvalue { i32, ptr } %0, 0
  %"extract-cons:cmp-type4" = icmp eq i32 %get_type3, 6
  br i1 %"extract-cons:cmp-type4", label %"extract-cons:return2", label %error

"extract-cons:return2":                           ; preds = %"extract-cons:return"
  %"extract-cons:return5" = extractvalue { i32, ptr } %0, 1
  %"extract-cons:6" = load { ptr, ptr }, ptr %"extract-cons:return5", align 8
  %"get cdr" = extractvalue { ptr, ptr } %"extract-cons:6", 1
  %"load cdr" = load { i32, ptr }, ptr %"get cdr", align 8
  %get_type8 = extractvalue { i32, ptr } %"load cdr", 0
  %"extract-cons:cmp-type9" = icmp eq i32 %get_type8, 6
  br i1 %"extract-cons:cmp-type9", label %"extract-cons:return7", label %error

"extract-cons:return7":                           ; preds = %"extract-cons:return2"
  %"extract-cons:return10" = extractvalue { i32, ptr } %"load cdr", 1
  %"extract-cons:11" = load { ptr, ptr }, ptr %"extract-cons:return10", align 8
  %"get car12" = extractvalue { ptr, ptr } %"extract-cons:11", 0
  %"load car13" = load { i32, ptr }, ptr %"get car12", align 8
  %get_type15 = extractvalue { i32, ptr } %"load car", 0
  %"extract-cons:cmp-type16" = icmp eq i32 %get_type15, 6
  br i1 %"extract-cons:cmp-type16", label %"extract-cons:return14", label %error

"extract-cons:return14":                          ; preds = %"extract-cons:return7"
  %"extract-cons:return17" = extractvalue { i32, ptr } %"load car", 1
  %"extract-cons:18" = load { ptr, ptr }, ptr %"extract-cons:return17", align 8
  %"get cdr19" = extractvalue { ptr, ptr } %"extract-cons:18", 1
  store { i32, ptr } %"load car13", ptr %"get cdr19", align 8
  ret { i32, ptr } zeroinitializer
}

define { i32, ptr } @not({ i32, ptr } %0) {
entry:
  %get_type = extractvalue { i32, ptr } %0, 0
  %"extract-cons:cmp-type" = icmp eq i32 %get_type, 6
  br i1 %"extract-cons:cmp-type", label %"extract-cons:return", label %error

error:                                            ; preds = %entry
  %"error phi" = phi { ptr, i32 } [ { ptr @"error exit.31", i32 1 }, %entry ]
  %error_msg = extractvalue { ptr, i32 } %"error phi", 0
  %error_code = extractvalue { ptr, i32 } %"error phi", 1
  %print = call i32 (ptr, ...) @printf(ptr %error_msg)
  call void @exit(i32 %error_code)
  unreachable

"extract-cons:return":                            ; preds = %entry
  %"extract-cons:return1" = extractvalue { i32, ptr } %0, 1
  %"extract-cons:" = load { ptr, ptr }, ptr %"extract-cons:return1", align 8
  %"get car" = extractvalue { ptr, ptr } %"extract-cons:", 0
  %"load car" = load { i32, ptr }, ptr %"get car", align 8
  %get_type2 = extractvalue { i32, ptr } %"load car", 0
  %"not bool check" = icmp ne i32 %get_type2, 1
  %"get object context" = extractvalue { i32, ptr } %"load car", 1
  %"get bool value" = load i1, ptr %"get object context", align 1
  %"non bool or true" = or i1 %"not bool check", %"get bool value"
  %not = xor i1 %"non bool or true", true
  %"object value" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr (i1, ptr null, i32 1) to i32))
  store i1 %not, ptr %"object value", align 1
  %"insert value object" = insertvalue { i32, ptr } { i32 1, ptr null }, ptr %"object value", 1
  ret { i32, ptr } %"insert value object"
}

define { i32, ptr } @print({ i32, ptr } %0) {
entry:
  %get_type = extractvalue { i32, ptr } %0, 0
  %"extract-cons:cmp-type" = icmp eq i32 %get_type, 6
  br i1 %"extract-cons:cmp-type", label %"extract-cons:return", label %error

error:                                            ; preds = %entry
  %"error phi" = phi { ptr, i32 } [ { ptr @"error exit.32", i32 1 }, %entry ]
  %error_msg = extractvalue { ptr, i32 } %"error phi", 0
  %error_code = extractvalue { ptr, i32 } %"error phi", 1
  %print = call i32 (ptr, ...) @printf(ptr %error_msg)
  call void @exit(i32 %error_code)
  unreachable

"extract-cons:return":                            ; preds = %entry
  %"extract-cons:return1" = extractvalue { i32, ptr } %0, 1
  %"extract-cons:" = load { ptr, ptr }, ptr %"extract-cons:return1", align 8
  %"get car" = extractvalue { ptr, ptr } %"extract-cons:", 0
  %"load car" = load { i32, ptr }, ptr %"get car", align 8
  call void @print-obj({ i32, ptr } %"load car")
  ret { i32, ptr } zeroinitializer
}

define { i32, ptr } @-1({ i32, ptr } %0) {
entry:
  %get_type = extractvalue { i32, ptr } %0, 0
  %"extract-cons:cmp-type" = icmp eq i32 %get_type, 6
  br i1 %"extract-cons:cmp-type", label %"extract-cons:return", label %error

error:                                            ; preds = %"extract-cons:return", %entry
  %"error phi" = phi { ptr, i32 } [ { ptr @"error exit.33", i32 1 }, %entry ], [ { ptr @"error exit.34", i32 1 }, %"extract-cons:return" ]
  %error_msg = extractvalue { ptr, i32 } %"error phi", 0
  %error_code = extractvalue { ptr, i32 } %"error phi", 1
  %print = call i32 (ptr, ...) @printf(ptr %error_msg)
  call void @exit(i32 %error_code)
  unreachable

"extract-cons:return":                            ; preds = %entry
  %"extract-cons:return1" = extractvalue { i32, ptr } %0, 1
  %"extract-cons:" = load { ptr, ptr }, ptr %"extract-cons:return1", align 8
  %"get car" = extractvalue { ptr, ptr } %"extract-cons:", 0
  %"load car" = load { i32, ptr }, ptr %"get car", align 8
  %get_type2 = extractvalue { i32, ptr } %"load car", 0
  %"extract-number:cmp-type" = icmp eq i32 %get_type2, 2
  br i1 %"extract-number:cmp-type", label %"extract-number:return", label %error

"extract-number:return":                          ; preds = %"extract-cons:return"
  %"extract-number:return3" = extractvalue { i32, ptr } %"load car", 1
  %"extract-number:" = load double, ptr %"extract-number:return3", align 8
  %"sub 1" = fsub double %"extract-number:", 1.000000e+00
  %"object value" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr (double, ptr null, i32 1) to i32))
  store double %"sub 1", ptr %"object value", align 8
  %"insert value object" = insertvalue { i32, ptr } { i32 2, ptr null }, ptr %"object value", 1
  %"object value4" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr (double, ptr null, i32 1) to i32))
  store double %"sub 1", ptr %"object value4", align 8
  %"insert value object5" = insertvalue { i32, ptr } { i32 2, ptr null }, ptr %"object value4", 1
  ret { i32, ptr } %"insert value object5"
}

define { i32, ptr } @"+1"({ i32, ptr } %0) {
entry:
  %get_type = extractvalue { i32, ptr } %0, 0
  %"extract-cons:cmp-type" = icmp eq i32 %get_type, 6
  br i1 %"extract-cons:cmp-type", label %"extract-cons:return", label %error

error:                                            ; preds = %"extract-cons:return", %entry
  %"error phi" = phi { ptr, i32 } [ { ptr @"error exit.35", i32 1 }, %entry ], [ { ptr @"error exit.36", i32 1 }, %"extract-cons:return" ]
  %error_msg = extractvalue { ptr, i32 } %"error phi", 0
  %error_code = extractvalue { ptr, i32 } %"error phi", 1
  %print = call i32 (ptr, ...) @printf(ptr %error_msg)
  call void @exit(i32 %error_code)
  unreachable

"extract-cons:return":                            ; preds = %entry
  %"extract-cons:return1" = extractvalue { i32, ptr } %0, 1
  %"extract-cons:" = load { ptr, ptr }, ptr %"extract-cons:return1", align 8
  %"get car" = extractvalue { ptr, ptr } %"extract-cons:", 0
  %"load car" = load { i32, ptr }, ptr %"get car", align 8
  %get_type2 = extractvalue { i32, ptr } %"load car", 0
  %"extract-number:cmp-type" = icmp eq i32 %get_type2, 2
  br i1 %"extract-number:cmp-type", label %"extract-number:return", label %error

"extract-number:return":                          ; preds = %"extract-cons:return"
  %"extract-number:return3" = extractvalue { i32, ptr } %"load car", 1
  %"extract-number:" = load double, ptr %"extract-number:return3", align 8
  %"add 1" = fadd double %"extract-number:", 1.000000e+00
  %"object value" = tail call ptr @malloc(i32 ptrtoint (ptr getelementptr (double, ptr null, i32 1) to i32))
  store double %"add 1", ptr %"object value", align 8
  %"insert value object" = insertvalue { i32, ptr } { i32 2, ptr null }, ptr %"object value", 1
  ret { i32, ptr } %"insert value object"
}

define { i32, ptr } @car({ i32, ptr } %0) {
entry:
  %get_type = extractvalue { i32, ptr } %0, 0
  %"extract-cons:cmp-type" = icmp eq i32 %get_type, 6
  br i1 %"extract-cons:cmp-type", label %"extract-cons:return", label %error

error:                                            ; preds = %"extract-cons:return", %entry
  %"error phi" = phi { ptr, i32 } [ { ptr @"error exit.39", i32 1 }, %entry ], [ { ptr @"error exit.40", i32 1 }, %"extract-cons:return" ]
  %error_msg = extractvalue { ptr, i32 } %"error phi", 0
  %error_code = extractvalue { ptr, i32 } %"error phi", 1
  %print = call i32 (ptr, ...) @printf(ptr %error_msg)
  call void @exit(i32 %error_code)
  unreachable

"extract-cons:return":                            ; preds = %entry
  %"extract-cons:return1" = extractvalue { i32, ptr } %0, 1
  %"extract-cons:" = load { ptr, ptr }, ptr %"extract-cons:return1", align 8
  %"get car" = extractvalue { ptr, ptr } %"extract-cons:", 0
  %"load car" = load { i32, ptr }, ptr %"get car", align 8
  %get_type3 = extractvalue { i32, ptr } %"load car", 0
  %"extract-cons:cmp-type4" = icmp eq i32 %get_type3, 6
  br i1 %"extract-cons:cmp-type4", label %"extract-cons:return2", label %error

"extract-cons:return2":                           ; preds = %"extract-cons:return"
  %"extract-cons:return5" = extractvalue { i32, ptr } %"load car", 1
  %"extract-cons:6" = load { ptr, ptr }, ptr %"extract-cons:return5", align 8
  %"get car7" = extractvalue { ptr, ptr } %"extract-cons:6", 0
  %"load car8" = load { i32, ptr }, ptr %"get car7", align 8
  ret { i32, ptr } %"load car8"
}

define { i32, ptr } @cdr({ i32, ptr } %0) {
entry:
  %get_type = extractvalue { i32, ptr } %0, 0
  %"extract-cons:cmp-type" = icmp eq i32 %get_type, 6
  br i1 %"extract-cons:cmp-type", label %"extract-cons:return", label %error

error:                                            ; preds = %"extract-cons:return", %entry
  %"error phi" = phi { ptr, i32 } [ { ptr @"error exit.41", i32 1 }, %entry ], [ { ptr @"error exit.42", i32 1 }, %"extract-cons:return" ]
  %error_msg = extractvalue { ptr, i32 } %"error phi", 0
  %error_code = extractvalue { ptr, i32 } %"error phi", 1
  %print = call i32 (ptr, ...) @printf(ptr %error_msg)
  call void @exit(i32 %error_code)
  unreachable

"extract-cons:return":                            ; preds = %entry
  %"extract-cons:return1" = extractvalue { i32, ptr } %0, 1
  %"extract-cons:" = load { ptr, ptr }, ptr %"extract-cons:return1", align 8
  %"get car" = extractvalue { ptr, ptr } %"extract-cons:", 0
  %"load car" = load { i32, ptr }, ptr %"get car", align 8
  %get_type3 = extractvalue { i32, ptr } %"load car", 0
  %"extract-cons:cmp-type4" = icmp eq i32 %get_type3, 6
  br i1 %"extract-cons:cmp-type4", label %"extract-cons:return2", label %error

"extract-cons:return2":                           ; preds = %"extract-cons:return"
  %"extract-cons:return5" = extractvalue { i32, ptr } %"load car", 1
  %"extract-cons:6" = load { ptr, ptr }, ptr %"extract-cons:return5", align 8
  %"get cdr" = extractvalue { ptr, ptr } %"extract-cons:6", 1
  %"load cdr" = load { i32, ptr }, ptr %"get cdr", align 8
  ret { i32, ptr } %"load cdr"
}

define { i32, ptr } @caar({ i32, ptr } %0) {
entry:
  %get_type = extractvalue { i32, ptr } %0, 0
  %"extract-cons:cmp-type" = icmp eq i32 %get_type, 6
  br i1 %"extract-cons:cmp-type", label %"extract-cons:return", label %error

error:                                            ; preds = %"extract-cons:return2", %"extract-cons:return", %entry
  %"error phi" = phi { ptr, i32 } [ { ptr @"error exit.43", i32 1 }, %entry ], [ { ptr @"error exit.44", i32 1 }, %"extract-cons:return" ], [ { ptr @"error exit.45", i32 1 }, %"extract-cons:return2" ]
  %error_msg = extractvalue { ptr, i32 } %"error phi", 0
  %error_code = extractvalue { ptr, i32 } %"error phi", 1
  %print = call i32 (ptr, ...) @printf(ptr %error_msg)
  call void @exit(i32 %error_code)
  unreachable

"extract-cons:return":                            ; preds = %entry
  %"extract-cons:return1" = extractvalue { i32, ptr } %0, 1
  %"extract-cons:" = load { ptr, ptr }, ptr %"extract-cons:return1", align 8
  %"get car" = extractvalue { ptr, ptr } %"extract-cons:", 0
  %"load car" = load { i32, ptr }, ptr %"get car", align 8
  %get_type3 = extractvalue { i32, ptr } %"load car", 0
  %"extract-cons:cmp-type4" = icmp eq i32 %get_type3, 6
  br i1 %"extract-cons:cmp-type4", label %"extract-cons:return2", label %error

"extract-cons:return2":                           ; preds = %"extract-cons:return"
  %"extract-cons:return5" = extractvalue { i32, ptr } %"load car", 1
  %"extract-cons:6" = load { ptr, ptr }, ptr %"extract-cons:return5", align 8
  %"get car7" = extractvalue { ptr, ptr } %"extract-cons:6", 0
  %"load car8" = load { i32, ptr }, ptr %"get car7", align 8
  %get_type10 = extractvalue { i32, ptr } %"load car8", 0
  %"extract-cons:cmp-type11" = icmp eq i32 %get_type10, 6
  br i1 %"extract-cons:cmp-type11", label %"extract-cons:return9", label %error

"extract-cons:return9":                           ; preds = %"extract-cons:return2"
  %"extract-cons:return12" = extractvalue { i32, ptr } %"load car8", 1
  %"extract-cons:13" = load { ptr, ptr }, ptr %"extract-cons:return12", align 8
  %"get car14" = extractvalue { ptr, ptr } %"extract-cons:13", 0
  %"load car15" = load { i32, ptr }, ptr %"get car14", align 8
  ret { i32, ptr } %"load car15"
}

define { i32, ptr } @cadr({ i32, ptr } %0) {
entry:
  %get_type = extractvalue { i32, ptr } %0, 0
  %"extract-cons:cmp-type" = icmp eq i32 %get_type, 6
  br i1 %"extract-cons:cmp-type", label %"extract-cons:return", label %error

error:                                            ; preds = %"extract-cons:return2", %"extract-cons:return", %entry
  %"error phi" = phi { ptr, i32 } [ { ptr @"error exit.46", i32 1 }, %entry ], [ { ptr @"error exit.47", i32 1 }, %"extract-cons:return" ], [ { ptr @"error exit.48", i32 1 }, %"extract-cons:return2" ]
  %error_msg = extractvalue { ptr, i32 } %"error phi", 0
  %error_code = extractvalue { ptr, i32 } %"error phi", 1
  %print = call i32 (ptr, ...) @printf(ptr %error_msg)
  call void @exit(i32 %error_code)
  unreachable

"extract-cons:return":                            ; preds = %entry
  %"extract-cons:return1" = extractvalue { i32, ptr } %0, 1
  %"extract-cons:" = load { ptr, ptr }, ptr %"extract-cons:return1", align 8
  %"get car" = extractvalue { ptr, ptr } %"extract-cons:", 0
  %"load car" = load { i32, ptr }, ptr %"get car", align 8
  %get_type3 = extractvalue { i32, ptr } %"load car", 0
  %"extract-cons:cmp-type4" = icmp eq i32 %get_type3, 6
  br i1 %"extract-cons:cmp-type4", label %"extract-cons:return2", label %error

"extract-cons:return2":                           ; preds = %"extract-cons:return"
  %"extract-cons:return5" = extractvalue { i32, ptr } %"load car", 1
  %"extract-cons:6" = load { ptr, ptr }, ptr %"extract-cons:return5", align 8
  %"get cdr" = extractvalue { ptr, ptr } %"extract-cons:6", 1
  %"load cdr" = load { i32, ptr }, ptr %"get cdr", align 8
  %get_type8 = extractvalue { i32, ptr } %"load cdr", 0
  %"extract-cons:cmp-type9" = icmp eq i32 %get_type8, 6
  br i1 %"extract-cons:cmp-type9", label %"extract-cons:return7", label %error

"extract-cons:return7":                           ; preds = %"extract-cons:return2"
  %"extract-cons:return10" = extractvalue { i32, ptr } %"load cdr", 1
  %"extract-cons:11" = load { ptr, ptr }, ptr %"extract-cons:return10", align 8
  %"get car12" = extractvalue { ptr, ptr } %"extract-cons:11", 0
  %"load car13" = load { i32, ptr }, ptr %"get car12", align 8
  ret { i32, ptr } %"load car13"
}

define { i32, ptr } @cdar({ i32, ptr } %0) {
entry:
  %get_type = extractvalue { i32, ptr } %0, 0
  %"extract-cons:cmp-type" = icmp eq i32 %get_type, 6
  br i1 %"extract-cons:cmp-type", label %"extract-cons:return", label %error

error:                                            ; preds = %"extract-cons:return2", %"extract-cons:return", %entry
  %"error phi" = phi { ptr, i32 } [ { ptr @"error exit.49", i32 1 }, %entry ], [ { ptr @"error exit.50", i32 1 }, %"extract-cons:return" ], [ { ptr @"error exit.51", i32 1 }, %"extract-cons:return2" ]
  %error_msg = extractvalue { ptr, i32 } %"error phi", 0
  %error_code = extractvalue { ptr, i32 } %"error phi", 1
  %print = call i32 (ptr, ...) @printf(ptr %error_msg)
  call void @exit(i32 %error_code)
  unreachable

"extract-cons:return":                            ; preds = %entry
  %"extract-cons:return1" = extractvalue { i32, ptr } %0, 1
  %"extract-cons:" = load { ptr, ptr }, ptr %"extract-cons:return1", align 8
  %"get car" = extractvalue { ptr, ptr } %"extract-cons:", 0
  %"load car" = load { i32, ptr }, ptr %"get car", align 8
  %get_type3 = extractvalue { i32, ptr } %"load car", 0
  %"extract-cons:cmp-type4" = icmp eq i32 %get_type3, 6
  br i1 %"extract-cons:cmp-type4", label %"extract-cons:return2", label %error

"extract-cons:return2":                           ; preds = %"extract-cons:return"
  %"extract-cons:return5" = extractvalue { i32, ptr } %"load car", 1
  %"extract-cons:6" = load { ptr, ptr }, ptr %"extract-cons:return5", align 8
  %"get car7" = extractvalue { ptr, ptr } %"extract-cons:6", 0
  %"load car8" = load { i32, ptr }, ptr %"get car7", align 8
  %get_type10 = extractvalue { i32, ptr } %"load car8", 0
  %"extract-cons:cmp-type11" = icmp eq i32 %get_type10, 6
  br i1 %"extract-cons:cmp-type11", label %"extract-cons:return9", label %error

"extract-cons:return9":                           ; preds = %"extract-cons:return2"
  %"extract-cons:return12" = extractvalue { i32, ptr } %"load car8", 1
  %"extract-cons:13" = load { ptr, ptr }, ptr %"extract-cons:return12", align 8
  %"get cdr" = extractvalue { ptr, ptr } %"extract-cons:13", 1
  %"load cdr" = load { i32, ptr }, ptr %"get cdr", align 8
  ret { i32, ptr } %"load cdr"
}

define { i32, ptr } @cddr({ i32, ptr } %0) {
entry:
  %get_type = extractvalue { i32, ptr } %0, 0
  %"extract-cons:cmp-type" = icmp eq i32 %get_type, 6
  br i1 %"extract-cons:cmp-type", label %"extract-cons:return", label %error

error:                                            ; preds = %"extract-cons:return2", %"extract-cons:return", %entry
  %"error phi" = phi { ptr, i32 } [ { ptr @"error exit.52", i32 1 }, %entry ], [ { ptr @"error exit.53", i32 1 }, %"extract-cons:return" ], [ { ptr @"error exit.54", i32 1 }, %"extract-cons:return2" ]
  %error_msg = extractvalue { ptr, i32 } %"error phi", 0
  %error_code = extractvalue { ptr, i32 } %"error phi", 1
  %print = call i32 (ptr, ...) @printf(ptr %error_msg)
  call void @exit(i32 %error_code)
  unreachable

"extract-cons:return":                            ; preds = %entry
  %"extract-cons:return1" = extractvalue { i32, ptr } %0, 1
  %"extract-cons:" = load { ptr, ptr }, ptr %"extract-cons:return1", align 8
  %"get car" = extractvalue { ptr, ptr } %"extract-cons:", 0
  %"load car" = load { i32, ptr }, ptr %"get car", align 8
  %get_type3 = extractvalue { i32, ptr } %"load car", 0
  %"extract-cons:cmp-type4" = icmp eq i32 %get_type3, 6
  br i1 %"extract-cons:cmp-type4", label %"extract-cons:return2", label %error

"extract-cons:return2":                           ; preds = %"extract-cons:return"
  %"extract-cons:return5" = extractvalue { i32, ptr } %"load car", 1
  %"extract-cons:6" = load { ptr, ptr }, ptr %"extract-cons:return5", align 8
  %"get cdr" = extractvalue { ptr, ptr } %"extract-cons:6", 1
  %"load cdr" = load { i32, ptr }, ptr %"get cdr", align 8
  %get_type8 = extractvalue { i32, ptr } %"load cdr", 0
  %"extract-cons:cmp-type9" = icmp eq i32 %get_type8, 6
  br i1 %"extract-cons:cmp-type9", label %"extract-cons:return7", label %error

"extract-cons:return7":                           ; preds = %"extract-cons:return2"
  %"extract-cons:return10" = extractvalue { i32, ptr } %"load cdr", 1
  %"extract-cons:11" = load { ptr, ptr }, ptr %"extract-cons:return10", align 8
  %"get cdr12" = extractvalue { ptr, ptr } %"extract-cons:11", 1
  %"load cdr13" = load { i32, ptr }, ptr %"get cdr12", align 8
  ret { i32, ptr } %"load cdr13"
}

define { i32, ptr } @caaar({ i32, ptr } %0) {
entry:
  %get_type = extractvalue { i32, ptr } %0, 0
  %"extract-cons:cmp-type" = icmp eq i32 %get_type, 6
  br i1 %"extract-cons:cmp-type", label %"extract-cons:return", label %error

error:                                            ; preds = %"extract-cons:return9", %"extract-cons:return2", %"extract-cons:return", %entry
  %"error phi" = phi { ptr, i32 } [ { ptr @"error exit.55", i32 1 }, %entry ], [ { ptr @"error exit.56", i32 1 }, %"extract-cons:return" ], [ { ptr @"error exit.57", i32 1 }, %"extract-cons:return2" ], [ { ptr @"error exit.58", i32 1 }, %"extract-cons:return9" ]
  %error_msg = extractvalue { ptr, i32 } %"error phi", 0
  %error_code = extractvalue { ptr, i32 } %"error phi", 1
  %print = call i32 (ptr, ...) @printf(ptr %error_msg)
  call void @exit(i32 %error_code)
  unreachable

"extract-cons:return":                            ; preds = %entry
  %"extract-cons:return1" = extractvalue { i32, ptr } %0, 1
  %"extract-cons:" = load { ptr, ptr }, ptr %"extract-cons:return1", align 8
  %"get car" = extractvalue { ptr, ptr } %"extract-cons:", 0
  %"load car" = load { i32, ptr }, ptr %"get car", align 8
  %get_type3 = extractvalue { i32, ptr } %"load car", 0
  %"extract-cons:cmp-type4" = icmp eq i32 %get_type3, 6
  br i1 %"extract-cons:cmp-type4", label %"extract-cons:return2", label %error

"extract-cons:return2":                           ; preds = %"extract-cons:return"
  %"extract-cons:return5" = extractvalue { i32, ptr } %"load car", 1
  %"extract-cons:6" = load { ptr, ptr }, ptr %"extract-cons:return5", align 8
  %"get car7" = extractvalue { ptr, ptr } %"extract-cons:6", 0
  %"load car8" = load { i32, ptr }, ptr %"get car7", align 8
  %get_type10 = extractvalue { i32, ptr } %"load car8", 0
  %"extract-cons:cmp-type11" = icmp eq i32 %get_type10, 6
  br i1 %"extract-cons:cmp-type11", label %"extract-cons:return9", label %error

"extract-cons:return9":                           ; preds = %"extract-cons:return2"
  %"extract-cons:return12" = extractvalue { i32, ptr } %"load car8", 1
  %"extract-cons:13" = load { ptr, ptr }, ptr %"extract-cons:return12", align 8
  %"get car14" = extractvalue { ptr, ptr } %"extract-cons:13", 0
  %"load car15" = load { i32, ptr }, ptr %"get car14", align 8
  %get_type17 = extractvalue { i32, ptr } %"load car15", 0
  %"extract-cons:cmp-type18" = icmp eq i32 %get_type17, 6
  br i1 %"extract-cons:cmp-type18", label %"extract-cons:return16", label %error

"extract-cons:return16":                          ; preds = %"extract-cons:return9"
  %"extract-cons:return19" = extractvalue { i32, ptr } %"load car15", 1
  %"extract-cons:20" = load { ptr, ptr }, ptr %"extract-cons:return19", align 8
  %"get car21" = extractvalue { ptr, ptr } %"extract-cons:20", 0
  %"load car22" = load { i32, ptr }, ptr %"get car21", align 8
  ret { i32, ptr } %"load car22"
}

define { i32, ptr } @caadr({ i32, ptr } %0) {
entry:
  %get_type = extractvalue { i32, ptr } %0, 0
  %"extract-cons:cmp-type" = icmp eq i32 %get_type, 6
  br i1 %"extract-cons:cmp-type", label %"extract-cons:return", label %error

error:                                            ; preds = %"extract-cons:return7", %"extract-cons:return2", %"extract-cons:return", %entry
  %"error phi" = phi { ptr, i32 } [ { ptr @"error exit.59", i32 1 }, %entry ], [ { ptr @"error exit.60", i32 1 }, %"extract-cons:return" ], [ { ptr @"error exit.61", i32 1 }, %"extract-cons:return2" ], [ { ptr @"error exit.62", i32 1 }, %"extract-cons:return7" ]
  %error_msg = extractvalue { ptr, i32 } %"error phi", 0
  %error_code = extractvalue { ptr, i32 } %"error phi", 1
  %print = call i32 (ptr, ...) @printf(ptr %error_msg)
  call void @exit(i32 %error_code)
  unreachable

"extract-cons:return":                            ; preds = %entry
  %"extract-cons:return1" = extractvalue { i32, ptr } %0, 1
  %"extract-cons:" = load { ptr, ptr }, ptr %"extract-cons:return1", align 8
  %"get car" = extractvalue { ptr, ptr } %"extract-cons:", 0
  %"load car" = load { i32, ptr }, ptr %"get car", align 8
  %get_type3 = extractvalue { i32, ptr } %"load car", 0
  %"extract-cons:cmp-type4" = icmp eq i32 %get_type3, 6
  br i1 %"extract-cons:cmp-type4", label %"extract-cons:return2", label %error

"extract-cons:return2":                           ; preds = %"extract-cons:return"
  %"extract-cons:return5" = extractvalue { i32, ptr } %"load car", 1
  %"extract-cons:6" = load { ptr, ptr }, ptr %"extract-cons:return5", align 8
  %"get cdr" = extractvalue { ptr, ptr } %"extract-cons:6", 1
  %"load cdr" = load { i32, ptr }, ptr %"get cdr", align 8
  %get_type8 = extractvalue { i32, ptr } %"load cdr", 0
  %"extract-cons:cmp-type9" = icmp eq i32 %get_type8, 6
  br i1 %"extract-cons:cmp-type9", label %"extract-cons:return7", label %error

"extract-cons:return7":                           ; preds = %"extract-cons:return2"
  %"extract-cons:return10" = extractvalue { i32, ptr } %"load cdr", 1
  %"extract-cons:11" = load { ptr, ptr }, ptr %"extract-cons:return10", align 8
  %"get car12" = extractvalue { ptr, ptr } %"extract-cons:11", 0
  %"load car13" = load { i32, ptr }, ptr %"get car12", align 8
  %get_type15 = extractvalue { i32, ptr } %"load car13", 0
  %"extract-cons:cmp-type16" = icmp eq i32 %get_type15, 6
  br i1 %"extract-cons:cmp-type16", label %"extract-cons:return14", label %error

"extract-cons:return14":                          ; preds = %"extract-cons:return7"
  %"extract-cons:return17" = extractvalue { i32, ptr } %"load car13", 1
  %"extract-cons:18" = load { ptr, ptr }, ptr %"extract-cons:return17", align 8
  %"get car19" = extractvalue { ptr, ptr } %"extract-cons:18", 0
  %"load car20" = load { i32, ptr }, ptr %"get car19", align 8
  ret { i32, ptr } %"load car20"
}

define { i32, ptr } @cadar({ i32, ptr } %0) {
entry:
  %get_type = extractvalue { i32, ptr } %0, 0
  %"extract-cons:cmp-type" = icmp eq i32 %get_type, 6
  br i1 %"extract-cons:cmp-type", label %"extract-cons:return", label %error

error:                                            ; preds = %"extract-cons:return9", %"extract-cons:return2", %"extract-cons:return", %entry
  %"error phi" = phi { ptr, i32 } [ { ptr @"error exit.63", i32 1 }, %entry ], [ { ptr @"error exit.64", i32 1 }, %"extract-cons:return" ], [ { ptr @"error exit.65", i32 1 }, %"extract-cons:return2" ], [ { ptr @"error exit.66", i32 1 }, %"extract-cons:return9" ]
  %error_msg = extractvalue { ptr, i32 } %"error phi", 0
  %error_code = extractvalue { ptr, i32 } %"error phi", 1
  %print = call i32 (ptr, ...) @printf(ptr %error_msg)
  call void @exit(i32 %error_code)
  unreachable

"extract-cons:return":                            ; preds = %entry
  %"extract-cons:return1" = extractvalue { i32, ptr } %0, 1
  %"extract-cons:" = load { ptr, ptr }, ptr %"extract-cons:return1", align 8
  %"get car" = extractvalue { ptr, ptr } %"extract-cons:", 0
  %"load car" = load { i32, ptr }, ptr %"get car", align 8
  %get_type3 = extractvalue { i32, ptr } %"load car", 0
  %"extract-cons:cmp-type4" = icmp eq i32 %get_type3, 6
  br i1 %"extract-cons:cmp-type4", label %"extract-cons:return2", label %error

"extract-cons:return2":                           ; preds = %"extract-cons:return"
  %"extract-cons:return5" = extractvalue { i32, ptr } %"load car", 1
  %"extract-cons:6" = load { ptr, ptr }, ptr %"extract-cons:return5", align 8
  %"get car7" = extractvalue { ptr, ptr } %"extract-cons:6", 0
  %"load car8" = load { i32, ptr }, ptr %"get car7", align 8
  %get_type10 = extractvalue { i32, ptr } %"load car8", 0
  %"extract-cons:cmp-type11" = icmp eq i32 %get_type10, 6
  br i1 %"extract-cons:cmp-type11", label %"extract-cons:return9", label %error

"extract-cons:return9":                           ; preds = %"extract-cons:return2"
  %"extract-cons:return12" = extractvalue { i32, ptr } %"load car8", 1
  %"extract-cons:13" = load { ptr, ptr }, ptr %"extract-cons:return12", align 8
  %"get cdr" = extractvalue { ptr, ptr } %"extract-cons:13", 1
  %"load cdr" = load { i32, ptr }, ptr %"get cdr", align 8
  %get_type15 = extractvalue { i32, ptr } %"load cdr", 0
  %"extract-cons:cmp-type16" = icmp eq i32 %get_type15, 6
  br i1 %"extract-cons:cmp-type16", label %"extract-cons:return14", label %error

"extract-cons:return14":                          ; preds = %"extract-cons:return9"
  %"extract-cons:return17" = extractvalue { i32, ptr } %"load cdr", 1
  %"extract-cons:18" = load { ptr, ptr }, ptr %"extract-cons:return17", align 8
  %"get car19" = extractvalue { ptr, ptr } %"extract-cons:18", 0
  %"load car20" = load { i32, ptr }, ptr %"get car19", align 8
  ret { i32, ptr } %"load car20"
}

define { i32, ptr } @caddr({ i32, ptr } %0) {
entry:
  %get_type = extractvalue { i32, ptr } %0, 0
  %"extract-cons:cmp-type" = icmp eq i32 %get_type, 6
  br i1 %"extract-cons:cmp-type", label %"extract-cons:return", label %error

error:                                            ; preds = %"extract-cons:return7", %"extract-cons:return2", %"extract-cons:return", %entry
  %"error phi" = phi { ptr, i32 } [ { ptr @"error exit.67", i32 1 }, %entry ], [ { ptr @"error exit.68", i32 1 }, %"extract-cons:return" ], [ { ptr @"error exit.69", i32 1 }, %"extract-cons:return2" ], [ { ptr @"error exit.70", i32 1 }, %"extract-cons:return7" ]
  %error_msg = extractvalue { ptr, i32 } %"error phi", 0
  %error_code = extractvalue { ptr, i32 } %"error phi", 1
  %print = call i32 (ptr, ...) @printf(ptr %error_msg)
  call void @exit(i32 %error_code)
  unreachable

"extract-cons:return":                            ; preds = %entry
  %"extract-cons:return1" = extractvalue { i32, ptr } %0, 1
  %"extract-cons:" = load { ptr, ptr }, ptr %"extract-cons:return1", align 8
  %"get car" = extractvalue { ptr, ptr } %"extract-cons:", 0
  %"load car" = load { i32, ptr }, ptr %"get car", align 8
  %get_type3 = extractvalue { i32, ptr } %"load car", 0
  %"extract-cons:cmp-type4" = icmp eq i32 %get_type3, 6
  br i1 %"extract-cons:cmp-type4", label %"extract-cons:return2", label %error

"extract-cons:return2":                           ; preds = %"extract-cons:return"
  %"extract-cons:return5" = extractvalue { i32, ptr } %"load car", 1
  %"extract-cons:6" = load { ptr, ptr }, ptr %"extract-cons:return5", align 8
  %"get cdr" = extractvalue { ptr, ptr } %"extract-cons:6", 1
  %"load cdr" = load { i32, ptr }, ptr %"get cdr", align 8
  %get_type8 = extractvalue { i32, ptr } %"load cdr", 0
  %"extract-cons:cmp-type9" = icmp eq i32 %get_type8, 6
  br i1 %"extract-cons:cmp-type9", label %"extract-cons:return7", label %error

"extract-cons:return7":                           ; preds = %"extract-cons:return2"
  %"extract-cons:return10" = extractvalue { i32, ptr } %"load cdr", 1
  %"extract-cons:11" = load { ptr, ptr }, ptr %"extract-cons:return10", align 8
  %"get cdr12" = extractvalue { ptr, ptr } %"extract-cons:11", 1
  %"load cdr13" = load { i32, ptr }, ptr %"get cdr12", align 8
  %get_type15 = extractvalue { i32, ptr } %"load cdr13", 0
  %"extract-cons:cmp-type16" = icmp eq i32 %get_type15, 6
  br i1 %"extract-cons:cmp-type16", label %"extract-cons:return14", label %error

"extract-cons:return14":                          ; preds = %"extract-cons:return7"
  %"extract-cons:return17" = extractvalue { i32, ptr } %"load cdr13", 1
  %"extract-cons:18" = load { ptr, ptr }, ptr %"extract-cons:return17", align 8
  %"get car19" = extractvalue { ptr, ptr } %"extract-cons:18", 0
  %"load car20" = load { i32, ptr }, ptr %"get car19", align 8
  ret { i32, ptr } %"load car20"
}

define { i32, ptr } @cdaar({ i32, ptr } %0) {
entry:
  %get_type = extractvalue { i32, ptr } %0, 0
  %"extract-cons:cmp-type" = icmp eq i32 %get_type, 6
  br i1 %"extract-cons:cmp-type", label %"extract-cons:return", label %error

error:                                            ; preds = %"extract-cons:return9", %"extract-cons:return2", %"extract-cons:return", %entry
  %"error phi" = phi { ptr, i32 } [ { ptr @"error exit.71", i32 1 }, %entry ], [ { ptr @"error exit.72", i32 1 }, %"extract-cons:return" ], [ { ptr @"error exit.73", i32 1 }, %"extract-cons:return2" ], [ { ptr @"error exit.74", i32 1 }, %"extract-cons:return9" ]
  %error_msg = extractvalue { ptr, i32 } %"error phi", 0
  %error_code = extractvalue { ptr, i32 } %"error phi", 1
  %print = call i32 (ptr, ...) @printf(ptr %error_msg)
  call void @exit(i32 %error_code)
  unreachable

"extract-cons:return":                            ; preds = %entry
  %"extract-cons:return1" = extractvalue { i32, ptr } %0, 1
  %"extract-cons:" = load { ptr, ptr }, ptr %"extract-cons:return1", align 8
  %"get car" = extractvalue { ptr, ptr } %"extract-cons:", 0
  %"load car" = load { i32, ptr }, ptr %"get car", align 8
  %get_type3 = extractvalue { i32, ptr } %"load car", 0
  %"extract-cons:cmp-type4" = icmp eq i32 %get_type3, 6
  br i1 %"extract-cons:cmp-type4", label %"extract-cons:return2", label %error

"extract-cons:return2":                           ; preds = %"extract-cons:return"
  %"extract-cons:return5" = extractvalue { i32, ptr } %"load car", 1
  %"extract-cons:6" = load { ptr, ptr }, ptr %"extract-cons:return5", align 8
  %"get car7" = extractvalue { ptr, ptr } %"extract-cons:6", 0
  %"load car8" = load { i32, ptr }, ptr %"get car7", align 8
  %get_type10 = extractvalue { i32, ptr } %"load car8", 0
  %"extract-cons:cmp-type11" = icmp eq i32 %get_type10, 6
  br i1 %"extract-cons:cmp-type11", label %"extract-cons:return9", label %error

"extract-cons:return9":                           ; preds = %"extract-cons:return2"
  %"extract-cons:return12" = extractvalue { i32, ptr } %"load car8", 1
  %"extract-cons:13" = load { ptr, ptr }, ptr %"extract-cons:return12", align 8
  %"get car14" = extractvalue { ptr, ptr } %"extract-cons:13", 0
  %"load car15" = load { i32, ptr }, ptr %"get car14", align 8
  %get_type17 = extractvalue { i32, ptr } %"load car15", 0
  %"extract-cons:cmp-type18" = icmp eq i32 %get_type17, 6
  br i1 %"extract-cons:cmp-type18", label %"extract-cons:return16", label %error

"extract-cons:return16":                          ; preds = %"extract-cons:return9"
  %"extract-cons:return19" = extractvalue { i32, ptr } %"load car15", 1
  %"extract-cons:20" = load { ptr, ptr }, ptr %"extract-cons:return19", align 8
  %"get cdr" = extractvalue { ptr, ptr } %"extract-cons:20", 1
  %"load cdr" = load { i32, ptr }, ptr %"get cdr", align 8
  ret { i32, ptr } %"load cdr"
}

define { i32, ptr } @cdadr({ i32, ptr } %0) {
entry:
  %get_type = extractvalue { i32, ptr } %0, 0
  %"extract-cons:cmp-type" = icmp eq i32 %get_type, 6
  br i1 %"extract-cons:cmp-type", label %"extract-cons:return", label %error

error:                                            ; preds = %"extract-cons:return7", %"extract-cons:return2", %"extract-cons:return", %entry
  %"error phi" = phi { ptr, i32 } [ { ptr @"error exit.75", i32 1 }, %entry ], [ { ptr @"error exit.76", i32 1 }, %"extract-cons:return" ], [ { ptr @"error exit.77", i32 1 }, %"extract-cons:return2" ], [ { ptr @"error exit.78", i32 1 }, %"extract-cons:return7" ]
  %error_msg = extractvalue { ptr, i32 } %"error phi", 0
  %error_code = extractvalue { ptr, i32 } %"error phi", 1
  %print = call i32 (ptr, ...) @printf(ptr %error_msg)
  call void @exit(i32 %error_code)
  unreachable

"extract-cons:return":                            ; preds = %entry
  %"extract-cons:return1" = extractvalue { i32, ptr } %0, 1
  %"extract-cons:" = load { ptr, ptr }, ptr %"extract-cons:return1", align 8
  %"get car" = extractvalue { ptr, ptr } %"extract-cons:", 0
  %"load car" = load { i32, ptr }, ptr %"get car", align 8
  %get_type3 = extractvalue { i32, ptr } %"load car", 0
  %"extract-cons:cmp-type4" = icmp eq i32 %get_type3, 6
  br i1 %"extract-cons:cmp-type4", label %"extract-cons:return2", label %error

"extract-cons:return2":                           ; preds = %"extract-cons:return"
  %"extract-cons:return5" = extractvalue { i32, ptr } %"load car", 1
  %"extract-cons:6" = load { ptr, ptr }, ptr %"extract-cons:return5", align 8
  %"get cdr" = extractvalue { ptr, ptr } %"extract-cons:6", 1
  %"load cdr" = load { i32, ptr }, ptr %"get cdr", align 8
  %get_type8 = extractvalue { i32, ptr } %"load cdr", 0
  %"extract-cons:cmp-type9" = icmp eq i32 %get_type8, 6
  br i1 %"extract-cons:cmp-type9", label %"extract-cons:return7", label %error

"extract-cons:return7":                           ; preds = %"extract-cons:return2"
  %"extract-cons:return10" = extractvalue { i32, ptr } %"load cdr", 1
  %"extract-cons:11" = load { ptr, ptr }, ptr %"extract-cons:return10", align 8
  %"get car12" = extractvalue { ptr, ptr } %"extract-cons:11", 0
  %"load car13" = load { i32, ptr }, ptr %"get car12", align 8
  %get_type15 = extractvalue { i32, ptr } %"load car13", 0
  %"extract-cons:cmp-type16" = icmp eq i32 %get_type15, 6
  br i1 %"extract-cons:cmp-type16", label %"extract-cons:return14", label %error

"extract-cons:return14":                          ; preds = %"extract-cons:return7"
  %"extract-cons:return17" = extractvalue { i32, ptr } %"load car13", 1
  %"extract-cons:18" = load { ptr, ptr }, ptr %"extract-cons:return17", align 8
  %"get cdr19" = extractvalue { ptr, ptr } %"extract-cons:18", 1
  %"load cdr20" = load { i32, ptr }, ptr %"get cdr19", align 8
  ret { i32, ptr } %"load cdr20"
}

define { i32, ptr } @cddar({ i32, ptr } %0) {
entry:
  %get_type = extractvalue { i32, ptr } %0, 0
  %"extract-cons:cmp-type" = icmp eq i32 %get_type, 6
  br i1 %"extract-cons:cmp-type", label %"extract-cons:return", label %error

error:                                            ; preds = %"extract-cons:return9", %"extract-cons:return2", %"extract-cons:return", %entry
  %"error phi" = phi { ptr, i32 } [ { ptr @"error exit.79", i32 1 }, %entry ], [ { ptr @"error exit.80", i32 1 }, %"extract-cons:return" ], [ { ptr @"error exit.81", i32 1 }, %"extract-cons:return2" ], [ { ptr @"error exit.82", i32 1 }, %"extract-cons:return9" ]
  %error_msg = extractvalue { ptr, i32 } %"error phi", 0
  %error_code = extractvalue { ptr, i32 } %"error phi", 1
  %print = call i32 (ptr, ...) @printf(ptr %error_msg)
  call void @exit(i32 %error_code)
  unreachable

"extract-cons:return":                            ; preds = %entry
  %"extract-cons:return1" = extractvalue { i32, ptr } %0, 1
  %"extract-cons:" = load { ptr, ptr }, ptr %"extract-cons:return1", align 8
  %"get car" = extractvalue { ptr, ptr } %"extract-cons:", 0
  %"load car" = load { i32, ptr }, ptr %"get car", align 8
  %get_type3 = extractvalue { i32, ptr } %"load car", 0
  %"extract-cons:cmp-type4" = icmp eq i32 %get_type3, 6
  br i1 %"extract-cons:cmp-type4", label %"extract-cons:return2", label %error

"extract-cons:return2":                           ; preds = %"extract-cons:return"
  %"extract-cons:return5" = extractvalue { i32, ptr } %"load car", 1
  %"extract-cons:6" = load { ptr, ptr }, ptr %"extract-cons:return5", align 8
  %"get car7" = extractvalue { ptr, ptr } %"extract-cons:6", 0
  %"load car8" = load { i32, ptr }, ptr %"get car7", align 8
  %get_type10 = extractvalue { i32, ptr } %"load car8", 0
  %"extract-cons:cmp-type11" = icmp eq i32 %get_type10, 6
  br i1 %"extract-cons:cmp-type11", label %"extract-cons:return9", label %error

"extract-cons:return9":                           ; preds = %"extract-cons:return2"
  %"extract-cons:return12" = extractvalue { i32, ptr } %"load car8", 1
  %"extract-cons:13" = load { ptr, ptr }, ptr %"extract-cons:return12", align 8
  %"get cdr" = extractvalue { ptr, ptr } %"extract-cons:13", 1
  %"load cdr" = load { i32, ptr }, ptr %"get cdr", align 8
  %get_type15 = extractvalue { i32, ptr } %"load cdr", 0
  %"extract-cons:cmp-type16" = icmp eq i32 %get_type15, 6
  br i1 %"extract-cons:cmp-type16", label %"extract-cons:return14", label %error

"extract-cons:return14":                          ; preds = %"extract-cons:return9"
  %"extract-cons:return17" = extractvalue { i32, ptr } %"load cdr", 1
  %"extract-cons:18" = load { ptr, ptr }, ptr %"extract-cons:return17", align 8
  %"get cdr19" = extractvalue { ptr, ptr } %"extract-cons:18", 1
  %"load cdr20" = load { i32, ptr }, ptr %"get cdr19", align 8
  ret { i32, ptr } %"load cdr20"
}

define { i32, ptr } @cdddr({ i32, ptr } %0) {
entry:
  %get_type = extractvalue { i32, ptr } %0, 0
  %"extract-cons:cmp-type" = icmp eq i32 %get_type, 6
  br i1 %"extract-cons:cmp-type", label %"extract-cons:return", label %error

error:                                            ; preds = %"extract-cons:return7", %"extract-cons:return2", %"extract-cons:return", %entry
  %"error phi" = phi { ptr, i32 } [ { ptr @"error exit.83", i32 1 }, %entry ], [ { ptr @"error exit.84", i32 1 }, %"extract-cons:return" ], [ { ptr @"error exit.85", i32 1 }, %"extract-cons:return2" ], [ { ptr @"error exit.86", i32 1 }, %"extract-cons:return7" ]
  %error_msg = extractvalue { ptr, i32 } %"error phi", 0
  %error_code = extractvalue { ptr, i32 } %"error phi", 1
  %print = call i32 (ptr, ...) @printf(ptr %error_msg)
  call void @exit(i32 %error_code)
  unreachable

"extract-cons:return":                            ; preds = %entry
  %"extract-cons:return1" = extractvalue { i32, ptr } %0, 1
  %"extract-cons:" = load { ptr, ptr }, ptr %"extract-cons:return1", align 8
  %"get car" = extractvalue { ptr, ptr } %"extract-cons:", 0
  %"load car" = load { i32, ptr }, ptr %"get car", align 8
  %get_type3 = extractvalue { i32, ptr } %"load car", 0
  %"extract-cons:cmp-type4" = icmp eq i32 %get_type3, 6
  br i1 %"extract-cons:cmp-type4", label %"extract-cons:return2", label %error

"extract-cons:return2":                           ; preds = %"extract-cons:return"
  %"extract-cons:return5" = extractvalue { i32, ptr } %"load car", 1
  %"extract-cons:6" = load { ptr, ptr }, ptr %"extract-cons:return5", align 8
  %"get cdr" = extractvalue { ptr, ptr } %"extract-cons:6", 1
  %"load cdr" = load { i32, ptr }, ptr %"get cdr", align 8
  %get_type8 = extractvalue { i32, ptr } %"load cdr", 0
  %"extract-cons:cmp-type9" = icmp eq i32 %get_type8, 6
  br i1 %"extract-cons:cmp-type9", label %"extract-cons:return7", label %error

"extract-cons:return7":                           ; preds = %"extract-cons:return2"
  %"extract-cons:return10" = extractvalue { i32, ptr } %"load cdr", 1
  %"extract-cons:11" = load { ptr, ptr }, ptr %"extract-cons:return10", align 8
  %"get cdr12" = extractvalue { ptr, ptr } %"extract-cons:11", 1
  %"load cdr13" = load { i32, ptr }, ptr %"get cdr12", align 8
  %get_type15 = extractvalue { i32, ptr } %"load cdr13", 0
  %"extract-cons:cmp-type16" = icmp eq i32 %get_type15, 6
  br i1 %"extract-cons:cmp-type16", label %"extract-cons:return14", label %error

"extract-cons:return14":                          ; preds = %"extract-cons:return7"
  %"extract-cons:return17" = extractvalue { i32, ptr } %"load cdr13", 1
  %"extract-cons:18" = load { ptr, ptr }, ptr %"extract-cons:return17", align 8
  %"get cdr19" = extractvalue { ptr, ptr } %"extract-cons:18", 1
  %"load cdr20" = load { i32, ptr }, ptr %"get cdr19", align 8
  ret { i32, ptr } %"load cdr20"
}

define { i32, ptr } @caaaar({ i32, ptr } %0) {
entry:
  %get_type = extractvalue { i32, ptr } %0, 0
  %"extract-cons:cmp-type" = icmp eq i32 %get_type, 6
  br i1 %"extract-cons:cmp-type", label %"extract-cons:return", label %error

error:                                            ; preds = %"extract-cons:return16", %"extract-cons:return9", %"extract-cons:return2", %"extract-cons:return", %entry
  %"error phi" = phi { ptr, i32 } [ { ptr @"error exit.87", i32 1 }, %entry ], [ { ptr @"error exit.88", i32 1 }, %"extract-cons:return" ], [ { ptr @"error exit.89", i32 1 }, %"extract-cons:return2" ], [ { ptr @"error exit.90", i32 1 }, %"extract-cons:return9" ], [ { ptr @"error exit.91", i32 1 }, %"extract-cons:return16" ]
  %error_msg = extractvalue { ptr, i32 } %"error phi", 0
  %error_code = extractvalue { ptr, i32 } %"error phi", 1
  %print = call i32 (ptr, ...) @printf(ptr %error_msg)
  call void @exit(i32 %error_code)
  unreachable

"extract-cons:return":                            ; preds = %entry
  %"extract-cons:return1" = extractvalue { i32, ptr } %0, 1
  %"extract-cons:" = load { ptr, ptr }, ptr %"extract-cons:return1", align 8
  %"get car" = extractvalue { ptr, ptr } %"extract-cons:", 0
  %"load car" = load { i32, ptr }, ptr %"get car", align 8
  %get_type3 = extractvalue { i32, ptr } %"load car", 0
  %"extract-cons:cmp-type4" = icmp eq i32 %get_type3, 6
  br i1 %"extract-cons:cmp-type4", label %"extract-cons:return2", label %error

"extract-cons:return2":                           ; preds = %"extract-cons:return"
  %"extract-cons:return5" = extractvalue { i32, ptr } %"load car", 1
  %"extract-cons:6" = load { ptr, ptr }, ptr %"extract-cons:return5", align 8
  %"get car7" = extractvalue { ptr, ptr } %"extract-cons:6", 0
  %"load car8" = load { i32, ptr }, ptr %"get car7", align 8
  %get_type10 = extractvalue { i32, ptr } %"load car8", 0
  %"extract-cons:cmp-type11" = icmp eq i32 %get_type10, 6
  br i1 %"extract-cons:cmp-type11", label %"extract-cons:return9", label %error

"extract-cons:return9":                           ; preds = %"extract-cons:return2"
  %"extract-cons:return12" = extractvalue { i32, ptr } %"load car8", 1
  %"extract-cons:13" = load { ptr, ptr }, ptr %"extract-cons:return12", align 8
  %"get car14" = extractvalue { ptr, ptr } %"extract-cons:13", 0
  %"load car15" = load { i32, ptr }, ptr %"get car14", align 8
  %get_type17 = extractvalue { i32, ptr } %"load car15", 0
  %"extract-cons:cmp-type18" = icmp eq i32 %get_type17, 6
  br i1 %"extract-cons:cmp-type18", label %"extract-cons:return16", label %error

"extract-cons:return16":                          ; preds = %"extract-cons:return9"
  %"extract-cons:return19" = extractvalue { i32, ptr } %"load car15", 1
  %"extract-cons:20" = load { ptr, ptr }, ptr %"extract-cons:return19", align 8
  %"get car21" = extractvalue { ptr, ptr } %"extract-cons:20", 0
  %"load car22" = load { i32, ptr }, ptr %"get car21", align 8
  %get_type24 = extractvalue { i32, ptr } %"load car22", 0
  %"extract-cons:cmp-type25" = icmp eq i32 %get_type24, 6
  br i1 %"extract-cons:cmp-type25", label %"extract-cons:return23", label %error

"extract-cons:return23":                          ; preds = %"extract-cons:return16"
  %"extract-cons:return26" = extractvalue { i32, ptr } %"load car22", 1
  %"extract-cons:27" = load { ptr, ptr }, ptr %"extract-cons:return26", align 8
  %"get car28" = extractvalue { ptr, ptr } %"extract-cons:27", 0
  %"load car29" = load { i32, ptr }, ptr %"get car28", align 8
  ret { i32, ptr } %"load car29"
}

define { i32, ptr } @caaadr({ i32, ptr } %0) {
entry:
  %get_type = extractvalue { i32, ptr } %0, 0
  %"extract-cons:cmp-type" = icmp eq i32 %get_type, 6
  br i1 %"extract-cons:cmp-type", label %"extract-cons:return", label %error

error:                                            ; preds = %"extract-cons:return14", %"extract-cons:return7", %"extract-cons:return2", %"extract-cons:return", %entry
  %"error phi" = phi { ptr, i32 } [ { ptr @"error exit.92", i32 1 }, %entry ], [ { ptr @"error exit.93", i32 1 }, %"extract-cons:return" ], [ { ptr @"error exit.94", i32 1 }, %"extract-cons:return2" ], [ { ptr @"error exit.95", i32 1 }, %"extract-cons:return7" ], [ { ptr @"error exit.96", i32 1 }, %"extract-cons:return14" ]
  %error_msg = extractvalue { ptr, i32 } %"error phi", 0
  %error_code = extractvalue { ptr, i32 } %"error phi", 1
  %print = call i32 (ptr, ...) @printf(ptr %error_msg)
  call void @exit(i32 %error_code)
  unreachable

"extract-cons:return":                            ; preds = %entry
  %"extract-cons:return1" = extractvalue { i32, ptr } %0, 1
  %"extract-cons:" = load { ptr, ptr }, ptr %"extract-cons:return1", align 8
  %"get car" = extractvalue { ptr, ptr } %"extract-cons:", 0
  %"load car" = load { i32, ptr }, ptr %"get car", align 8
  %get_type3 = extractvalue { i32, ptr } %"load car", 0
  %"extract-cons:cmp-type4" = icmp eq i32 %get_type3, 6
  br i1 %"extract-cons:cmp-type4", label %"extract-cons:return2", label %error

"extract-cons:return2":                           ; preds = %"extract-cons:return"
  %"extract-cons:return5" = extractvalue { i32, ptr } %"load car", 1
  %"extract-cons:6" = load { ptr, ptr }, ptr %"extract-cons:return5", align 8
  %"get cdr" = extractvalue { ptr, ptr } %"extract-cons:6", 1
  %"load cdr" = load { i32, ptr }, ptr %"get cdr", align 8
  %get_type8 = extractvalue { i32, ptr } %"load cdr", 0
  %"extract-cons:cmp-type9" = icmp eq i32 %get_type8, 6
  br i1 %"extract-cons:cmp-type9", label %"extract-cons:return7", label %error

"extract-cons:return7":                           ; preds = %"extract-cons:return2"
  %"extract-cons:return10" = extractvalue { i32, ptr } %"load cdr", 1
  %"extract-cons:11" = load { ptr, ptr }, ptr %"extract-cons:return10", align 8
  %"get car12" = extractvalue { ptr, ptr } %"extract-cons:11", 0
  %"load car13" = load { i32, ptr }, ptr %"get car12", align 8
  %get_type15 = extractvalue { i32, ptr } %"load car13", 0
  %"extract-cons:cmp-type16" = icmp eq i32 %get_type15, 6
  br i1 %"extract-cons:cmp-type16", label %"extract-cons:return14", label %error

"extract-cons:return14":                          ; preds = %"extract-cons:return7"
  %"extract-cons:return17" = extractvalue { i32, ptr } %"load car13", 1
  %"extract-cons:18" = load { ptr, ptr }, ptr %"extract-cons:return17", align 8
  %"get car19" = extractvalue { ptr, ptr } %"extract-cons:18", 0
  %"load car20" = load { i32, ptr }, ptr %"get car19", align 8
  %get_type22 = extractvalue { i32, ptr } %"load car20", 0
  %"extract-cons:cmp-type23" = icmp eq i32 %get_type22, 6
  br i1 %"extract-cons:cmp-type23", label %"extract-cons:return21", label %error

"extract-cons:return21":                          ; preds = %"extract-cons:return14"
  %"extract-cons:return24" = extractvalue { i32, ptr } %"load car20", 1
  %"extract-cons:25" = load { ptr, ptr }, ptr %"extract-cons:return24", align 8
  %"get car26" = extractvalue { ptr, ptr } %"extract-cons:25", 0
  %"load car27" = load { i32, ptr }, ptr %"get car26", align 8
  ret { i32, ptr } %"load car27"
}

define { i32, ptr } @caadar({ i32, ptr } %0) {
entry:
  %get_type = extractvalue { i32, ptr } %0, 0
  %"extract-cons:cmp-type" = icmp eq i32 %get_type, 6
  br i1 %"extract-cons:cmp-type", label %"extract-cons:return", label %error

error:                                            ; preds = %"extract-cons:return14", %"extract-cons:return9", %"extract-cons:return2", %"extract-cons:return", %entry
  %"error phi" = phi { ptr, i32 } [ { ptr @"error exit.97", i32 1 }, %entry ], [ { ptr @"error exit.98", i32 1 }, %"extract-cons:return" ], [ { ptr @"error exit.99", i32 1 }, %"extract-cons:return2" ], [ { ptr @"error exit.100", i32 1 }, %"extract-cons:return9" ], [ { ptr @"error exit.101", i32 1 }, %"extract-cons:return14" ]
  %error_msg = extractvalue { ptr, i32 } %"error phi", 0
  %error_code = extractvalue { ptr, i32 } %"error phi", 1
  %print = call i32 (ptr, ...) @printf(ptr %error_msg)
  call void @exit(i32 %error_code)
  unreachable

"extract-cons:return":                            ; preds = %entry
  %"extract-cons:return1" = extractvalue { i32, ptr } %0, 1
  %"extract-cons:" = load { ptr, ptr }, ptr %"extract-cons:return1", align 8
  %"get car" = extractvalue { ptr, ptr } %"extract-cons:", 0
  %"load car" = load { i32, ptr }, ptr %"get car", align 8
  %get_type3 = extractvalue { i32, ptr } %"load car", 0
  %"extract-cons:cmp-type4" = icmp eq i32 %get_type3, 6
  br i1 %"extract-cons:cmp-type4", label %"extract-cons:return2", label %error

"extract-cons:return2":                           ; preds = %"extract-cons:return"
  %"extract-cons:return5" = extractvalue { i32, ptr } %"load car", 1
  %"extract-cons:6" = load { ptr, ptr }, ptr %"extract-cons:return5", align 8
  %"get car7" = extractvalue { ptr, ptr } %"extract-cons:6", 0
  %"load car8" = load { i32, ptr }, ptr %"get car7", align 8
  %get_type10 = extractvalue { i32, ptr } %"load car8", 0
  %"extract-cons:cmp-type11" = icmp eq i32 %get_type10, 6
  br i1 %"extract-cons:cmp-type11", label %"extract-cons:return9", label %error

"extract-cons:return9":                           ; preds = %"extract-cons:return2"
  %"extract-cons:return12" = extractvalue { i32, ptr } %"load car8", 1
  %"extract-cons:13" = load { ptr, ptr }, ptr %"extract-cons:return12", align 8
  %"get cdr" = extractvalue { ptr, ptr } %"extract-cons:13", 1
  %"load cdr" = load { i32, ptr }, ptr %"get cdr", align 8
  %get_type15 = extractvalue { i32, ptr } %"load cdr", 0
  %"extract-cons:cmp-type16" = icmp eq i32 %get_type15, 6
  br i1 %"extract-cons:cmp-type16", label %"extract-cons:return14", label %error

"extract-cons:return14":                          ; preds = %"extract-cons:return9"
  %"extract-cons:return17" = extractvalue { i32, ptr } %"load cdr", 1
  %"extract-cons:18" = load { ptr, ptr }, ptr %"extract-cons:return17", align 8
  %"get car19" = extractvalue { ptr, ptr } %"extract-cons:18", 0
  %"load car20" = load { i32, ptr }, ptr %"get car19", align 8
  %get_type22 = extractvalue { i32, ptr } %"load car20", 0
  %"extract-cons:cmp-type23" = icmp eq i32 %get_type22, 6
  br i1 %"extract-cons:cmp-type23", label %"extract-cons:return21", label %error

"extract-cons:return21":                          ; preds = %"extract-cons:return14"
  %"extract-cons:return24" = extractvalue { i32, ptr } %"load car20", 1
  %"extract-cons:25" = load { ptr, ptr }, ptr %"extract-cons:return24", align 8
  %"get car26" = extractvalue { ptr, ptr } %"extract-cons:25", 0
  %"load car27" = load { i32, ptr }, ptr %"get car26", align 8
  ret { i32, ptr } %"load car27"
}

define { i32, ptr } @caaddr({ i32, ptr } %0) {
entry:
  %get_type = extractvalue { i32, ptr } %0, 0
  %"extract-cons:cmp-type" = icmp eq i32 %get_type, 6
  br i1 %"extract-cons:cmp-type", label %"extract-cons:return", label %error

error:                                            ; preds = %"extract-cons:return14", %"extract-cons:return7", %"extract-cons:return2", %"extract-cons:return", %entry
  %"error phi" = phi { ptr, i32 } [ { ptr @"error exit.102", i32 1 }, %entry ], [ { ptr @"error exit.103", i32 1 }, %"extract-cons:return" ], [ { ptr @"error exit.104", i32 1 }, %"extract-cons:return2" ], [ { ptr @"error exit.105", i32 1 }, %"extract-cons:return7" ], [ { ptr @"error exit.106", i32 1 }, %"extract-cons:return14" ]
  %error_msg = extractvalue { ptr, i32 } %"error phi", 0
  %error_code = extractvalue { ptr, i32 } %"error phi", 1
  %print = call i32 (ptr, ...) @printf(ptr %error_msg)
  call void @exit(i32 %error_code)
  unreachable

"extract-cons:return":                            ; preds = %entry
  %"extract-cons:return1" = extractvalue { i32, ptr } %0, 1
  %"extract-cons:" = load { ptr, ptr }, ptr %"extract-cons:return1", align 8
  %"get car" = extractvalue { ptr, ptr } %"extract-cons:", 0
  %"load car" = load { i32, ptr }, ptr %"get car", align 8
  %get_type3 = extractvalue { i32, ptr } %"load car", 0
  %"extract-cons:cmp-type4" = icmp eq i32 %get_type3, 6
  br i1 %"extract-cons:cmp-type4", label %"extract-cons:return2", label %error

"extract-cons:return2":                           ; preds = %"extract-cons:return"
  %"extract-cons:return5" = extractvalue { i32, ptr } %"load car", 1
  %"extract-cons:6" = load { ptr, ptr }, ptr %"extract-cons:return5", align 8
  %"get cdr" = extractvalue { ptr, ptr } %"extract-cons:6", 1
  %"load cdr" = load { i32, ptr }, ptr %"get cdr", align 8
  %get_type8 = extractvalue { i32, ptr } %"load cdr", 0
  %"extract-cons:cmp-type9" = icmp eq i32 %get_type8, 6
  br i1 %"extract-cons:cmp-type9", label %"extract-cons:return7", label %error

"extract-cons:return7":                           ; preds = %"extract-cons:return2"
  %"extract-cons:return10" = extractvalue { i32, ptr } %"load cdr", 1
  %"extract-cons:11" = load { ptr, ptr }, ptr %"extract-cons:return10", align 8
  %"get cdr12" = extractvalue { ptr, ptr } %"extract-cons:11", 1
  %"load cdr13" = load { i32, ptr }, ptr %"get cdr12", align 8
  %get_type15 = extractvalue { i32, ptr } %"load cdr13", 0
  %"extract-cons:cmp-type16" = icmp eq i32 %get_type15, 6
  br i1 %"extract-cons:cmp-type16", label %"extract-cons:return14", label %error

"extract-cons:return14":                          ; preds = %"extract-cons:return7"
  %"extract-cons:return17" = extractvalue { i32, ptr } %"load cdr13", 1
  %"extract-cons:18" = load { ptr, ptr }, ptr %"extract-cons:return17", align 8
  %"get car19" = extractvalue { ptr, ptr } %"extract-cons:18", 0
  %"load car20" = load { i32, ptr }, ptr %"get car19", align 8
  %get_type22 = extractvalue { i32, ptr } %"load car20", 0
  %"extract-cons:cmp-type23" = icmp eq i32 %get_type22, 6
  br i1 %"extract-cons:cmp-type23", label %"extract-cons:return21", label %error

"extract-cons:return21":                          ; preds = %"extract-cons:return14"
  %"extract-cons:return24" = extractvalue { i32, ptr } %"load car20", 1
  %"extract-cons:25" = load { ptr, ptr }, ptr %"extract-cons:return24", align 8
  %"get car26" = extractvalue { ptr, ptr } %"extract-cons:25", 0
  %"load car27" = load { i32, ptr }, ptr %"get car26", align 8
  ret { i32, ptr } %"load car27"
}

define { i32, ptr } @cadaar({ i32, ptr } %0) {
entry:
  %get_type = extractvalue { i32, ptr } %0, 0
  %"extract-cons:cmp-type" = icmp eq i32 %get_type, 6
  br i1 %"extract-cons:cmp-type", label %"extract-cons:return", label %error

error:                                            ; preds = %"extract-cons:return16", %"extract-cons:return9", %"extract-cons:return2", %"extract-cons:return", %entry
  %"error phi" = phi { ptr, i32 } [ { ptr @"error exit.107", i32 1 }, %entry ], [ { ptr @"error exit.108", i32 1 }, %"extract-cons:return" ], [ { ptr @"error exit.109", i32 1 }, %"extract-cons:return2" ], [ { ptr @"error exit.110", i32 1 }, %"extract-cons:return9" ], [ { ptr @"error exit.111", i32 1 }, %"extract-cons:return16" ]
  %error_msg = extractvalue { ptr, i32 } %"error phi", 0
  %error_code = extractvalue { ptr, i32 } %"error phi", 1
  %print = call i32 (ptr, ...) @printf(ptr %error_msg)
  call void @exit(i32 %error_code)
  unreachable

"extract-cons:return":                            ; preds = %entry
  %"extract-cons:return1" = extractvalue { i32, ptr } %0, 1
  %"extract-cons:" = load { ptr, ptr }, ptr %"extract-cons:return1", align 8
  %"get car" = extractvalue { ptr, ptr } %"extract-cons:", 0
  %"load car" = load { i32, ptr }, ptr %"get car", align 8
  %get_type3 = extractvalue { i32, ptr } %"load car", 0
  %"extract-cons:cmp-type4" = icmp eq i32 %get_type3, 6
  br i1 %"extract-cons:cmp-type4", label %"extract-cons:return2", label %error

"extract-cons:return2":                           ; preds = %"extract-cons:return"
  %"extract-cons:return5" = extractvalue { i32, ptr } %"load car", 1
  %"extract-cons:6" = load { ptr, ptr }, ptr %"extract-cons:return5", align 8
  %"get car7" = extractvalue { ptr, ptr } %"extract-cons:6", 0
  %"load car8" = load { i32, ptr }, ptr %"get car7", align 8
  %get_type10 = extractvalue { i32, ptr } %"load car8", 0
  %"extract-cons:cmp-type11" = icmp eq i32 %get_type10, 6
  br i1 %"extract-cons:cmp-type11", label %"extract-cons:return9", label %error

"extract-cons:return9":                           ; preds = %"extract-cons:return2"
  %"extract-cons:return12" = extractvalue { i32, ptr } %"load car8", 1
  %"extract-cons:13" = load { ptr, ptr }, ptr %"extract-cons:return12", align 8
  %"get car14" = extractvalue { ptr, ptr } %"extract-cons:13", 0
  %"load car15" = load { i32, ptr }, ptr %"get car14", align 8
  %get_type17 = extractvalue { i32, ptr } %"load car15", 0
  %"extract-cons:cmp-type18" = icmp eq i32 %get_type17, 6
  br i1 %"extract-cons:cmp-type18", label %"extract-cons:return16", label %error

"extract-cons:return16":                          ; preds = %"extract-cons:return9"
  %"extract-cons:return19" = extractvalue { i32, ptr } %"load car15", 1
  %"extract-cons:20" = load { ptr, ptr }, ptr %"extract-cons:return19", align 8
  %"get cdr" = extractvalue { ptr, ptr } %"extract-cons:20", 1
  %"load cdr" = load { i32, ptr }, ptr %"get cdr", align 8
  %get_type22 = extractvalue { i32, ptr } %"load cdr", 0
  %"extract-cons:cmp-type23" = icmp eq i32 %get_type22, 6
  br i1 %"extract-cons:cmp-type23", label %"extract-cons:return21", label %error

"extract-cons:return21":                          ; preds = %"extract-cons:return16"
  %"extract-cons:return24" = extractvalue { i32, ptr } %"load cdr", 1
  %"extract-cons:25" = load { ptr, ptr }, ptr %"extract-cons:return24", align 8
  %"get car26" = extractvalue { ptr, ptr } %"extract-cons:25", 0
  %"load car27" = load { i32, ptr }, ptr %"get car26", align 8
  ret { i32, ptr } %"load car27"
}

define { i32, ptr } @cadadr({ i32, ptr } %0) {
entry:
  %get_type = extractvalue { i32, ptr } %0, 0
  %"extract-cons:cmp-type" = icmp eq i32 %get_type, 6
  br i1 %"extract-cons:cmp-type", label %"extract-cons:return", label %error

error:                                            ; preds = %"extract-cons:return14", %"extract-cons:return7", %"extract-cons:return2", %"extract-cons:return", %entry
  %"error phi" = phi { ptr, i32 } [ { ptr @"error exit.112", i32 1 }, %entry ], [ { ptr @"error exit.113", i32 1 }, %"extract-cons:return" ], [ { ptr @"error exit.114", i32 1 }, %"extract-cons:return2" ], [ { ptr @"error exit.115", i32 1 }, %"extract-cons:return7" ], [ { ptr @"error exit.116", i32 1 }, %"extract-cons:return14" ]
  %error_msg = extractvalue { ptr, i32 } %"error phi", 0
  %error_code = extractvalue { ptr, i32 } %"error phi", 1
  %print = call i32 (ptr, ...) @printf(ptr %error_msg)
  call void @exit(i32 %error_code)
  unreachable

"extract-cons:return":                            ; preds = %entry
  %"extract-cons:return1" = extractvalue { i32, ptr } %0, 1
  %"extract-cons:" = load { ptr, ptr }, ptr %"extract-cons:return1", align 8
  %"get car" = extractvalue { ptr, ptr } %"extract-cons:", 0
  %"load car" = load { i32, ptr }, ptr %"get car", align 8
  %get_type3 = extractvalue { i32, ptr } %"load car", 0
  %"extract-cons:cmp-type4" = icmp eq i32 %get_type3, 6
  br i1 %"extract-cons:cmp-type4", label %"extract-cons:return2", label %error

"extract-cons:return2":                           ; preds = %"extract-cons:return"
  %"extract-cons:return5" = extractvalue { i32, ptr } %"load car", 1
  %"extract-cons:6" = load { ptr, ptr }, ptr %"extract-cons:return5", align 8
  %"get cdr" = extractvalue { ptr, ptr } %"extract-cons:6", 1
  %"load cdr" = load { i32, ptr }, ptr %"get cdr", align 8
  %get_type8 = extractvalue { i32, ptr } %"load cdr", 0
  %"extract-cons:cmp-type9" = icmp eq i32 %get_type8, 6
  br i1 %"extract-cons:cmp-type9", label %"extract-cons:return7", label %error

"extract-cons:return7":                           ; preds = %"extract-cons:return2"
  %"extract-cons:return10" = extractvalue { i32, ptr } %"load cdr", 1
  %"extract-cons:11" = load { ptr, ptr }, ptr %"extract-cons:return10", align 8
  %"get car12" = extractvalue { ptr, ptr } %"extract-cons:11", 0
  %"load car13" = load { i32, ptr }, ptr %"get car12", align 8
  %get_type15 = extractvalue { i32, ptr } %"load car13", 0
  %"extract-cons:cmp-type16" = icmp eq i32 %get_type15, 6
  br i1 %"extract-cons:cmp-type16", label %"extract-cons:return14", label %error

"extract-cons:return14":                          ; preds = %"extract-cons:return7"
  %"extract-cons:return17" = extractvalue { i32, ptr } %"load car13", 1
  %"extract-cons:18" = load { ptr, ptr }, ptr %"extract-cons:return17", align 8
  %"get cdr19" = extractvalue { ptr, ptr } %"extract-cons:18", 1
  %"load cdr20" = load { i32, ptr }, ptr %"get cdr19", align 8
  %get_type22 = extractvalue { i32, ptr } %"load cdr20", 0
  %"extract-cons:cmp-type23" = icmp eq i32 %get_type22, 6
  br i1 %"extract-cons:cmp-type23", label %"extract-cons:return21", label %error

"extract-cons:return21":                          ; preds = %"extract-cons:return14"
  %"extract-cons:return24" = extractvalue { i32, ptr } %"load cdr20", 1
  %"extract-cons:25" = load { ptr, ptr }, ptr %"extract-cons:return24", align 8
  %"get car26" = extractvalue { ptr, ptr } %"extract-cons:25", 0
  %"load car27" = load { i32, ptr }, ptr %"get car26", align 8
  ret { i32, ptr } %"load car27"
}

define { i32, ptr } @caddar({ i32, ptr } %0) {
entry:
  %get_type = extractvalue { i32, ptr } %0, 0
  %"extract-cons:cmp-type" = icmp eq i32 %get_type, 6
  br i1 %"extract-cons:cmp-type", label %"extract-cons:return", label %error

error:                                            ; preds = %"extract-cons:return14", %"extract-cons:return9", %"extract-cons:return2", %"extract-cons:return", %entry
  %"error phi" = phi { ptr, i32 } [ { ptr @"error exit.117", i32 1 }, %entry ], [ { ptr @"error exit.118", i32 1 }, %"extract-cons:return" ], [ { ptr @"error exit.119", i32 1 }, %"extract-cons:return2" ], [ { ptr @"error exit.120", i32 1 }, %"extract-cons:return9" ], [ { ptr @"error exit.121", i32 1 }, %"extract-cons:return14" ]
  %error_msg = extractvalue { ptr, i32 } %"error phi", 0
  %error_code = extractvalue { ptr, i32 } %"error phi", 1
  %print = call i32 (ptr, ...) @printf(ptr %error_msg)
  call void @exit(i32 %error_code)
  unreachable

"extract-cons:return":                            ; preds = %entry
  %"extract-cons:return1" = extractvalue { i32, ptr } %0, 1
  %"extract-cons:" = load { ptr, ptr }, ptr %"extract-cons:return1", align 8
  %"get car" = extractvalue { ptr, ptr } %"extract-cons:", 0
  %"load car" = load { i32, ptr }, ptr %"get car", align 8
  %get_type3 = extractvalue { i32, ptr } %"load car", 0
  %"extract-cons:cmp-type4" = icmp eq i32 %get_type3, 6
  br i1 %"extract-cons:cmp-type4", label %"extract-cons:return2", label %error

"extract-cons:return2":                           ; preds = %"extract-cons:return"
  %"extract-cons:return5" = extractvalue { i32, ptr } %"load car", 1
  %"extract-cons:6" = load { ptr, ptr }, ptr %"extract-cons:return5", align 8
  %"get car7" = extractvalue { ptr, ptr } %"extract-cons:6", 0
  %"load car8" = load { i32, ptr }, ptr %"get car7", align 8
  %get_type10 = extractvalue { i32, ptr } %"load car8", 0
  %"extract-cons:cmp-type11" = icmp eq i32 %get_type10, 6
  br i1 %"extract-cons:cmp-type11", label %"extract-cons:return9", label %error

"extract-cons:return9":                           ; preds = %"extract-cons:return2"
  %"extract-cons:return12" = extractvalue { i32, ptr } %"load car8", 1
  %"extract-cons:13" = load { ptr, ptr }, ptr %"extract-cons:return12", align 8
  %"get cdr" = extractvalue { ptr, ptr } %"extract-cons:13", 1
  %"load cdr" = load { i32, ptr }, ptr %"get cdr", align 8
  %get_type15 = extractvalue { i32, ptr } %"load cdr", 0
  %"extract-cons:cmp-type16" = icmp eq i32 %get_type15, 6
  br i1 %"extract-cons:cmp-type16", label %"extract-cons:return14", label %error

"extract-cons:return14":                          ; preds = %"extract-cons:return9"
  %"extract-cons:return17" = extractvalue { i32, ptr } %"load cdr", 1
  %"extract-cons:18" = load { ptr, ptr }, ptr %"extract-cons:return17", align 8
  %"get cdr19" = extractvalue { ptr, ptr } %"extract-cons:18", 1
  %"load cdr20" = load { i32, ptr }, ptr %"get cdr19", align 8
  %get_type22 = extractvalue { i32, ptr } %"load cdr20", 0
  %"extract-cons:cmp-type23" = icmp eq i32 %get_type22, 6
  br i1 %"extract-cons:cmp-type23", label %"extract-cons:return21", label %error

"extract-cons:return21":                          ; preds = %"extract-cons:return14"
  %"extract-cons:return24" = extractvalue { i32, ptr } %"load cdr20", 1
  %"extract-cons:25" = load { ptr, ptr }, ptr %"extract-cons:return24", align 8
  %"get car26" = extractvalue { ptr, ptr } %"extract-cons:25", 0
  %"load car27" = load { i32, ptr }, ptr %"get car26", align 8
  ret { i32, ptr } %"load car27"
}

define { i32, ptr } @cadddr({ i32, ptr } %0) {
entry:
  %get_type = extractvalue { i32, ptr } %0, 0
  %"extract-cons:cmp-type" = icmp eq i32 %get_type, 6
  br i1 %"extract-cons:cmp-type", label %"extract-cons:return", label %error

error:                                            ; preds = %"extract-cons:return14", %"extract-cons:return7", %"extract-cons:return2", %"extract-cons:return", %entry
  %"error phi" = phi { ptr, i32 } [ { ptr @"error exit.122", i32 1 }, %entry ], [ { ptr @"error exit.123", i32 1 }, %"extract-cons:return" ], [ { ptr @"error exit.124", i32 1 }, %"extract-cons:return2" ], [ { ptr @"error exit.125", i32 1 }, %"extract-cons:return7" ], [ { ptr @"error exit.126", i32 1 }, %"extract-cons:return14" ]
  %error_msg = extractvalue { ptr, i32 } %"error phi", 0
  %error_code = extractvalue { ptr, i32 } %"error phi", 1
  %print = call i32 (ptr, ...) @printf(ptr %error_msg)
  call void @exit(i32 %error_code)
  unreachable

"extract-cons:return":                            ; preds = %entry
  %"extract-cons:return1" = extractvalue { i32, ptr } %0, 1
  %"extract-cons:" = load { ptr, ptr }, ptr %"extract-cons:return1", align 8
  %"get car" = extractvalue { ptr, ptr } %"extract-cons:", 0
  %"load car" = load { i32, ptr }, ptr %"get car", align 8
  %get_type3 = extractvalue { i32, ptr } %"load car", 0
  %"extract-cons:cmp-type4" = icmp eq i32 %get_type3, 6
  br i1 %"extract-cons:cmp-type4", label %"extract-cons:return2", label %error

"extract-cons:return2":                           ; preds = %"extract-cons:return"
  %"extract-cons:return5" = extractvalue { i32, ptr } %"load car", 1
  %"extract-cons:6" = load { ptr, ptr }, ptr %"extract-cons:return5", align 8
  %"get cdr" = extractvalue { ptr, ptr } %"extract-cons:6", 1
  %"load cdr" = load { i32, ptr }, ptr %"get cdr", align 8
  %get_type8 = extractvalue { i32, ptr } %"load cdr", 0
  %"extract-cons:cmp-type9" = icmp eq i32 %get_type8, 6
  br i1 %"extract-cons:cmp-type9", label %"extract-cons:return7", label %error

"extract-cons:return7":                           ; preds = %"extract-cons:return2"
  %"extract-cons:return10" = extractvalue { i32, ptr } %"load cdr", 1
  %"extract-cons:11" = load { ptr, ptr }, ptr %"extract-cons:return10", align 8
  %"get cdr12" = extractvalue { ptr, ptr } %"extract-cons:11", 1
  %"load cdr13" = load { i32, ptr }, ptr %"get cdr12", align 8
  %get_type15 = extractvalue { i32, ptr } %"load cdr13", 0
  %"extract-cons:cmp-type16" = icmp eq i32 %get_type15, 6
  br i1 %"extract-cons:cmp-type16", label %"extract-cons:return14", label %error

"extract-cons:return14":                          ; preds = %"extract-cons:return7"
  %"extract-cons:return17" = extractvalue { i32, ptr } %"load cdr13", 1
  %"extract-cons:18" = load { ptr, ptr }, ptr %"extract-cons:return17", align 8
  %"get cdr19" = extractvalue { ptr, ptr } %"extract-cons:18", 1
  %"load cdr20" = load { i32, ptr }, ptr %"get cdr19", align 8
  %get_type22 = extractvalue { i32, ptr } %"load cdr20", 0
  %"extract-cons:cmp-type23" = icmp eq i32 %get_type22, 6
  br i1 %"extract-cons:cmp-type23", label %"extract-cons:return21", label %error

"extract-cons:return21":                          ; preds = %"extract-cons:return14"
  %"extract-cons:return24" = extractvalue { i32, ptr } %"load cdr20", 1
  %"extract-cons:25" = load { ptr, ptr }, ptr %"extract-cons:return24", align 8
  %"get car26" = extractvalue { ptr, ptr } %"extract-cons:25", 0
  %"load car27" = load { i32, ptr }, ptr %"get car26", align 8
  ret { i32, ptr } %"load car27"
}

define { i32, ptr } @cdaaar({ i32, ptr } %0) {
entry:
  %get_type = extractvalue { i32, ptr } %0, 0
  %"extract-cons:cmp-type" = icmp eq i32 %get_type, 6
  br i1 %"extract-cons:cmp-type", label %"extract-cons:return", label %error

error:                                            ; preds = %"extract-cons:return16", %"extract-cons:return9", %"extract-cons:return2", %"extract-cons:return", %entry
  %"error phi" = phi { ptr, i32 } [ { ptr @"error exit.127", i32 1 }, %entry ], [ { ptr @"error exit.128", i32 1 }, %"extract-cons:return" ], [ { ptr @"error exit.129", i32 1 }, %"extract-cons:return2" ], [ { ptr @"error exit.130", i32 1 }, %"extract-cons:return9" ], [ { ptr @"error exit.131", i32 1 }, %"extract-cons:return16" ]
  %error_msg = extractvalue { ptr, i32 } %"error phi", 0
  %error_code = extractvalue { ptr, i32 } %"error phi", 1
  %print = call i32 (ptr, ...) @printf(ptr %error_msg)
  call void @exit(i32 %error_code)
  unreachable

"extract-cons:return":                            ; preds = %entry
  %"extract-cons:return1" = extractvalue { i32, ptr } %0, 1
  %"extract-cons:" = load { ptr, ptr }, ptr %"extract-cons:return1", align 8
  %"get car" = extractvalue { ptr, ptr } %"extract-cons:", 0
  %"load car" = load { i32, ptr }, ptr %"get car", align 8
  %get_type3 = extractvalue { i32, ptr } %"load car", 0
  %"extract-cons:cmp-type4" = icmp eq i32 %get_type3, 6
  br i1 %"extract-cons:cmp-type4", label %"extract-cons:return2", label %error

"extract-cons:return2":                           ; preds = %"extract-cons:return"
  %"extract-cons:return5" = extractvalue { i32, ptr } %"load car", 1
  %"extract-cons:6" = load { ptr, ptr }, ptr %"extract-cons:return5", align 8
  %"get car7" = extractvalue { ptr, ptr } %"extract-cons:6", 0
  %"load car8" = load { i32, ptr }, ptr %"get car7", align 8
  %get_type10 = extractvalue { i32, ptr } %"load car8", 0
  %"extract-cons:cmp-type11" = icmp eq i32 %get_type10, 6
  br i1 %"extract-cons:cmp-type11", label %"extract-cons:return9", label %error

"extract-cons:return9":                           ; preds = %"extract-cons:return2"
  %"extract-cons:return12" = extractvalue { i32, ptr } %"load car8", 1
  %"extract-cons:13" = load { ptr, ptr }, ptr %"extract-cons:return12", align 8
  %"get car14" = extractvalue { ptr, ptr } %"extract-cons:13", 0
  %"load car15" = load { i32, ptr }, ptr %"get car14", align 8
  %get_type17 = extractvalue { i32, ptr } %"load car15", 0
  %"extract-cons:cmp-type18" = icmp eq i32 %get_type17, 6
  br i1 %"extract-cons:cmp-type18", label %"extract-cons:return16", label %error

"extract-cons:return16":                          ; preds = %"extract-cons:return9"
  %"extract-cons:return19" = extractvalue { i32, ptr } %"load car15", 1
  %"extract-cons:20" = load { ptr, ptr }, ptr %"extract-cons:return19", align 8
  %"get car21" = extractvalue { ptr, ptr } %"extract-cons:20", 0
  %"load car22" = load { i32, ptr }, ptr %"get car21", align 8
  %get_type24 = extractvalue { i32, ptr } %"load car22", 0
  %"extract-cons:cmp-type25" = icmp eq i32 %get_type24, 6
  br i1 %"extract-cons:cmp-type25", label %"extract-cons:return23", label %error

"extract-cons:return23":                          ; preds = %"extract-cons:return16"
  %"extract-cons:return26" = extractvalue { i32, ptr } %"load car22", 1
  %"extract-cons:27" = load { ptr, ptr }, ptr %"extract-cons:return26", align 8
  %"get cdr" = extractvalue { ptr, ptr } %"extract-cons:27", 1
  %"load cdr" = load { i32, ptr }, ptr %"get cdr", align 8
  ret { i32, ptr } %"load cdr"
}

define { i32, ptr } @cdaadr({ i32, ptr } %0) {
entry:
  %get_type = extractvalue { i32, ptr } %0, 0
  %"extract-cons:cmp-type" = icmp eq i32 %get_type, 6
  br i1 %"extract-cons:cmp-type", label %"extract-cons:return", label %error

error:                                            ; preds = %"extract-cons:return14", %"extract-cons:return7", %"extract-cons:return2", %"extract-cons:return", %entry
  %"error phi" = phi { ptr, i32 } [ { ptr @"error exit.132", i32 1 }, %entry ], [ { ptr @"error exit.133", i32 1 }, %"extract-cons:return" ], [ { ptr @"error exit.134", i32 1 }, %"extract-cons:return2" ], [ { ptr @"error exit.135", i32 1 }, %"extract-cons:return7" ], [ { ptr @"error exit.136", i32 1 }, %"extract-cons:return14" ]
  %error_msg = extractvalue { ptr, i32 } %"error phi", 0
  %error_code = extractvalue { ptr, i32 } %"error phi", 1
  %print = call i32 (ptr, ...) @printf(ptr %error_msg)
  call void @exit(i32 %error_code)
  unreachable

"extract-cons:return":                            ; preds = %entry
  %"extract-cons:return1" = extractvalue { i32, ptr } %0, 1
  %"extract-cons:" = load { ptr, ptr }, ptr %"extract-cons:return1", align 8
  %"get car" = extractvalue { ptr, ptr } %"extract-cons:", 0
  %"load car" = load { i32, ptr }, ptr %"get car", align 8
  %get_type3 = extractvalue { i32, ptr } %"load car", 0
  %"extract-cons:cmp-type4" = icmp eq i32 %get_type3, 6
  br i1 %"extract-cons:cmp-type4", label %"extract-cons:return2", label %error

"extract-cons:return2":                           ; preds = %"extract-cons:return"
  %"extract-cons:return5" = extractvalue { i32, ptr } %"load car", 1
  %"extract-cons:6" = load { ptr, ptr }, ptr %"extract-cons:return5", align 8
  %"get cdr" = extractvalue { ptr, ptr } %"extract-cons:6", 1
  %"load cdr" = load { i32, ptr }, ptr %"get cdr", align 8
  %get_type8 = extractvalue { i32, ptr } %"load cdr", 0
  %"extract-cons:cmp-type9" = icmp eq i32 %get_type8, 6
  br i1 %"extract-cons:cmp-type9", label %"extract-cons:return7", label %error

"extract-cons:return7":                           ; preds = %"extract-cons:return2"
  %"extract-cons:return10" = extractvalue { i32, ptr } %"load cdr", 1
  %"extract-cons:11" = load { ptr, ptr }, ptr %"extract-cons:return10", align 8
  %"get car12" = extractvalue { ptr, ptr } %"extract-cons:11", 0
  %"load car13" = load { i32, ptr }, ptr %"get car12", align 8
  %get_type15 = extractvalue { i32, ptr } %"load car13", 0
  %"extract-cons:cmp-type16" = icmp eq i32 %get_type15, 6
  br i1 %"extract-cons:cmp-type16", label %"extract-cons:return14", label %error

"extract-cons:return14":                          ; preds = %"extract-cons:return7"
  %"extract-cons:return17" = extractvalue { i32, ptr } %"load car13", 1
  %"extract-cons:18" = load { ptr, ptr }, ptr %"extract-cons:return17", align 8
  %"get car19" = extractvalue { ptr, ptr } %"extract-cons:18", 0
  %"load car20" = load { i32, ptr }, ptr %"get car19", align 8
  %get_type22 = extractvalue { i32, ptr } %"load car20", 0
  %"extract-cons:cmp-type23" = icmp eq i32 %get_type22, 6
  br i1 %"extract-cons:cmp-type23", label %"extract-cons:return21", label %error

"extract-cons:return21":                          ; preds = %"extract-cons:return14"
  %"extract-cons:return24" = extractvalue { i32, ptr } %"load car20", 1
  %"extract-cons:25" = load { ptr, ptr }, ptr %"extract-cons:return24", align 8
  %"get cdr26" = extractvalue { ptr, ptr } %"extract-cons:25", 1
  %"load cdr27" = load { i32, ptr }, ptr %"get cdr26", align 8
  ret { i32, ptr } %"load cdr27"
}

define { i32, ptr } @cdadar({ i32, ptr } %0) {
entry:
  %get_type = extractvalue { i32, ptr } %0, 0
  %"extract-cons:cmp-type" = icmp eq i32 %get_type, 6
  br i1 %"extract-cons:cmp-type", label %"extract-cons:return", label %error

error:                                            ; preds = %"extract-cons:return14", %"extract-cons:return9", %"extract-cons:return2", %"extract-cons:return", %entry
  %"error phi" = phi { ptr, i32 } [ { ptr @"error exit.137", i32 1 }, %entry ], [ { ptr @"error exit.138", i32 1 }, %"extract-cons:return" ], [ { ptr @"error exit.139", i32 1 }, %"extract-cons:return2" ], [ { ptr @"error exit.140", i32 1 }, %"extract-cons:return9" ], [ { ptr @"error exit.141", i32 1 }, %"extract-cons:return14" ]
  %error_msg = extractvalue { ptr, i32 } %"error phi", 0
  %error_code = extractvalue { ptr, i32 } %"error phi", 1
  %print = call i32 (ptr, ...) @printf(ptr %error_msg)
  call void @exit(i32 %error_code)
  unreachable

"extract-cons:return":                            ; preds = %entry
  %"extract-cons:return1" = extractvalue { i32, ptr } %0, 1
  %"extract-cons:" = load { ptr, ptr }, ptr %"extract-cons:return1", align 8
  %"get car" = extractvalue { ptr, ptr } %"extract-cons:", 0
  %"load car" = load { i32, ptr }, ptr %"get car", align 8
  %get_type3 = extractvalue { i32, ptr } %"load car", 0
  %"extract-cons:cmp-type4" = icmp eq i32 %get_type3, 6
  br i1 %"extract-cons:cmp-type4", label %"extract-cons:return2", label %error

"extract-cons:return2":                           ; preds = %"extract-cons:return"
  %"extract-cons:return5" = extractvalue { i32, ptr } %"load car", 1
  %"extract-cons:6" = load { ptr, ptr }, ptr %"extract-cons:return5", align 8
  %"get car7" = extractvalue { ptr, ptr } %"extract-cons:6", 0
  %"load car8" = load { i32, ptr }, ptr %"get car7", align 8
  %get_type10 = extractvalue { i32, ptr } %"load car8", 0
  %"extract-cons:cmp-type11" = icmp eq i32 %get_type10, 6
  br i1 %"extract-cons:cmp-type11", label %"extract-cons:return9", label %error

"extract-cons:return9":                           ; preds = %"extract-cons:return2"
  %"extract-cons:return12" = extractvalue { i32, ptr } %"load car8", 1
  %"extract-cons:13" = load { ptr, ptr }, ptr %"extract-cons:return12", align 8
  %"get cdr" = extractvalue { ptr, ptr } %"extract-cons:13", 1
  %"load cdr" = load { i32, ptr }, ptr %"get cdr", align 8
  %get_type15 = extractvalue { i32, ptr } %"load cdr", 0
  %"extract-cons:cmp-type16" = icmp eq i32 %get_type15, 6
  br i1 %"extract-cons:cmp-type16", label %"extract-cons:return14", label %error

"extract-cons:return14":                          ; preds = %"extract-cons:return9"
  %"extract-cons:return17" = extractvalue { i32, ptr } %"load cdr", 1
  %"extract-cons:18" = load { ptr, ptr }, ptr %"extract-cons:return17", align 8
  %"get car19" = extractvalue { ptr, ptr } %"extract-cons:18", 0
  %"load car20" = load { i32, ptr }, ptr %"get car19", align 8
  %get_type22 = extractvalue { i32, ptr } %"load car20", 0
  %"extract-cons:cmp-type23" = icmp eq i32 %get_type22, 6
  br i1 %"extract-cons:cmp-type23", label %"extract-cons:return21", label %error

"extract-cons:return21":                          ; preds = %"extract-cons:return14"
  %"extract-cons:return24" = extractvalue { i32, ptr } %"load car20", 1
  %"extract-cons:25" = load { ptr, ptr }, ptr %"extract-cons:return24", align 8
  %"get cdr26" = extractvalue { ptr, ptr } %"extract-cons:25", 1
  %"load cdr27" = load { i32, ptr }, ptr %"get cdr26", align 8
  ret { i32, ptr } %"load cdr27"
}

define { i32, ptr } @cdaddr({ i32, ptr } %0) {
entry:
  %get_type = extractvalue { i32, ptr } %0, 0
  %"extract-cons:cmp-type" = icmp eq i32 %get_type, 6
  br i1 %"extract-cons:cmp-type", label %"extract-cons:return", label %error

error:                                            ; preds = %"extract-cons:return14", %"extract-cons:return7", %"extract-cons:return2", %"extract-cons:return", %entry
  %"error phi" = phi { ptr, i32 } [ { ptr @"error exit.142", i32 1 }, %entry ], [ { ptr @"error exit.143", i32 1 }, %"extract-cons:return" ], [ { ptr @"error exit.144", i32 1 }, %"extract-cons:return2" ], [ { ptr @"error exit.145", i32 1 }, %"extract-cons:return7" ], [ { ptr @"error exit.146", i32 1 }, %"extract-cons:return14" ]
  %error_msg = extractvalue { ptr, i32 } %"error phi", 0
  %error_code = extractvalue { ptr, i32 } %"error phi", 1
  %print = call i32 (ptr, ...) @printf(ptr %error_msg)
  call void @exit(i32 %error_code)
  unreachable

"extract-cons:return":                            ; preds = %entry
  %"extract-cons:return1" = extractvalue { i32, ptr } %0, 1
  %"extract-cons:" = load { ptr, ptr }, ptr %"extract-cons:return1", align 8
  %"get car" = extractvalue { ptr, ptr } %"extract-cons:", 0
  %"load car" = load { i32, ptr }, ptr %"get car", align 8
  %get_type3 = extractvalue { i32, ptr } %"load car", 0
  %"extract-cons:cmp-type4" = icmp eq i32 %get_type3, 6
  br i1 %"extract-cons:cmp-type4", label %"extract-cons:return2", label %error

"extract-cons:return2":                           ; preds = %"extract-cons:return"
  %"extract-cons:return5" = extractvalue { i32, ptr } %"load car", 1
  %"extract-cons:6" = load { ptr, ptr }, ptr %"extract-cons:return5", align 8
  %"get cdr" = extractvalue { ptr, ptr } %"extract-cons:6", 1
  %"load cdr" = load { i32, ptr }, ptr %"get cdr", align 8
  %get_type8 = extractvalue { i32, ptr } %"load cdr", 0
  %"extract-cons:cmp-type9" = icmp eq i32 %get_type8, 6
  br i1 %"extract-cons:cmp-type9", label %"extract-cons:return7", label %error

"extract-cons:return7":                           ; preds = %"extract-cons:return2"
  %"extract-cons:return10" = extractvalue { i32, ptr } %"load cdr", 1
  %"extract-cons:11" = load { ptr, ptr }, ptr %"extract-cons:return10", align 8
  %"get cdr12" = extractvalue { ptr, ptr } %"extract-cons:11", 1
  %"load cdr13" = load { i32, ptr }, ptr %"get cdr12", align 8
  %get_type15 = extractvalue { i32, ptr } %"load cdr13", 0
  %"extract-cons:cmp-type16" = icmp eq i32 %get_type15, 6
  br i1 %"extract-cons:cmp-type16", label %"extract-cons:return14", label %error

"extract-cons:return14":                          ; preds = %"extract-cons:return7"
  %"extract-cons:return17" = extractvalue { i32, ptr } %"load cdr13", 1
  %"extract-cons:18" = load { ptr, ptr }, ptr %"extract-cons:return17", align 8
  %"get car19" = extractvalue { ptr, ptr } %"extract-cons:18", 0
  %"load car20" = load { i32, ptr }, ptr %"get car19", align 8
  %get_type22 = extractvalue { i32, ptr } %"load car20", 0
  %"extract-cons:cmp-type23" = icmp eq i32 %get_type22, 6
  br i1 %"extract-cons:cmp-type23", label %"extract-cons:return21", label %error

"extract-cons:return21":                          ; preds = %"extract-cons:return14"
  %"extract-cons:return24" = extractvalue { i32, ptr } %"load car20", 1
  %"extract-cons:25" = load { ptr, ptr }, ptr %"extract-cons:return24", align 8
  %"get cdr26" = extractvalue { ptr, ptr } %"extract-cons:25", 1
  %"load cdr27" = load { i32, ptr }, ptr %"get cdr26", align 8
  ret { i32, ptr } %"load cdr27"
}

define { i32, ptr } @cddaar({ i32, ptr } %0) {
entry:
  %get_type = extractvalue { i32, ptr } %0, 0
  %"extract-cons:cmp-type" = icmp eq i32 %get_type, 6
  br i1 %"extract-cons:cmp-type", label %"extract-cons:return", label %error

error:                                            ; preds = %"extract-cons:return16", %"extract-cons:return9", %"extract-cons:return2", %"extract-cons:return", %entry
  %"error phi" = phi { ptr, i32 } [ { ptr @"error exit.147", i32 1 }, %entry ], [ { ptr @"error exit.148", i32 1 }, %"extract-cons:return" ], [ { ptr @"error exit.149", i32 1 }, %"extract-cons:return2" ], [ { ptr @"error exit.150", i32 1 }, %"extract-cons:return9" ], [ { ptr @"error exit.151", i32 1 }, %"extract-cons:return16" ]
  %error_msg = extractvalue { ptr, i32 } %"error phi", 0
  %error_code = extractvalue { ptr, i32 } %"error phi", 1
  %print = call i32 (ptr, ...) @printf(ptr %error_msg)
  call void @exit(i32 %error_code)
  unreachable

"extract-cons:return":                            ; preds = %entry
  %"extract-cons:return1" = extractvalue { i32, ptr } %0, 1
  %"extract-cons:" = load { ptr, ptr }, ptr %"extract-cons:return1", align 8
  %"get car" = extractvalue { ptr, ptr } %"extract-cons:", 0
  %"load car" = load { i32, ptr }, ptr %"get car", align 8
  %get_type3 = extractvalue { i32, ptr } %"load car", 0
  %"extract-cons:cmp-type4" = icmp eq i32 %get_type3, 6
  br i1 %"extract-cons:cmp-type4", label %"extract-cons:return2", label %error

"extract-cons:return2":                           ; preds = %"extract-cons:return"
  %"extract-cons:return5" = extractvalue { i32, ptr } %"load car", 1
  %"extract-cons:6" = load { ptr, ptr }, ptr %"extract-cons:return5", align 8
  %"get car7" = extractvalue { ptr, ptr } %"extract-cons:6", 0
  %"load car8" = load { i32, ptr }, ptr %"get car7", align 8
  %get_type10 = extractvalue { i32, ptr } %"load car8", 0
  %"extract-cons:cmp-type11" = icmp eq i32 %get_type10, 6
  br i1 %"extract-cons:cmp-type11", label %"extract-cons:return9", label %error

"extract-cons:return9":                           ; preds = %"extract-cons:return2"
  %"extract-cons:return12" = extractvalue { i32, ptr } %"load car8", 1
  %"extract-cons:13" = load { ptr, ptr }, ptr %"extract-cons:return12", align 8
  %"get car14" = extractvalue { ptr, ptr } %"extract-cons:13", 0
  %"load car15" = load { i32, ptr }, ptr %"get car14", align 8
  %get_type17 = extractvalue { i32, ptr } %"load car15", 0
  %"extract-cons:cmp-type18" = icmp eq i32 %get_type17, 6
  br i1 %"extract-cons:cmp-type18", label %"extract-cons:return16", label %error

"extract-cons:return16":                          ; preds = %"extract-cons:return9"
  %"extract-cons:return19" = extractvalue { i32, ptr } %"load car15", 1
  %"extract-cons:20" = load { ptr, ptr }, ptr %"extract-cons:return19", align 8
  %"get cdr" = extractvalue { ptr, ptr } %"extract-cons:20", 1
  %"load cdr" = load { i32, ptr }, ptr %"get cdr", align 8
  %get_type22 = extractvalue { i32, ptr } %"load cdr", 0
  %"extract-cons:cmp-type23" = icmp eq i32 %get_type22, 6
  br i1 %"extract-cons:cmp-type23", label %"extract-cons:return21", label %error

"extract-cons:return21":                          ; preds = %"extract-cons:return16"
  %"extract-cons:return24" = extractvalue { i32, ptr } %"load cdr", 1
  %"extract-cons:25" = load { ptr, ptr }, ptr %"extract-cons:return24", align 8
  %"get cdr26" = extractvalue { ptr, ptr } %"extract-cons:25", 1
  %"load cdr27" = load { i32, ptr }, ptr %"get cdr26", align 8
  ret { i32, ptr } %"load cdr27"
}

define { i32, ptr } @cddadr({ i32, ptr } %0) {
entry:
  %get_type = extractvalue { i32, ptr } %0, 0
  %"extract-cons:cmp-type" = icmp eq i32 %get_type, 6
  br i1 %"extract-cons:cmp-type", label %"extract-cons:return", label %error

error:                                            ; preds = %"extract-cons:return14", %"extract-cons:return7", %"extract-cons:return2", %"extract-cons:return", %entry
  %"error phi" = phi { ptr, i32 } [ { ptr @"error exit.152", i32 1 }, %entry ], [ { ptr @"error exit.153", i32 1 }, %"extract-cons:return" ], [ { ptr @"error exit.154", i32 1 }, %"extract-cons:return2" ], [ { ptr @"error exit.155", i32 1 }, %"extract-cons:return7" ], [ { ptr @"error exit.156", i32 1 }, %"extract-cons:return14" ]
  %error_msg = extractvalue { ptr, i32 } %"error phi", 0
  %error_code = extractvalue { ptr, i32 } %"error phi", 1
  %print = call i32 (ptr, ...) @printf(ptr %error_msg)
  call void @exit(i32 %error_code)
  unreachable

"extract-cons:return":                            ; preds = %entry
  %"extract-cons:return1" = extractvalue { i32, ptr } %0, 1
  %"extract-cons:" = load { ptr, ptr }, ptr %"extract-cons:return1", align 8
  %"get car" = extractvalue { ptr, ptr } %"extract-cons:", 0
  %"load car" = load { i32, ptr }, ptr %"get car", align 8
  %get_type3 = extractvalue { i32, ptr } %"load car", 0
  %"extract-cons:cmp-type4" = icmp eq i32 %get_type3, 6
  br i1 %"extract-cons:cmp-type4", label %"extract-cons:return2", label %error

"extract-cons:return2":                           ; preds = %"extract-cons:return"
  %"extract-cons:return5" = extractvalue { i32, ptr } %"load car", 1
  %"extract-cons:6" = load { ptr, ptr }, ptr %"extract-cons:return5", align 8
  %"get cdr" = extractvalue { ptr, ptr } %"extract-cons:6", 1
  %"load cdr" = load { i32, ptr }, ptr %"get cdr", align 8
  %get_type8 = extractvalue { i32, ptr } %"load cdr", 0
  %"extract-cons:cmp-type9" = icmp eq i32 %get_type8, 6
  br i1 %"extract-cons:cmp-type9", label %"extract-cons:return7", label %error

"extract-cons:return7":                           ; preds = %"extract-cons:return2"
  %"extract-cons:return10" = extractvalue { i32, ptr } %"load cdr", 1
  %"extract-cons:11" = load { ptr, ptr }, ptr %"extract-cons:return10", align 8
  %"get car12" = extractvalue { ptr, ptr } %"extract-cons:11", 0
  %"load car13" = load { i32, ptr }, ptr %"get car12", align 8
  %get_type15 = extractvalue { i32, ptr } %"load car13", 0
  %"extract-cons:cmp-type16" = icmp eq i32 %get_type15, 6
  br i1 %"extract-cons:cmp-type16", label %"extract-cons:return14", label %error

"extract-cons:return14":                          ; preds = %"extract-cons:return7"
  %"extract-cons:return17" = extractvalue { i32, ptr } %"load car13", 1
  %"extract-cons:18" = load { ptr, ptr }, ptr %"extract-cons:return17", align 8
  %"get cdr19" = extractvalue { ptr, ptr } %"extract-cons:18", 1
  %"load cdr20" = load { i32, ptr }, ptr %"get cdr19", align 8
  %get_type22 = extractvalue { i32, ptr } %"load cdr20", 0
  %"extract-cons:cmp-type23" = icmp eq i32 %get_type22, 6
  br i1 %"extract-cons:cmp-type23", label %"extract-cons:return21", label %error

"extract-cons:return21":                          ; preds = %"extract-cons:return14"
  %"extract-cons:return24" = extractvalue { i32, ptr } %"load cdr20", 1
  %"extract-cons:25" = load { ptr, ptr }, ptr %"extract-cons:return24", align 8
  %"get cdr26" = extractvalue { ptr, ptr } %"extract-cons:25", 1
  %"load cdr27" = load { i32, ptr }, ptr %"get cdr26", align 8
  ret { i32, ptr } %"load cdr27"
}

define { i32, ptr } @cdddar({ i32, ptr } %0) {
entry:
  %get_type = extractvalue { i32, ptr } %0, 0
  %"extract-cons:cmp-type" = icmp eq i32 %get_type, 6
  br i1 %"extract-cons:cmp-type", label %"extract-cons:return", label %error

error:                                            ; preds = %"extract-cons:return14", %"extract-cons:return9", %"extract-cons:return2", %"extract-cons:return", %entry
  %"error phi" = phi { ptr, i32 } [ { ptr @"error exit.157", i32 1 }, %entry ], [ { ptr @"error exit.158", i32 1 }, %"extract-cons:return" ], [ { ptr @"error exit.159", i32 1 }, %"extract-cons:return2" ], [ { ptr @"error exit.160", i32 1 }, %"extract-cons:return9" ], [ { ptr @"error exit.161", i32 1 }, %"extract-cons:return14" ]
  %error_msg = extractvalue { ptr, i32 } %"error phi", 0
  %error_code = extractvalue { ptr, i32 } %"error phi", 1
  %print = call i32 (ptr, ...) @printf(ptr %error_msg)
  call void @exit(i32 %error_code)
  unreachable

"extract-cons:return":                            ; preds = %entry
  %"extract-cons:return1" = extractvalue { i32, ptr } %0, 1
  %"extract-cons:" = load { ptr, ptr }, ptr %"extract-cons:return1", align 8
  %"get car" = extractvalue { ptr, ptr } %"extract-cons:", 0
  %"load car" = load { i32, ptr }, ptr %"get car", align 8
  %get_type3 = extractvalue { i32, ptr } %"load car", 0
  %"extract-cons:cmp-type4" = icmp eq i32 %get_type3, 6
  br i1 %"extract-cons:cmp-type4", label %"extract-cons:return2", label %error

"extract-cons:return2":                           ; preds = %"extract-cons:return"
  %"extract-cons:return5" = extractvalue { i32, ptr } %"load car", 1
  %"extract-cons:6" = load { ptr, ptr }, ptr %"extract-cons:return5", align 8
  %"get car7" = extractvalue { ptr, ptr } %"extract-cons:6", 0
  %"load car8" = load { i32, ptr }, ptr %"get car7", align 8
  %get_type10 = extractvalue { i32, ptr } %"load car8", 0
  %"extract-cons:cmp-type11" = icmp eq i32 %get_type10, 6
  br i1 %"extract-cons:cmp-type11", label %"extract-cons:return9", label %error

"extract-cons:return9":                           ; preds = %"extract-cons:return2"
  %"extract-cons:return12" = extractvalue { i32, ptr } %"load car8", 1
  %"extract-cons:13" = load { ptr, ptr }, ptr %"extract-cons:return12", align 8
  %"get cdr" = extractvalue { ptr, ptr } %"extract-cons:13", 1
  %"load cdr" = load { i32, ptr }, ptr %"get cdr", align 8
  %get_type15 = extractvalue { i32, ptr } %"load cdr", 0
  %"extract-cons:cmp-type16" = icmp eq i32 %get_type15, 6
  br i1 %"extract-cons:cmp-type16", label %"extract-cons:return14", label %error

"extract-cons:return14":                          ; preds = %"extract-cons:return9"
  %"extract-cons:return17" = extractvalue { i32, ptr } %"load cdr", 1
  %"extract-cons:18" = load { ptr, ptr }, ptr %"extract-cons:return17", align 8
  %"get cdr19" = extractvalue { ptr, ptr } %"extract-cons:18", 1
  %"load cdr20" = load { i32, ptr }, ptr %"get cdr19", align 8
  %get_type22 = extractvalue { i32, ptr } %"load cdr20", 0
  %"extract-cons:cmp-type23" = icmp eq i32 %get_type22, 6
  br i1 %"extract-cons:cmp-type23", label %"extract-cons:return21", label %error

"extract-cons:return21":                          ; preds = %"extract-cons:return14"
  %"extract-cons:return24" = extractvalue { i32, ptr } %"load cdr20", 1
  %"extract-cons:25" = load { ptr, ptr }, ptr %"extract-cons:return24", align 8
  %"get cdr26" = extractvalue { ptr, ptr } %"extract-cons:25", 1
  %"load cdr27" = load { i32, ptr }, ptr %"get cdr26", align 8
  ret { i32, ptr } %"load cdr27"
}

define { i32, ptr } @cddddr({ i32, ptr } %0) {
entry:
  %get_type = extractvalue { i32, ptr } %0, 0
  %"extract-cons:cmp-type" = icmp eq i32 %get_type, 6
  br i1 %"extract-cons:cmp-type", label %"extract-cons:return", label %error

error:                                            ; preds = %"extract-cons:return14", %"extract-cons:return7", %"extract-cons:return2", %"extract-cons:return", %entry
  %"error phi" = phi { ptr, i32 } [ { ptr @"error exit.162", i32 1 }, %entry ], [ { ptr @"error exit.163", i32 1 }, %"extract-cons:return" ], [ { ptr @"error exit.164", i32 1 }, %"extract-cons:return2" ], [ { ptr @"error exit.165", i32 1 }, %"extract-cons:return7" ], [ { ptr @"error exit.166", i32 1 }, %"extract-cons:return14" ]
  %error_msg = extractvalue { ptr, i32 } %"error phi", 0
  %error_code = extractvalue { ptr, i32 } %"error phi", 1
  %print = call i32 (ptr, ...) @printf(ptr %error_msg)
  call void @exit(i32 %error_code)
  unreachable

"extract-cons:return":                            ; preds = %entry
  %"extract-cons:return1" = extractvalue { i32, ptr } %0, 1
  %"extract-cons:" = load { ptr, ptr }, ptr %"extract-cons:return1", align 8
  %"get car" = extractvalue { ptr, ptr } %"extract-cons:", 0
  %"load car" = load { i32, ptr }, ptr %"get car", align 8
  %get_type3 = extractvalue { i32, ptr } %"load car", 0
  %"extract-cons:cmp-type4" = icmp eq i32 %get_type3, 6
  br i1 %"extract-cons:cmp-type4", label %"extract-cons:return2", label %error

"extract-cons:return2":                           ; preds = %"extract-cons:return"
  %"extract-cons:return5" = extractvalue { i32, ptr } %"load car", 1
  %"extract-cons:6" = load { ptr, ptr }, ptr %"extract-cons:return5", align 8
  %"get cdr" = extractvalue { ptr, ptr } %"extract-cons:6", 1
  %"load cdr" = load { i32, ptr }, ptr %"get cdr", align 8
  %get_type8 = extractvalue { i32, ptr } %"load cdr", 0
  %"extract-cons:cmp-type9" = icmp eq i32 %get_type8, 6
  br i1 %"extract-cons:cmp-type9", label %"extract-cons:return7", label %error

"extract-cons:return7":                           ; preds = %"extract-cons:return2"
  %"extract-cons:return10" = extractvalue { i32, ptr } %"load cdr", 1
  %"extract-cons:11" = load { ptr, ptr }, ptr %"extract-cons:return10", align 8
  %"get cdr12" = extractvalue { ptr, ptr } %"extract-cons:11", 1
  %"load cdr13" = load { i32, ptr }, ptr %"get cdr12", align 8
  %get_type15 = extractvalue { i32, ptr } %"load cdr13", 0
  %"extract-cons:cmp-type16" = icmp eq i32 %get_type15, 6
  br i1 %"extract-cons:cmp-type16", label %"extract-cons:return14", label %error

"extract-cons:return14":                          ; preds = %"extract-cons:return7"
  %"extract-cons:return17" = extractvalue { i32, ptr } %"load cdr13", 1
  %"extract-cons:18" = load { ptr, ptr }, ptr %"extract-cons:return17", align 8
  %"get cdr19" = extractvalue { ptr, ptr } %"extract-cons:18", 1
  %"load cdr20" = load { i32, ptr }, ptr %"get cdr19", align 8
  %get_type22 = extractvalue { i32, ptr } %"load cdr20", 0
  %"extract-cons:cmp-type23" = icmp eq i32 %get_type22, 6
  br i1 %"extract-cons:cmp-type23", label %"extract-cons:return21", label %error

"extract-cons:return21":                          ; preds = %"extract-cons:return14"
  %"extract-cons:return24" = extractvalue { i32, ptr } %"load cdr20", 1
  %"extract-cons:25" = load { ptr, ptr }, ptr %"extract-cons:return24", align 8
  %"get cdr26" = extractvalue { ptr, ptr } %"extract-cons:25", 1
  %"load cdr27" = load { i32, ptr }, ptr %"get cdr26", align 8
  ret { i32, ptr } %"load cdr27"
}

