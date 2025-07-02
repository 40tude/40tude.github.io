fn dereferencing01() {
    println!("\nDereferencing 01 : 101");

    let my_value = 5; // => my_value: i32
    println!("my_value : {}", my_value);

    let addr_of_my_value = &my_value; // => addr_of_my_value: &i32
    println!("addr_of_my_value : {:p}", addr_of_my_value);

    let content_at_addr_of_my_value = *addr_of_my_value; // content_at_addr_of_my_value => i32
    println!("content_at_addr_of_my_value : {}", content_at_addr_of_my_value);
}

fn dereferencing02_1() {
    println!("\nDereferencing 02_1 : mutabMutability of the referenced variableility\n");
    let my_value = 5; // immutable variable
    println!("my_value : {}", my_value);
    let ref_to_my_value = &my_value; // immutable reference to immutable variable
    println!("ref_to_my_value : {}", ref_to_my_value);
    println!();

    // *ref_to_my_value = 24; // => does not compile: `ref_to_my_value` is a `&` reference, so the data it refers to cannot be written

    let mut my_mutable_value = 55; // mutable variable
    println!("my_mutable_value : {}", my_mutable_value);
    let ref_to_my_mutable_value = &mut my_mutable_value; // mutable reference to mutable value
    println!("ref_to_my_mutable_value : {}", ref_to_my_mutable_value);
    println!();
    *ref_to_my_mutable_value += 1;
    println!("ref_to_my_mutable_value : {}", ref_to_my_mutable_value);
    println!("my_mutable_value : {}", my_mutable_value);
    println!();
}

fn dereferencing02_2() {
    println!("\nDereferencing 02_2 : Mutability of the reference\n");
    let my_value = 5; // immutable variable
    println!("my_value : {}", my_value);
    let other_value = 42;
    println!("other_value : {}", other_value);
    println!();
    let ref_to_my_value = &my_value; // immutable reference to immutable variable
    println!("ref_to_my_value : {}", ref_to_my_value);
    println!();

    // ref_to_my_value = &other_value; // => does not compile: cannot assign twice to immutable variable `ref_to_my_value`

    let ref_to_my_value = &other_value; // => shadowing. Does compile
    println!("ref_to_my_value : {}", ref_to_my_value); // => ref_to_my_value: &i32
    println!();
    let mut mut_ref_to_my_value = &my_value; // mutable reference to immutable variable
    println!("mut_ref_to_my_value : {}", mut_ref_to_my_value);
    mut_ref_to_my_value = &other_value; // mut_ref_to_my_value now reference other_value
    println!("mut_ref_to_my_value : {}", mut_ref_to_my_value);
    let other_value = std::f64::consts::PI; // => other_value: f64
    println!("other_value : {}", other_value);

    // mut_ref_to_my_value = &other_value; // => does not compile: expected `&{integer}`, found `&f64`
}

fn dereferencing03() {
    println!("\nDereferencing 03 : ref as argument\n");

    fn my_function01(v: Vec<i32>) {
        println!("{:?}", v);
    }

    fn my_function02(v: &Vec<i32>) {
        println!("{:?}", *v);
        println!("{:?}", v); // deref coercion in action
    }

    fn my_function03(v: &[i32]) {
        // accept reference to vectors or arrays
        // println!("{:?}", *v);    // Does not compile because *v is of type [i32] with no Sized trait (expected by println!)
        // Only references like `&[i32]` implement the `Debug` trait; `[i32]` alone doesn't, as it's dynamically sized)
        println!("{:?}", &*v); // Overkill ?
        println!("{:?}", v);
    }

    let my_vector = vec![42, 43, 44];
    my_function01(my_vector); // after the call my_vector disappears
                              // println!("{:?}", my_vector); // Does not compile

    let my_vector = vec![42, 43, 44]; // must recreate my_vector
    my_function02(&my_vector); // pass a reference
    my_function03(&my_vector);

    let my_array = [142, 143, 144]; // an array on the stack
    my_function03(&my_array);
}

