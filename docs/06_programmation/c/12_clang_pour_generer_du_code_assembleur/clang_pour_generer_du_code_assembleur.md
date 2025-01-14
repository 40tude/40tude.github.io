---
layout: default
title: "Clang pour générer du code assembleur"
parent: "ANSI C"
#math: mathjax
date: 2013-12-25 23:16:23
last_modified_date: 2020-05-03 17:38:26
---

# Clang pour générer du code assembleur

Avec CVI 2013, si pour une raison ou pour une autre vous souhaitez traduire en assembleur un de vos codes source, ouvrez une console (touche Windows + R et vous tapez cmd) dans laquelle vous saisissez la commande suivante. Attention, ici on suppose que le code s'appelle ``wintest.c`` (voir à la fin de la ligne de commande)

Le résultat se retrouvera dans le fichier `compile.txt`

```
"C:\Program Files (x86)\National Instruments\CVI2013\bin\clang\2.9\clang.exe" -cc1 -triple i386-pc-win32 -S -nostdinc -triple i386-pc-win32 -fno-caret-diagnostics -fno-gnu-keywords -fms-extensions -mms-bitfields -fmath-errno -mdisable-fp-elim -Wno-microsoft -Werror=gnu -Werror=pointer-arith -Wpointer-sign -Wunreachable-code -Wparentheses -Wunused-variable -Wuninitialized -Wdiv-by-zero -Warray-bounds -Wnull-dereference -Wignored-qualifiers -Wreturn-type -Wpanel-handle -Wtautological-compare -Wempty-body -Wmissing-braces -Woverlength-strings -Wcast-align -Wmissing-noreturn -Winvalid-noreturn -Wshadow -Wswitch -Wswitch-enum -Wunused-function -Wunused-label -Wunused-parameter -Wunused-value -Wused-but-marked-unused -Wconversion -Wsign-compare -Wchar-subscripts -Wdeclaration-after-statement -Wtypecheck-convert -Wfloat-equal -Wvla -Wattributes -Wunknown-attributes -Wdeprecated-declarations -Wpacked -Wpadded -Wcomment -Wformat -Wimplicit-int -Wimplicit-function-declaration -Wbuiltin-implicit-decl -Wmissing-declarations -Wmissing-field-initializers -Wtrigraphs -Wmultichar -Wlong-long -Wunknown-pragmas -Wignored-pragmas -Wpragma-reset -Wpragma-on-off-switch -Wpragma-unused -Wend-of-directive -Wpragma-not-supported -Wunexpected-token -Wfour-char-constants -Wdiscard-qual -Wsign-conversion -Wvariadic-macros -Wmacro-name -Wmacro-poisoning -Wmacro-redefined -Wbuiltin-macro-redefined -Wbuiltin-macro-undefined -Wunused-macros -Wgnu-designator -Wnonnull -Wconstant-conversion -Wout-of-scope-declarations -Wstatic-non-static -Wconstant-too-large -W#warnings -Wundef -Winclude-next -Wextra-tokens -Wline-number -Wweak -Wmember-redeclare -Wexpected-semi -Wtoo-many-args -Wliteral-range -Wunknown-escape -Wshift -Wchar-init -Wbraces-init -Wincomplete-field -Wtentative -Wordered-comparison -Wpointer-compare -Wstack-memory -Wexcess-elements -Wmissing-terminating -Willegal-storage -Wclang -Wbackslash -Wdecl-param -Wnegative-to-unsigned -Wextern -Wconstant-logical-operand -Winitializer-overrides -Wvariadic-call-conv -Wmain -Womp-nested-parallel -Womp-repeated-ordered -Wpointer-int-conv -Wint-conversion -Wpointer-conv -fdiagnostics-show-option -cvi-debugging=extended -Werror=implicit-function-declaration -Wuninitialized-runtime -I "C:\Program Files (x86)\National Instruments\CVI2013\sdk\include" -I "C:\Program Files (x86)\National Instruments\CVI2013\include\ansi" -I "C:\Program Files (x86)\National Instruments\CVI2013\include" -D_CVI_ -DWIN32_LEAN_AND_MEAN wintest.c 2>compile.txt
```

Arrêtez de râler... On va simplifier les choses. Quoiqu'il en soit, si à la fin vous ne retrouvez pas de fichier ``wintest.s`` lisez le contenu du fichier ``compile.txt`` et essayez de retrouver vos petits.

Bon, cela dit, on peut faire plus court et plus simple. Par exemple on peut essayer la ligne suivante si on souhaite un fichier compile.txt un peu moins long :

