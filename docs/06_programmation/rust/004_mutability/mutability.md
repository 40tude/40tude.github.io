---
published: true
layout: default
title: "Mutabilité du Binding Local, Mutabilité d'une Référence"
parent: "Rust"
#math: mathjax
date               : 2025-03-30 11:00:00
last_modified_date : 2025-03-30 11:00:00
---

# Mutabilité du Binding Local, Mutabilité d'une Référence


<div align="center">
<img src="./assets/img_00.webp" alt="" width="225" loading="lazy"/>
</div>

## Rustlings : move_semantics3.rs

Comme beaucoup de ceux qui débutent avec Rust, j'ai installé [Rustlings](https://github.com/rust-lang/rustlings) et voici l’énoncé de l'exercice ``move_semantics3.rs``. 

On nous demande de satisfaire le compilateur. OK... Regardons le code. Dans la section ``test`` on crée un vecteur `vec0` qu'on passe comme argument à une fonction ``fill_vec()``. Cette dernière retourne un vecteur ``vec1`` qui n'est autre que le précédent auquel on a ajouté la valeur 88. De son côté la fonction ``fill_vec()`` a un paramètre ``vec`` qui est un vecteur de i32 et elle retourne un vecteur de i32. Dans le corps de la fonction il y a un ``.push(88)`` qui modifie le contenu du vecteur.   

```rust
// TODO: Fix the compiler error in the function without adding any new line.
fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    vec.push(88);
    vec
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_semantics3() {
        let vec0 = vec![22, 44, 66];
        let vec1 = fill_vec(vec0);
        assert_eq!(vec1, [22, 44, 66, 88]);
    }
}
```
Voici la solution que je propose :

```rust
// TODO: Fix the compiler error in the function without adding any new line.
fn fill_vec(mut vec: Vec<i32>) -> Vec<i32> {
    //      ^^^----- Do you see the `mut` here
    vec.push(88);
    vec
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_semantics3() {
        let vec0 = vec![22, 44, 66];
        let vec1 = fill_vec(vec0);
        assert_eq!(vec1, [22, 44, 66, 88]);
    }
}
```
Dans la signature de la fonction ``fill_vec()`` j'ai juste rajouté un `mut` devant le paramètre ``vec``. Ok, super... Et? Hé bien maintenant il va falloir expliquer ce qui se passe et cela va nous permettre de revenir sur pas mal de sujets. 

## Dans la fonction main()

Je regarde la ligne

```rust
let vec0 = vec![22, 44, 66];
```

`vec0` est ce que l'on appelle en Rust, un **binding** non mutable sur un ``Vec<i32>``. 

*Hep, hep, hep. Tu peux reprendre, ça yest, tu m'as perdu... Je vois ce qu'est un vecteur de i32. C'est un tableau qui contient des entiers codés sur 32 bits susceptible de voir sa taille augmenter ou diminuer. Par contre binding... Pourquoi tu dis pas simplement qu'on déclare une variable ``vec0`` ?*

En fait, si on était dans un autre langage de programmation, C++ par exemple, oui on dirait que la ligne correspond à la déclaration de la variable ``vec0``. Après ça, j'expliquerai que, en gros, on associe un nom (`vec0`) à une valeur (ou un ensemble de valeurs ici). 

En Rust la notion de binding va peu plus loin :
1. Il y a toujours l'association d'un nom à une valeur (ou à un ensemble de valeurs) 
2. à qui on ajoute des notions de propriétés (et de prêt). 

Un binding c'est donc un engagement fort (un contrat) qu'on signe avec notre sang auprès du compilateur et ce dernier refusera de compiler notre code si on ne respecte pas notre parole. Comme tu le vois, en Rust l'ambiance est sympa, mais bon, c'est pour notre bien.

De plus, en Rust, par défaut tout est non mutable. Là où en C++ par défaut tout est mutable et où il faut que je précise si je veux qu'une variable soit constante :

```cpp

int       x = 42; // mutable par défaut
const int x = 42; // non mutable

```

En Rust, c'est le contraire :

```rust
let     x = 42;   // non mutable par défaut
let mut x = 42;   // mutable 

``` 

Du coup, si je reviens sur la ligne

```rust
let vec0 = vec![22, 44, 66];
```

`vec0` est bien un **binding** non mutable sur un ``Vec<i32>``. 

*Heu.. Attends... C'est le binding qui est non mutable? T'es sûr de ton coup?* 

Oui. La mutabilité est une propriété du binding, ce n'est pas une propriété des données (`[22, 44, 66]`) et encore moins du nom (`vec0`). 