fn dereferencing04() {
    println!("\nDereferencing 04 : Box, Rc and Deref\n");
    // Function that takes a value by reference
    fn print_ref(v: &i32) {
        println!("Value: {}", *v);
        println!("Value: {}", v);
    }
    // Function that takes a Box<i32>
    fn print_box(v: Box<i32>) {
        println!("Boxed value: {}", v);
    }
    // Create a value on the heap using Box
    let b = Box::new(123);
    println!("Address of the heap in the Box : {:p}", b);
    println!("Address of b on the stack      : {:p}", &b);
    println!("Dereferenced Box: {}", *b); // explicit deref
    println!("Dereferenced Box: {}", b); // works, thanks to deref coercion
                                         // The function expects &i32, but we give it &Box<i32>
                                         // Thanks to deref coercion, this works
    print_ref(&b);
    // Can also pass the Box directly if signature matches
    print_box(b); // b is moved here
}

fn dereferencing05() {
    println!("\nDereferencing 05 : Rc<T> and Reference Count\n");

    use std::rc::Rc;

    // Function that takes Rc<i32>
    fn print_rc(v: &Rc<i32>) {
        println!("From print_rc : {}", v);
    }

    // Create an Rc pointing to a value on the heap
    let rc1 = Rc::new(999);
    println!("Initial value: {}", rc1);
    println!("Address in Rc: {:p}", Rc::as_ptr(&rc1));
    println!("Reference count after creation: {}", Rc::strong_count(&rc1)); // 1
    print_rc(&rc1);

    // Create a clone of rc1 — this does not copy the value
    let rc2 = Rc::clone(&rc1);
    println!("\nAfter cloning rc1 into rc2:");
    println!("rc1 points to: {}", rc1);
    println!("rc2 points to: {}", rc2);
    println!("Reference count (rc1): {}", Rc::strong_count(&rc1)); // 2
    println!("Reference count (rc2): {}", Rc::strong_count(&rc2)); // 2

    {
        // Introduce a new scope
        let rc3 = Rc::clone(&rc2);
        println!("\nInside inner scope with rc3:");
        println!("rc3 points to: {}", rc3);
        println!("Reference count: {}", Rc::strong_count(&rc3)); // 3
    } // rc3 goes out of scope here

    println!("\nAfter rc3 is dropped:");
    println!("Reference count (rc1): {}", Rc::strong_count(&rc1)); // 2
}

fn dereferencing06() {
    println!("\nDereferencing 06 : Rc<RefCell<T>> for shared mutation (single-thread)\n");

    use std::cell::RefCell;
    use std::rc::Rc;

    // Rc enables multiple ownership, RefCell enables interior mutability
    let shared_vec = Rc::new(RefCell::new(vec![1, 2, 3]));
    println!("shared_vec: {:?}", shared_vec);

    println!("Reference count: {}", Rc::strong_count(&shared_vec));

    // Clone the Rc to get multiple owners
    let a = Rc::clone(&shared_vec);
    let b = Rc::clone(&shared_vec);
    println!("Reference count: {}", Rc::strong_count(&shared_vec)); // 3

    // Mutate the shared vector from owner `a`
    {
        let mut vec_ref = a.borrow_mut(); // borrow as mutable
        vec_ref.push(4);
        println!("a pushed 4: {:?}", vec_ref);
    }

    // Read from the shared vector via owner `b`
    {
        let vec_ref = b.borrow(); // borrow as immutable
        println!("b sees the vector: {:?}", vec_ref);
    }

    // Shows that the compiler doesn't see borrow conflicts, but the runtime does.
    {
        let _first = shared_vec.borrow_mut();
        // let _second = shared_vec.borrow_mut(); // panics at runtime
    }

    // Reference count stays at 3 until `a`, `b`, and `shared_vec` go out of scope
    println!("Reference count: {}", Rc::strong_count(&shared_vec)); // 3
}

//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//

fn destructuring01() {
    println!("\nDestructuring 01 : 101\n");

    let (x, y) = (1, 2); // (x, y) is a pattern
    println!("{x}, {y}");

    let (x, y) = (1, 3.14); // tuple => we can have different data type
    println!("{x}, {y}");

    let [a, b, c] = [10, 20, 30]; // [a, b, c] is a pattern
    println!("{a}, {b}, {c}");

    let x = 42; // `x` is a very simple pattern: it matches any value and binds it to the name `x`
    println!("{x}");

    let ((x1, y1), (x2, y2)) = ((1, 2), (3, 4)); // nested destructuring
    println!("{x1}, {y1}, {x2}, {y2}");
}

