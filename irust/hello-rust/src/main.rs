// following https://www.rust-lang.org/learn/get-started
#![allow(dead_code)]

// fn main() {
//     println!("Hello, world!");
// }

use ferris_says::say;
use std::io::{stdout, BufWriter};

// ----------------------------------------------------------------------------
fn main1() {
    let stdout = stdout();
    let mut writer = BufWriter::new(stdout.lock());

    let message = String::from("Hello fellow Rustaceans! w00t");
    let width = message.chars().count();
    say(message.as_bytes(), width, &mut writer).unwrap();

    let message = String::from("Hi again lol");
    let width = message.chars().count();
    say(message.as_bytes(), width, &mut writer).unwrap();

    // "Rust has hygienic macros, println! is an example of this."
    println!("Done."); // fascinating, this is printed before the above...
}

// ----------------------------------------------------------------------------
fn main2() {
    let mut x: i32 = 3;  // Mutable variable binding
    print!("{x}");       // Macro for printing, like printf
    while x != 1 {       // No parenthesis around expression
        if x % 2 == 0 {  // Math like in other languages
            x = x / 2;
        } else {
            x = 3 * x + 1;
        }
        print!(" -> {x}");
    }
    println!();
}

// ----------------------------------------------------------------------------
fn main3() {

    // lists
    let mut a: [i8; 10] = [42; 10];
    // a[11] = 0; // does not compile
    a[5] = 0; // ok
    println!("a: {:?}", a);

    // tuples
    let t: (i8, bool) = (7, true);
    println!("1st index: {}", t.0);
    println!("2nd index: {}", t.1);

    // references
    let mut x: i32 = 10;
    let ref_x: &mut i32 = &mut x;
    *ref_x = 20;
    println!("x: {x}");

    // // Rust will statically forbid dangling references
    // // in the code below x is dropped before ref_x is used
    // // because of the inner scope
    // let ref_x: &i32;
    // {
    //     let x: i32 = 10;
    //     ref_x = &x;
    // }
    // println!("ref_x: {ref_x}");

    // this works fine
    let x: i32 = 10;
    let ref_x: &i32 = &x;
    println!("ref_x: {ref_x}");

    // array sclices
    let a: [i32; 6] = [10, 20, 30, 40, 50, 60];
    // a[3] = 5; // does not compile because a is immutable
    println!("a: {a:?}");
    let s: &[i32] = &a[2..4];
    println!("s: {s:?}");

    // String vs str
    // &str an immutable reference to a string slice.
    let s1: &str = "Hello";
    println!("s1: {s1}");
    // String a mutable string buffer.
    let mut s2: String = String::from("Hello ");
    println!("s2: {s2}");
    s2.push_str(s1);
    println!("s2: {s2}");
}

// ----------------------------------------------------------------------------
// functions. fizzbuzz!
fn main4() {
    fizzbuzz_to(20);   // Defined below, no forward declaration needed
}

fn is_divisible_by(lhs: u32, rhs: u32) -> bool {
    if rhs == 0 {
        return false;  // Corner case, early return
    }
    lhs % rhs == 0     // The last expression is the return value
}

fn fizzbuzz(n: u32) -> () {  // No return value means returning the unit type `()`
    match (is_divisible_by(n, 3), is_divisible_by(n, 5)) {
        (true,  true)  => println!("fizzbuzz"),
        (true,  false) => println!("fizz"),
        (false, true)  => println!("buzz"),
        (false, false) => println!("{n}"),
    }
}

fn fizzbuzz_to(n: u32) {  // `-> ()` is normally omitted
    for n in 1..=n { // wtf syntax?
        fizzbuzz(n);
    }
}

// ----------------------------------------------------------------------------
// structs

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn inc_width(&mut self, delta: u32) {
        self.width += delta;
    }
}

fn main5() {
    let mut rect = Rectangle { width: 10, height: 5 };
    println!("old area: {}", rect.area());
    rect.inc_width(5);
    println!("new area: {}", rect.area());
}

// ----------------------------------------------------------------------------
// generic functions

// Overloading is not supported:
// Each function has a single implementation:
// Always takes a fixed number of parameters.
// Always takes a single set of parameter types.
// Default values are not supported:
// All call sites have the same number of arguments.
// Macros are sometimes used as an alternative.
// However, function parameters can be generic:

fn pick_one<T>(a: T, b: T) -> T {
    if std::process::id() % 2 == 0 { a } else { b }
}

fn main6() {
    println!("coin toss: {}", pick_one("heads", "tails"));
    println!("cash prize: {}", pick_one(500, 1000));
}

// ----------------------------------------------------------------------------
// type conversions

fn multiply(x: i16, y: i16) -> i16 {
    x * y
}

fn main7() {
    let x: i8 = 15;
    let y: i16 = 1000;

    println!("{x} * {y} = {}", multiply(x.into(), y));
}

// ----------------------------------------------------------------------------
// arrays and for loops

fn main8() {
    let array = [10, 20, 30];
    println!("array: {array:?}");

    print!("Iterating over array:");
    for n in array {
        print!(" {n}");
    }
    println!();

    print!("Iterating over range:");
    for i in 0..3 {
        print!(" {}", array[i]);
    }
    println!();
}

// ----------------------------------------------------------------------------
// transpose exercise

fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let mut transposed: [[i32; 3]; 3] = [[0; 3]; 3];
    for i in 0..3 {
        for j in 0..3 {
            transposed[i][j] = matrix[j][i];
        }
    }
    transposed
}

fn pretty_print(matrix: &[[i32; 3]; 3]) {
    for row in matrix {
        for elt in row {
            print!("{elt} ")
        }
        println!();
    }
}

fn main9() {

    let matrix = [
        [101, 102, 103], // <-- the comment makes rustfmt add a newline
        [201, 202, 203],
        [301, 302, 303],
    ];

    println!("matrix:");
    pretty_print(&matrix);

    let transposed = transpose(matrix);
    println!("transposed:");
    pretty_print(&transposed);
}

// ----------------------------------------------------------------------------
// put all together the exercises from Day 1: Morning

fn main() {
    main1();
    main2();
    main3();
    main4();
    main5();
    main6();
    main7();
    main8();
    main9();
}