Bouge pas, je t'explique et ça tombe bien car le type de données vecteur est intéressant. En effet, même si on le manipule comme une entité, dans les fait, ce type de données est constitué de 2 morceaux : 
1. il y a d'un côté une structure de contrôle (PLC) 
1. et de l'autre le jeu de données (22, 44, 66). 

**La structure de contrôle contient 3 champs :**
1. P = l'adresse où sont stockées en mémoire les données (22, 44, 66). C'est un pointeur.
1. L = la longueur actuelle du vecteur (3 ici par exemple)
1. C = la capacité du vecteur (10 par exemple). Si tel était le cas, le vecteur contiendrait 3 éléments de type i32 et il aurait la possibilité d'en recevoir 7 de plus avant de devoir être redimensionné.

**Le jeu de données :** 
* C'est une zone mémoire qui contient les 3 entiers 32 bits : [22, 44, 66]

De plus, les 2 constituants du type de données vecteur sont stockés dans 2 zones mémoire différentes :
1. La structure de contrôle (PLC) est sur la stack
1. Le jeu de données est sur le heap

Si tu veux tu peux imaginer qu'en mémoire la situation ressemble à ça :

<div align="center">
<img src="./assets/stack_heap.webp" alt="Rust stack heap" width="900" loading="lazy"/>
</div>

*Mais pourquoi c'est si compliqué?*

Imagine... Imagine qu'on est dans la fonction ``main()``, qu'on veut appeler une fonction et lui passer un paramètre. Faisons simple, et imaginons qu'on veut passer une valeur entière. Un peu comme dans [Le problème à 3 corps](https://www.youtube.com/watch?v=yEnW_1YdoS4) je te propose d'utiliser une machine très rustique...

<div align="center">
<img src="./assets/alu_3_bodies.webp" alt="ALU 3 bodies problem" width="900" loading="lazy"/>
</div>

Comment on fait ? Je te propose de mettre la valeur dans un classeur, de donner ce classeur à un cavalier et d'envoyer le cavalier à l'autre bout de la plaine. Là, on ouvre le classeur, on récupère la valeur et on execute le code de la fonction. Quand c'est terminé le cavalier revient, le classeur est vide car la fonction n'avait rien à renvoyer et on reprend l'execution de la fonction main().

Cool, ça marche. Maintenant si on veut passer 2 entiers. Même principe. Par contre attention à l'ordre. Faut que je me mette d'accord avec la fonction pour dire que la premiere feuille du classeur correspond au premier paramètre et la seconde au second paramètre.

Cool, ça marche encore... Et si maintenant je veux passer un nombre réel (3.14159) et un entier (42). Pareil, je fais attention à l'ordre et j'écris 3.14 sur une page et 42 sur l'autre. 

Cool, ça marche toujours. Imaginons maintenant que je veux passer un tableau de pixels (une image) dont la taille est connue à la compilation (640x480 pixels tous codés sur un octet). Là, c'est plus long mais je vais utiliser 640x480 pages et mettre sur chacune une valeur entre 0 et 255. À l'arrivée la fonction va lire toutes les pages du classeur et être capable de reconstituer l'image localement.