fn destructuring02() {
    println!("\nDestructuring 02 : partial and range destructuring\n");

    let (mut x, ..) = (41, 2, 3); // ignore the rest
    x += 1;
    println!("x: {x}");

    let (.., z) = (1, 2, 101); // ignore the rest
    println!("z: {z}");

    let age = 15;
    match age {
        1..=17 => println!("No way to access the dance floor."),
        _ => println!("Welcome to Studio 54!"),
    }

    let pair = ("Hari Seldon", 12050);
    let (_, just_the_year) = pair;
    println!("We only care about the year: {}", just_the_year);
}

fn destructuring02_bis() {
    println!("\nDestructuring 02 bis : partial and range destructuring\n");

    let mut data = (String::from("Obi-Wan"), 42);

    // This would move the String out of the tuple, making it unusable afterward:
    let (s, _) = data;
    println!("s: {}", s);
    // println!("data.0: {}", data.0); // does not compile

    let mut data = (String::from("Obi-Wan"), 42); // re-create data

    // Instead, we can use `ref` to borrow the String by reference
    let (ref s_ref, _) = data;
    println!("Using `ref`: {}", s_ref); // s_ref is a &String
    println!("Original still usable: {}", data.0); // data.0 is still valid

    // Now let's use `ref mut` to get a mutable reference to the String
    let (ref mut s_mut_ref, _) = data;
    s_mut_ref.push_str(" Kenobi");
    println!("Using `ref mut`: {}", s_mut_ref); // Modified through &mut String
    println!("Original mutated: {}", data.0); // Shows the mutation
}

// fn destructuring03() {
//     println!("\nDestructuring 03 : struct with let\n");

//     struct Point3D {
//         x: i32,
//         y: i32,
//         z: i32,
//         t: i32,
//     }
//     let pt = Point3D { x: 1, y: 2, z: 3, t: 2054 };

//     let Point3D { x, t, .. } = pt;
//     println!("x: {x} and time: {t}");

//     struct Scientist {
//         name: String,
//         field: String,
//     }

//     let hari = Scientist {
//         name: "Hari Seldon".to_string(),
//         field: "Psychohistory".to_string(),
//     };

//     let Scientist { name, field } = hari;
//     println!("{name} works in {field}");
// }

// fn destructuring03() {
//     println!("\nDestructuring 03 : struct with let\n");

//     struct Point3D {
//         x: i32,
//         y: i32,
//         z: i32,
//         t: i32,
//     }
//     let pt = Point3D { x: 1, y: 2, z: 3, t: 2054 };

//     let Point3D { x, t, .. } = pt;
//     println!("x: {x} and time: {t}");

// }

fn destructuring03() {
    println!("\nDestructuring 03 : struct of i32 with let");

    #[derive(Debug)]
    struct Point4D {
        x: i32,
        y: i32,
        z: i32,
        t: i32,
    }
    let pt = Point4D { x: 1, y: 2, z: 3, t: 2054 };

    let Point4D { x, t, .. } = pt; // copy
    println!("x: {x} and time: {t}");
    println!("{:?}", pt); // pt is available
}

fn destructuring03_bis() {
    println!("\nDestructuring 03 bis : struct of String with let");

    #[derive(Debug)]
    struct Person {
        last_name: String,
        first_name: String,
    }
    let luke = Person {
        last_name: "Skywalker".to_string(),
        first_name: "Luke".to_string(),
    };

    let Person { last_name, first_name } = luke; // move because String does'nt have Copy trait
    println!("{}-{}", last_name, first_name);
    // println!("{:?}", luke); // does not compile
}

// Does not compile. Require lifetime
// fn destructuring03_ter() {
//     println!("\nDestructuring 03 ter : struct of &str with let");