```
"C:\Program Files (x86)\National Instruments\CVI2013\bin\clang\2.9\clang.exe" -cc1 -triple i386-pc-win32 -S -nostdinc -triple i386-pc-win32 -fno-caret-diagnostics -fno-gnu-keywords -fms-extensions -mms-bitfields -fmath-errno -mdisable-fp-elim -Wno-microsoft -Werror=gnu -Werror=pointer-arith -Wpointer-sign -Wunreachable-code -Wparentheses -Wunused-variable -Wuninitialized -Wdiv-by-zero -Warray-bounds -Wnull-dereference -Wignored-qualifiers -Wreturn-type -Wpanel-handle -Wtautological-compare -Wempty-body -Wmissing-braces -Woverlength-strings -Wcast-align -Wmissing-noreturn -Winvalid-noreturn -Wshadow -Wswitch -Wswitch-enum -Wunused-function -Wunused-label -Wunused-parameter -Wunused-value -Wused-but-marked-unused -Wconversion -Wsign-compare -Wchar-subscripts -Wdeclaration-after-statement -Wtypecheck-convert -Wfloat-equal -Wattributes -Wunknown-attributes -Wdeprecated-declarations -Wpacked -Wcomment -Wformat -Wimplicit-int -Wimplicit-function-declaration -Wbuiltin-implicit-decl -Wmissing-declarations -Wmissing-field-initializers -Wtrigraphs -Wmultichar -Wlong-long -Wignored-pragmas -Wpragma-reset -Wpragma-on-off-switch -Wpragma-unused -Wend-of-directive -Wpragma-not-supported -Wunexpected-token -Wfour-char-constants -Wdiscard-qual -Wsign-conversion -Wvariadic-macros -Wmacro-name -Wmacro-poisoning -Wmacro-redefined -Wbuiltin-macro-redefined -Wbuiltin-macro-undefined -Wgnu-designator -Wnonnull -Wconstant-conversion -Wout-of-scope-declarations -Wstatic-non-static -Wconstant-too-large -W#warnings -Winclude-next -Wextra-tokens -Wline-number -Wweak -Wmember-redeclare -Wexpected-semi -Wtoo-many-args -Wliteral-range -Wunknown-escape -Wshift -Wchar-init -Wbraces-init -Wincomplete-field -Wtentative -Wordered-comparison -Wpointer-compare -Wstack-memory -Wexcess-elements -Wmissing-terminating -Willegal-storage -Wclang -Wbackslash -Wdecl-param -Wnegative-to-unsigned -Wextern -Wconstant-logical-operand -Winitializer-overrides -Wvariadic-call-conv -Wmain -Womp-nested-parallel -Womp-repeated-ordered -Wpointer-int-conv -Wint-conversion -Wpointer-conv -fdiagnostics-show-option -cvi-debugging=extended -Werror=implicit-function-declaration -Wuninitialized-runtime -I "C:\Program Files (x86)\National Instruments\CVI2013\sdk\include" -I "C:\Program Files (x86)\National Instruments\CVI2013\include\ansi" -I "C:\Program Files (x86)\National Instruments\CVI2013\include" -D_CVI_ -DWIN32_LEAN_AND_MEAN wintest.c 2>compile.txt`
```

En fait, dans la ligne ci-dessus j'ai enlevé les warnings suivants :

```
-Wundef
-Wunused-macros
-Wunknown-pragmas
-Wvla
-Wpadded`
```

Allez, on continue... On peut aller encore plus loin et avoir une ligne de commande beaucoup plus courte si on essaie la chose suivante :

```powershell
"C:\Program Files (x86)\National Instruments\CVI2013\bin\clang\2.9\clang.exe" -cc1 -triple i386-pc-win32 -S -nostdinc -fno-gnu-keywords -fms-extensions -mms-bitfields -fmath-errno -mdisable-fp-elim -Wall -I "C:\Program Files (x86)\National Instruments\CVI2013\sdk\include" -I "C:\Program Files (x86)\National Instruments\CVI2013\include\ansi" -I "C:\Program Files (x86)\National Instruments\CVI2013\include" -D_CVI_ -DWIN32_LEAN_AND_MEAN wintest.c 2>compile.txt`
```

Ci-dessus j'ai mis ``-Wall`` et j'ai enlevé le ``-fno-caret-diagnostics`` ainsi que tous les autres warnings. Enlever le ``-fno-caret-diagnostics`` permet d'avoir plus d'indices dans les explications qu'on retrouve dans le fichier compile.txt.

