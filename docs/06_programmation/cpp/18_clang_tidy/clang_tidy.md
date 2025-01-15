---
layout: default
title: "clang-tidy"
parent: "C++"
#math: mathjax
date: 2017-02-23 08:26:15
last_modified_date: 2020-05-03 14:24:12
---

# clang-tidy

Ne cherchez plus ``clang-modernize`` il est dorénavant inclus dans ``clang-tidy``.

Je suis sous Windows 10, dans une console classique ou une console Powershell. Pour une première invocation, on peut essayer :

```bash
clang-tidy raspicam.cpp -checks=* -- -std=c++11
```

On peut aussi utiliser des expressions régulières pour nommer les fichiers

```
clang-tidy test.[ch]pp -checks=* -- -std=c++11
```

Pour voir la liste des vérifications faites

```bash
clang-tidy raspicam.cpp -list-checks -checks=* -- -std=c++11
```

On voit alors (en date de Fev 2017) la liste suivante :

```
Enabled checks:
    boost-use-to-string
    cert-dcl03-c
    cert-dcl50-cpp
    cert-dcl54-cpp
    cert-dcl59-cpp
    cert-env33-c
    cert-err34-c
    cert-err52-cpp
    cert-err58-cpp
    cert-err60-cpp
    cert-err61-cpp
    cert-fio38-c
    cert-flp30-c
    cert-oop11-cpp
    clang-analyzer-alpha.core.BoolAssignment
    clang-analyzer-alpha.core.CallAndMessageUnInitRefArg
    clang-analyzer-alpha.core.CastSize
    clang-analyzer-alpha.core.CastToStruct
    clang-analyzer-alpha.core.DynamicTypeChecker
    clang-analyzer-alpha.core.FixedAddr
    clang-analyzer-alpha.core.IdenticalExpr
    clang-analyzer-alpha.core.PointerArithm
    clang-analyzer-alpha.core.PointerSub
    clang-analyzer-alpha.core.SizeofPtr
    clang-analyzer-alpha.core.TestAfterDivZero
    clang-analyzer-alpha.cplusplus.VirtualCall
    clang-analyzer-alpha.deadcode.UnreachableCode
    clang-analyzer-alpha.osx.cocoa.DirectIvarAssignment
    clang-analyzer-alpha.osx.cocoa.DirectIvarAssignmentForAnnotatedFunctions
    clang-analyzer-alpha.osx.cocoa.InstanceVariableInvalidation
    clang-analyzer-alpha.osx.cocoa.MissingInvalidationMethod
    clang-analyzer-alpha.osx.cocoa.localizability.PluralMisuseChecker
    clang-analyzer-alpha.security.ArrayBound
    clang-analyzer-alpha.security.ArrayBoundV2
    clang-analyzer-alpha.security.MallocOverflow
    clang-analyzer-alpha.security.ReturnPtrRange
    clang-analyzer-alpha.security.taint.TaintPropagation
    clang-analyzer-alpha.unix.Chroot
    clang-analyzer-alpha.unix.PthreadLock
    clang-analyzer-alpha.unix.SimpleStream
    clang-analyzer-alpha.unix.Stream
    clang-analyzer-alpha.unix.cstring.BufferOverlap
    clang-analyzer-alpha.unix.cstring.NotNullTerminated
    clang-analyzer-alpha.unix.cstring.OutOfBounds
    clang-analyzer-core.CallAndMessage
    clang-analyzer-core.DivideZero
    clang-analyzer-core.DynamicTypePropagation
    clang-analyzer-core.NonNullParamChecker
    clang-analyzer-core.NullDereference
    clang-analyzer-core.StackAddressEscape
    clang-analyzer-core.UndefinedBinaryOperatorResult
    clang-analyzer-core.VLASize
    clang-analyzer-core.builtin.BuiltinFunctions
    clang-analyzer-core.builtin.NoReturnFunctions
    clang-analyzer-core.uninitialized.ArraySubscript
    clang-analyzer-core.uninitialized.Assign
    clang-analyzer-core.uninitialized.Branch
    clang-analyzer-core.uninitialized.CapturedBlockVariable
    clang-analyzer-core.uninitialized.UndefReturn
    clang-analyzer-cplusplus.NewDelete
    clang-analyzer-cplusplus.NewDeleteLeaks
    clang-analyzer-deadcode.DeadStores
    clang-analyzer-llvm.Conventions
    clang-analyzer-nullability.NullPassedToNonnull
    clang-analyzer-nullability.NullReturnedFromNonnull
    clang-analyzer-nullability.NullableDereferenced
    clang-analyzer-nullability.NullablePassedToNonnull
    clang-analyzer-nullability.NullablePassedToNonnull
    clang-analyzer-optin.mpi.MPI-Checker
    clang-analyzer-optin.osx.cocoa.localizability.EmptyLocalizationContextChecker
    clang-analyzer-optin.osx.cocoa.localizability.NonLocalizedStringChecker
    clang-analyzer-optin.performance.Padding
    clang-analyzer-osx.API
    clang-analyzer-osx.SecKeychainAPI
    clang-analyzer-osx.cocoa.AtSync
    clang-analyzer-osx.cocoa.ClassRelease
    clang-analyzer-osx.cocoa.Dealloc
    clang-analyzer-osx.cocoa.IncompatibleMethodTypes
    clang-analyzer-osx.cocoa.Loops
    clang-analyzer-osx.cocoa.MissingSuperCall
    clang-analyzer-osx.cocoa.NSAutoreleasePool
    clang-analyzer-osx.cocoa.NSError
    clang-analyzer-osx.cocoa.NilArg
    clang-analyzer-osx.cocoa.NonNilReturnValue
    clang-analyzer-osx.cocoa.ObjCGenerics
    clang-analyzer-osx.cocoa.RetainCount
    clang-analyzer-osx.cocoa.SelfInit
    clang-analyzer-osx.cocoa.SuperDealloc
    clang-analyzer-osx.cocoa.UnusedIvars
    clang-analyzer-osx.cocoa.VariadicMethodTypes
    clang-analyzer-osx.coreFoundation.CFError
    clang-analyzer-osx.coreFoundation.CFNumber
    clang-analyzer-osx.coreFoundation.CFRetainRelease
    clang-analyzer-osx.coreFoundation.containers.OutOfBounds
    clang-analyzer-osx.coreFoundation.containers.PointerSizedValues
    clang-analyzer-security.FloatLoopCounter
    clang-analyzer-security.insecureAPI.UncheckedReturn
    clang-analyzer-security.insecureAPI.getpw
    clang-analyzer-security.insecureAPI.gets
    clang-analyzer-security.insecureAPI.mkstemp
    clang-analyzer-security.insecureAPI.mktemp
    clang-analyzer-security.insecureAPI.rand
    clang-analyzer-security.insecureAPI.strcpy
    clang-analyzer-security.insecureAPI.vfork
    clang-analyzer-unix.API
    clang-analyzer-unix.Malloc
    clang-analyzer-unix.MallocSizeof
    clang-analyzer-unix.MismatchedDeallocator
    clang-analyzer-unix.Vfork
    clang-analyzer-unix.cstring.BadSizeArg
    clang-analyzer-unix.cstring.NullArg
    cppcoreguidelines-c-copy-assignment-signature
    cppcoreguidelines-interfaces-global-init
    cppcoreguidelines-pro-bounds-array-to-pointer-decay
    cppcoreguidelines-pro-bounds-constant-array-index
    cppcoreguidelines-pro-bounds-pointer-arithmetic
    cppcoreguidelines-pro-type-const-cast
    cppcoreguidelines-pro-type-cstyle-cast
    cppcoreguidelines-pro-type-member-init
    cppcoreguidelines-pro-type-reinterpret-cast
    cppcoreguidelines-pro-type-static-cast-downcast
    cppcoreguidelines-pro-type-union-access
    cppcoreguidelines-pro-type-vararg
    google-build-explicit-make-pair
    google-build-namespaces
    google-build-using-namespace
    google-default-arguments
    google-explicit-constructor
    google-global-names-in-headers
    google-readability-braces-around-statements
    google-readability-casting
    google-readability-function-size
    google-readability-namespace-comments
    google-readability-redundant-smartptr-get
    google-readability-todo
    google-runtime-int
    google-runtime-member-string-references
    google-runtime-memset
    google-runtime-operator
    google-runtime-references
    llvm-header-guard
    llvm-include-order
    llvm-namespace-comment
    llvm-twine-local
    misc-argument-comment
    misc-assert-side-effect
    misc-bool-pointer-implicit-conversion
    misc-dangling-handle
    misc-definitions-in-headers
    misc-fold-init-type
    misc-forward-declaration-namespace
    misc-inaccurate-erase
    misc-incorrect-roundings
    misc-inefficient-algorithm
    misc-macro-parentheses
    misc-macro-repeated-side-effects
    misc-misplaced-const
    misc-misplaced-widening-cast
    misc-move-const-arg
    misc-move-constructor-init
    misc-multiple-statement-macro
    misc-new-delete-overloads
    misc-noexcept-move-constructor
    misc-non-copyable-objects
    misc-pointer-and-integral-operation
    misc-redundant-expression
    misc-sizeof-container
    misc-sizeof-expression
    misc-static-assert
    misc-string-constructor
    misc-string-integer-assignment
    misc-string-literal-with-embedded-nul
    misc-suspicious-missing-comma
    misc-suspicious-semicolon
    misc-suspicious-string-compare
    misc-swapped-arguments
    misc-throw-by-value-catch-by-reference
    misc-unconventional-assign-operator
    misc-undelegated-constructor
    misc-uniqueptr-reset-release
    misc-unused-alias-decls
    misc-unused-parameters
    misc-unused-raii
    misc-unused-using-decls
    misc-virtual-near-miss
    modernize-avoid-bind
    modernize-deprecated-headers
    modernize-loop-convert
    modernize-make-shared
    modernize-make-unique
    modernize-pass-by-value
    modernize-raw-string-literal
    modernize-redundant-void-arg
    modernize-replace-auto-ptr
    modernize-shrink-to-fit
    modernize-use-auto
    modernize-use-bool-literals
    modernize-use-default
    modernize-use-emplace
    modernize-use-nullptr
    modernize-use-override
    modernize-use-using
    performance-faster-string-find
    performance-for-range-copy
    performance-implicit-cast-in-loop
    performance-unnecessary-copy-initialization
    performance-unnecessary-value-param
    readability-avoid-const-params-in-decls
    readability-braces-around-statements
    readability-container-size-empty
    readability-deleted-default
    readability-else-after-return
    readability-function-size
    readability-identifier-naming
    readability-implicit-bool-cast
    readability-inconsistent-declaration-parameter-name
    readability-named-parameter
    readability-redundant-control-flow
    readability-redundant-smartptr-get
    readability-redundant-string-cstr
    readability-redundant-string-init
    readability-simplify-boolean-expr
    readability-static-definition-in-anonymous-namespace
    readability-uniqueptr-delete-release
```