//     #[derive(Debug)]
//     struct Person<'t> {
//         last_name: &str,
//         first_name: &str,
//     }
//     let luke = Person {
//         last_name: "Skywalker",
//         first_name: "Luke"
//     };

//     let Person {last_name, first_name } = luke;
//     println!("{}-{}", last_name, first_name);
//     println!("{:?}", luke);
// }

fn destructuring03_ter() {
    println!("\nDestructuring 03 ter : struct of &str with let");

    #[derive(Debug)]
    struct Person<'t> {
        last_name: &'t str,
        first_name: &'t str,
    }
    let luke = Person {
        last_name: "Skywalker",
        first_name: "Luke",
    };

    let Person { last_name, first_name } = luke; // copy of &str
    println!("{}-{}", last_name, first_name);
    println!("{:p} - {:p}", last_name, first_name);

    println!("{:?}", luke); // does compile
    println!("{:p} - {:p}", luke.last_name, luke.first_name);
}

fn destructuring03_qua() {
    println!("\nDestructuring 03 qua : struct of &str with let");

    #[derive(Debug)]
    struct Person<'t> {
        last_name: &'t str,
        first_name: &'t str,
    }

    let luke = Person {
        last_name: "Skywalker",
        first_name: "Luke",
    };

    let &Person { last_name, first_name } = &luke;
    // The line above is similar to the 2 lines below
    // let ref = &luke;
    // let Person {last_name, first_name} = ref;
    println!("{} - {}", last_name, first_name);
    println!("{:p} - {:p}", last_name, first_name);

    println!("{:?}", luke);
    println!("{:p} - {:p}", luke.last_name, luke.first_name);
}

fn destructuring04() {
    println!("\nDestructuring 04 : enum with let\n");

    #[derive(Debug)]
    enum Role {
        Emperor,
        Trader(String),
        Scientist { name: String, field: String },
    }

    let characters = vec![
        Role::Emperor,
        Role::Trader("Hober Mallow".to_string()),
        Role::Scientist {
            name: "Hari Seldon".to_string(),
            field: "Psychohistory".to_string(),
        },
    ];

    for role in characters {
        // role is Role, characters is consumed
        match role {
            Role::Emperor => println!("The Emperor rules... vaguely."),
            Role::Trader(name) => println!("A trader named {name}"),
            Role::Scientist { name, field } => {
                println!("Scientist {name} specializes in {field}")
            }
        }
    }
    // println!("{:?}", characters); // does not compile

    let Some(x) = Some(5) else { todo!() }; // Some is an enum
    println!("x: {x}");
}

fn destructuring04_bis() {
    println!("\nDestructuring 04_bis : enum with let\n");

    #[derive(Debug)]
    enum Role {
        Emperor,
        Trader(String),
        Scientist { name: String, field: String },
    }

    let characters = vec![
        Role::Emperor,
        Role::Trader("Hober Mallow".to_string()),
        Role::Scientist {
            name: "Hari Seldon".to_string(),
            field: "Psychohistory".to_string(),
        },
    ];

    for role in &characters {
        // role is &Role, characters not consumed
        match role {
            Role::Emperor => println!("The Emperor rules... vaguely."),
            Role::Trader(name) => println!("A trader named {name}"),
            Role::Scientist { name, field } => {
                println!("Scientist {name} specializes in {field}")
            }
        }
    }
    println!("{:?}", characters);

    let Some(x) = Some(5) else { todo!() }; // Some is an enum
    println!("x: {x}");
}

fn destructuring05() {
    println!("\nDestructuring 05 : function & closure parameters\n");

    fn print_coordinates((x, y): (i32, i32)) {
        println!("Function received: x = {}, y = {}", x, y);
    }
    let point = (10, 20);
    print_coordinates(point);
    println!("{:?}", point);

    fn print_full_name((first, last): (String, String)) {
        println!("Function received: First = {}, Last = {}", first, last);
    }
    let chief = ("Martin".to_string(), "Brody".to_string());
    print_full_name(chief);
    // println!("{:?}", chief);// does not compile

    let points = vec![(1, 2), (3, 4), (5, 6)];
    points.iter().for_each(|&(x, y)| {
        println!("Point: x = {}, y = {}", x, y);
    });
    println!("Point: {:?}", points);
}