Bon ben voilà on a terminé. Mouai... Qu'est ce qui se passe maintenant si je veux passer un tableau de nombres dont je ne connais pas, au moment de la compilation, la longueur. Pense aussi aux cas où je souhaite passer un tableau dont la longueur est susceptible de varier au cours du temps. Ce que l'on appelle un [vecteur](https://youtu.be/KcPLUn91md0?si=6jIkvDKofiS-_Nvx).

<div align="center">
<img src="./assets/vector.webp" alt="vecteur" width="900" loading="lazy"/>
</div>

On est mort... C'est pas pôssible... En effet à l'arrivée, la fonction ne va pas savoir combien de pages elle doit dépiler (lire) du classeur. Cela dit, on peut s'en sortir si on applique le **principe d'indirection**. En gros, au lieu de passer le vecteur lui même on va pas passer la description de ce dernier. Elle, elle a une taille fixe. Par exemple on peut decider de décrire un vecteur avec 2 pages. Une page avec un entier qui indique le nombre de coordonnées et une autre page qui indique avec un autre entier, l'endroit où dans le champs, aller chercher les valeurs quand on en a besoin. Le truc c'est que tout se passe comme si on passait à la fonction un vecteur de taille variable mais cela se fait au prix d'une mise à disposition plus lente. En effet, au lieu de lire les valeurs du vecteur dans les pages du classeur, il va falloir faire faire à un cavalier des aller-retours à l'autre bout du champs pour rapatrier les valeurs dont on aura besoin.

Du coup il faut retenir que :
1. la Stack 
    * permet de stocker des variables locales
    * quand une fonction appelle une autre fonction en lui passant des paramètres
        * elle dépose ses dernier sur la Stack (push)
        * la fonction les recupère dans le bon ordre (pop)
    * on ne met dans la Stack que des paramètres dont la taille est connue et des type simples (int, bool, float, tableau fixe, tuple, adresses mémoire) 
1. le Heap, c'est une zone libre du champs où on peut déposer des trucs
    * c'est trucs (structures de données) peuvent avoir des tailles dynamiques
    * tous ceux, toutes les fonctions, qui savent où se trouve le truc (qui ont son adresse) peuvent y accéder

Du coup on comprend pourquoi le vecteur est composé en 2 morceaux

**La structure de contrôle :** elle a une taille fixe, connue au moment de la compilation. Si le vecteur `vec0` est mutable, le paramètre len va peut être passer de 3 à 18 mais en gros cette valeur sera toujours codée par un ``usize`` (pense à un entier 64 bits). De même, si pour une raison ou pour une autre on doit déplacer la zone qui contient les données (on passe de 3 à 300 données par exemple et on manque de place), l’adresse (la valeur du pointeur dont je parlais précédemment) va changer mais ce sera toujours une address sur 64 bits. Donc même si les valeurs des champs de la structure de contrôle changent, la taille, le nombre d'octets occupés par cette dernière sera toujours la même. 

C'est cette structure de taille fixe qu'on va faire passer, d'une fonction à une autre via la stack.

**Le jeu de données :**  
Il est susceptible de voir sa taille évoluer. On le stocke de manière naturelle sur le heap.


*Ok, ok je comprends pourquoi un type de données dynamique comme un vecteur est découpé en 2 parties (stack et heap) mais y sont où la stack et le heap?*

Dans le cadre d'un process (exécutable) qui tourne sous Windows voilà (à peu près) à quoi ressemble le plan mémoire (c'est similaire sous Linux, sous Mac, je sais pas).

```
+-------------------------+  ← Adresses hautes (ex: 0xFFFFFFFFFFFFFFFF)
|      Kernel Space       |  ← Réservé au système (non accessible)
+-------------------------+
|     Stack (croît ↓)     |  ← Variables locales, retour de fonctions
|                         |     Allouée dynamiquement à l'exécution
+-------------------------+
|   Guard page / padding  |  ← Protection contre débordement de pile
+-------------------------+
|        Heap             |  ← new / malloc : allouée dynamiquement
|  (croît vers le haut ↑) |
+-------------------------+
|   BSS Segment (.bss)    |  ← Variables globales NON initialisées
+-------------------------+
|   Data Segment (.data)  |  ← Variables globales initialisées
+-------------------------+
|   Code Segment (.text)  |  ← Code exécutable, fonctions
+-------------------------+
|     PE Headers (.exe)   |  ← Headers du fichier PE (Portable Executable)
+-------------------------+
|     NULL Page (invalide)|  ← Provoque un segfault en cas de déréférencement
+-------------------------+  ← Adresse 0x0000000000000000

```
Et si je simplifie encore, voilà ce qu'il faut retenir :

<div align="center">
<img src="./assets/virtual_memory_space.webp" alt="Rust stack heap" width="900" loading="lazy"/>
</div>

* le programme executable (le process) croît qu'il est seul au monde 
* ce benêt pense qu'il a accès à un espace mémoire 64 bits dont les adresses vont de 0x00.. à 0xFF.. En fait c'est l'OS qui lui fait croire ça, mais non, bien sûr, il est dans un espace mémoire virtualisé. 
* le code qui est exécuté se trouve dans la partie "Code Segment". 
* il y a ensuite 2 zones qui contiennent d'une part les variables globales initialisées et non initialisées.
* la taille du bloc mémoire ``[code + data + BSS]`` est fixe et connu à la fin de la compilation/édition de liens. Donc du coup, ça c'est bon, ça bouge pas.

Quand le programme démarre, le processeur exécute les instructions qui sont dans le segment ``.text``. Si il a besoin de la valeur de telle ou telle variable globale il va la chercher dans la zone ``DATA``. 

Ensuite, si le programme a besoin de créer une variable locale il ira le faire dans la stack (la pile) et si il a besoin d'allouer une zone mémoire il le fera dans le Heap (le tas). 