On retrouve bien les vérifications de type "modernize-*"

Pour ne PAS utiliser les vérifications de type "clang-analyzer-alpha*" on peut invoquer

```bash
clang-tidy raspicam.cpp -list-checks -checks=*,-clang-analyzer-alpha.* -- -std=c++11
```

À ce stade on garde l'option ``-list-checks`` pour justement vérifier qu'on a plus les vérifications "clang-analyzer-alpha*".

{: .note }
Il ne faut **PAS** d'espaces entre ``-checks=*``, la virgule et ``-clang-analyzer-alpha.*`` (j'ai cherché un peu avant de comprendre).

Je suis content de ma liste de vérifications et des options. Je fais un dump de la config dans le fichier ``.clang-tidy`` (attention, c'est un fichier caché)

```bash
clang-tidy test.[ch]pp -dump-config -checks=*,-clang-analyzer-alpha.* -- -std=c++11 > .clang-tidy
```

J'invoque clang-tidy avec la liste des vérifications sur mes fichiers

```bash
clang-tidy raspicam.cpp -config='' -- -std=c++11
clang-tidy raspicam_test.cpp -config='' -- -std=c++11
```

### À lire :

* clang-tidy -help
* [L'aide en ligne](http://clang.llvm.org/extra/clang-tidy/)
* Aller lire cette [page](http://www.labri.fr/perso/fleury/posts/programming/using-clang-tidy-and-clang-format.html)