// In a for loop, the variable immediately after for is a pattern.
// That’s why we can destructure tuples directly inside the loop.”
fn destructuring06() {
    println!("\nDestructuring 06 : in for loops with .enumerate()\n");

    let characters = vec!["Hari", "Salvor", "Hober"];
    for (index, name) in characters.iter().enumerate() {
        println!("Character #{index} is {name}");
    }

    let characters = vec!["Harry".to_string(), "Hermione".to_string(), "Ron".to_string()];
    for (index, name) in characters.iter().enumerate() {
        println!("Character #{index} is {name}");
    }
}

fn destructuring06_bis() {
    println!("\nDestructuring 06 bis: in for loops with .enumerate()\n");

    let characters = vec!["Hari", "Salvor", "Hober"];
    for (index, name) in characters.iter().enumerate() {
        println!("Character #{index} is {name}");

        let bob = *name; // &str
        println!("{}", bob);

        // let bob = **name; // str, does not compile, no size known at compile-time
        // println!("{}", bob);

        // let bob = ***name; // ???, does not compile
        // println!("{}", bob);

        let bob = &name; // &&&str
        println!("{}", bob);
    }
}

// This line might look like we're referencing s, but &[x, y] is a pattern, not a reference. The compiler matches each &[i32; 2] and destructures it in-place
fn destructuring07() {
    println!("\nDestructuring 07 : for loop over array slices\n");

    let coordinates = vec![[1, 2], [3, 4], [5, 6]];
    for &[x, y] in &coordinates {
        println!("x: {}, y: {}", x, y);
    }
    // Alternative: without destructuring
    for coord in &coordinates {
        println!("coord[0]: {}, coord[1]: {}", coord[0], coord[1]);
    }
}

fn destructuring08() {
    println!("\nDereferencing 08 : destructuring pattern in for loop\n");

    let foundation: Vec<String> = vec!["Hari Seldon", "Salvor Hardin", "Hober Mallow", "The Mule", "Bayta Darell"]
        .into_iter()
        .map(|s| s.to_string())
        .collect();

    // The following loop will not compile
    // In a for loop, the value that directly follows the keyword for is a pattern
    // So `s`is NOT variable, &s is not a reference, &s is a pattern - specifically, a destructuring pattern.

    // for &s in &foundation {
    //     println!("String is : {}", s);
    // }

    for s in &foundation {
        println!("String is : {}", s);
    }
}

// Patterns can be used in loops to filter and destructure in a single step.
// Here, &Some(score) is not a reference
// It’s a pattern that matches a reference to an Option and destructures it if it’s Some

fn destructuring09() {
    println!("\nDestructuring 09 : matching Option<T> in a for loop\n");

    let maybe_scores = vec![Some(10), None, Some(30)];

    // The pattern is a reference to an Option, so we match &Some(x)
    for &opt in &maybe_scores {
        match opt {
            Some(score) => println!("Score: {}", score),
            None => println!("No score"),
        }
    }

    // Alternative: filter out None before the loop
    for score in maybe_scores.iter().filter_map(|opt| opt.as_ref()) {
        println!("Got a score (filter_map): {}", score);
    }

    // Using if-let inside the loop body
    for maybe in &maybe_scores {
        if let Some(score) = maybe {
            println!("Score via if-let: {}", score);
        }
    }

    // Rather than going through a Vec<Option<T>>, and ignoring the None in the loop
    // we can avoid the if let by flattening the Some directly in the iterator
    for score in maybe_scores.iter().flatten() {
        println!("Score via flatten: {}", score);
    }
}

fn main() {
    dereferencing01();
    dereferencing02_1();
    dereferencing02_2();
    dereferencing03();
    dereferencing04();
    dereferencing05();
    dereferencing06();

    destructuring01();
    destructuring02();
    destructuring03();
    destructuring03_bis();
    destructuring03_ter();
    destructuring03_qua();
    destructuring04();
    destructuring04_bis();
    destructuring05();
    destructuring06();
    destructuring06_bis();
    destructuring07();
    destructuring08();
    destructuring09();
}