Pour fixer les idées, sous Windows, la taille de la stack du process principal c'est 1MB (4KB sont pre-alloués pour gagner du temps). C'est configurable si besoin. Ensuite chaque thread créé dispose de sa propre stack dont la taille par défaut est de 2MB (c'est configurable aussi).

Concernant le heap on va dire qu'au départ sa taille est de 0.

*Et qu'est ce qui se passe si la Stack qui croît vers le bas rencontre le heap qui croît vers le haut?* C'est un croisement d'effluves et tout le monde sait qu'il ne faut jamais croiser les effluves.

<div align="center">
<iframe width="560" height="315" src="https://www.youtube.com/embed/TAQnOzY7QXE?si=JlvEEWL3BTvWJvbz" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" referrerpolicy="strict-origin-when-cross-origin" allowfullscreen></iframe>
</div>















Donc du coup, à propos du binding en Rust il faut garder en tête :
1. Association d'un nom à une valeur (ou à un ensemble de valeurs) 
2. Compléter avec des notions de propriétés (et de prêt)
3. La mutabilité est une propriété du binding



## Dans la fonction fill_vec()


Voici le code de la solution avec les commentaires associés.

```rust
// ptr, len and capacity are moved (not cloned) from vec0 to vec_in
// On the heap, the data pointed to by ptr are not copied (and are mutable)
// The `mut` keyword allows the function to modify the local binding vec_in
// This is possible because fill_vec owns vec_in exclusively
// vec_in is a mutable binding to Vec<i32>, not a mutable Vec itself
// In the function signature, `mut` works just like it would in: let mut vec_in = ...
fn fill_vec(mut vec_in: Vec<i32>) -> Vec<i32> {
    vec_in.push(88); // the data are modified
    vec_in           // vec_in is moved to the caller
} 

// fn main() {
    // vec0 is an immutable binding to a Vec<i32>
    // A binding associates a name to a value + rules of ownership & borrowing
    // mutability is a property of the binding NOT a property of the value (nor the name)
    // The term binding in Rust represents a strong contract with the compiler, not just a “classic” variable.
    // Here, this means you cannot call vec0.push(...) or reassign vec0
    // However, the Vec internally holds a pointer to heap-allocated memory, which is mutable by nature
    // Rust allows the ownership of vec0 to be transferred (moved), even if the binding is not mutable

    let vec0 = vec![22, 44, 66]; // immutable binding: cannot change vec0 or call vec0.push(...)
                                 // but the heap memory behind it is mutable
                                 // On the heap, data pointed by ptr are mutable
    let vec1 = fill_vec(vec0);   // vec0 is moved into fill_vec
                                 // vec0 is no longer usable after this point

    assert_eq!(vec1, [22, 44, 66, 88]);
// }


```























## Coding Interview Patterns - Bonus p 5 - Shift 0 to the end

```rust

// the function receives a mutable reference to a Vec<i32> such that it can modify the data pointed to by ptr
// The binding nums_in is immutable, but it holds a mutable reference
// This means we can mutate the Vec it points to, but we cannot reassign nums_in itself
// nums_in cannot be reassigned to point to another Vec
// but the Vec it refers to can be mutated (e.g. via push, swap, etc.)
fn shift_zeros_to_the_end(nums_in: &mut Vec<i32>){ 
    let mut left = 0;
    for right in 0..nums_in.len(){
        if nums_in[right] != 0 {
            nums_in.swap(left, right);
            left += 1;
        }
    }
}

fn main(){
    let mut vec0 = vec![1, 0, 5, 0, 3, 12]; // vec0 is a mutable binding so it can be passed as &mut
    shift_zeros_to_the_end(&mut vec0);      // we pass a mutable reference to allow the function to mutate the Vec
    assert_eq!(vec0, [1, 5, 3, 12, 0, 0]);  // values have been rearranged in-place
}

```




















## variations around mutability

```rust

// This function takes a mutable reference to a String
// Because it owns a &mut String, it is allowed to modify the contents of the String on the heap
fn change(str_in: &mut String){
    str_in.push_str(", world"); // Appends text to the original String
}

// This function takes an immutable reference to a String
// It is only allowed to read from the String, not modify it
fn dont_change(str_in: &String){
    println!("{}", str_in); // Reads and prints the string
}

// This function takes an immutable reference to a string slice (&str)
// The binding is mutable, so we can reassign str_in to another slice,
// but we cannot modify the data pointed to by the slice
fn change_view(mut str_in: &str) {
    str_in = &str_in[1..3]; // Rebinds str_in to a substring of the original
    println!("{:?}", str_in); // Prints the new slice
}

fn main() {
    // Create a mutable String binding
    let mut my_str = String::from("hello");
    
    // Pass an immutable reference to a function that reads the string
    dont_change(&my_str);
    
    // Pass an immutable reference (as a slice) to a function that creates a view into the string
    change_view(&my_str);
    
    // Pass a mutable reference to allow the function to modify the String
    change(&mut my_str);

    // Print the modified String
    println!("{}", my_str); // Should print: hello, world
}


```


