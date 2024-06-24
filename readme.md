# NOTES:

# Systems language:
    It is intended to be used (but not restricted to) to do lower level things
    Building a Compiler
    Building a browser
    Working closer to the OS/kernel

# Generally faster:
    Rust has a separate compilation step (similar to C++) that spits out an optimised binary and does a lot of static analysis at compile time. 
    JS does JIT compilation. 

# concurrency - running multiple threads on a single machine

note: node js is single threaded but can be used as a multithreaded by using IPC - inter process communication

# Memory Safe:
    Rust has a concept of owners,borrowing and lifetimes that make it extremely memory safe 

# Initializing rust project:
    cargo init (application)
    cargo init --lib (library-This would initialize a library that you can deploy for other people to use)


# Use Case:
Rust projects can be used to do a lot of things
    Create Backend for a Full stack app
    Create CLIs (command line interfaces)
    Create browsers
    Great Code Editors

# Variables:

# Number: i8 , i16 , i32 , i64 , i128
fn main() {
    let x: i32 = 1;
    let y: f32 = 32.8;
    println!("{}", x);
}

# Booleans:
fn main() {
    let is_male = false;
    let is_above_18 = true;
    
    if is_male {
        println!("You are a male");

    } else {
        println!("You are not a male");
    }

    if is_male && is_above_18 {
        print!("You are a legal male");
    }
}

# String:
    let str1:&str = "apx13";
    print!("str => {}\n",str1);
    
    let str2 = String::from("apx13_");
    print!("str2 => {}\n",str2);

        In Rust, &str and String are both used to represent strings, but they have different ownership and memory management characteristics:

        &str: This is a string slice, which is a reference to a string stored elsewhere. It's a borrowed reference, meaning it doesn't own the data it refers to. It's commonly used for string literals like "apx13" in your example.
        String: This is a growable, mutable, owned string type provided by the standard library. It owns the string data it contains, allowing for dynamic manipulation such as appending, removing, or modifying characters.

# Conditions:
pub fn main() {
    let x = 99;
    let is_even = is_even(x);
    if is_even {
        print!("{} is even", x);
    } else {
        print!("{} is odd", x);
    }
}

pub fn is_even(x: i32) -> bool {
    return x % 2 == 0;
}

Note:pub is short for "public", and it's used to specify the visibility of items such as functions, structs, enums, and other declarations.

# Loops:

pub fn main() {
    let str = String::from("harkirat singh");
    println!("First name {}", get_first_name(str))
    
}

pub fn get_first_name(str: String) -> String {
    let mut first_name = String::from("");
    for c in str.chars() {
        if c == ' ' {
            break
        }
        first_name.push(c);
    }
    return first_name;
}

fn _ in 0..10{
    println!("hi");
}

# Function:
fn do_sum(a: i32, b: i32) -> i32 {
	return a + b;
}

Note:In Rust, the main function serves as the entry point for executable programs,

# Memory Management in Rust:
Rust has it's own ownership model for memory management which makes it's extremely safe to memory errors
Not having a garbage collector is one of the key concept why rust is so fast 

memory management is primarily handled through ownership, borrowing, and lifetimes, which together ensure memory safety without needing a garbage collector. This approach is known as "ownership system" and it's one of the key features of Rust.

Ownership: Every value in Rust has a variable that's its owner. There can only be one owner at a time. When the owner goes out of scope, the value is dropped (or deallocated).
Borrowing: Instead of transferring ownership, Rust allows you to borrow references to values. Borrowing can be either immutable or mutable. Multiple immutable borrows can exist simultaneously, but only one mutable borrow can exist at a time, and it excludes any other borrows, whether mutable or immutable.
Lifetimes: Lifetimes ensure that references are valid for as long as they are needed and no longer. They prevent dangling references, which occur when a reference refers to memory that has been deallocated.

1. Mutability
2. Heap and Memory 
3. Ownership model 
4. Borrowing and references
5. Lifetimes

# Mutability
By default all the variables are immutable 

# Heap and Memory 
When something is stored in the heap their is still a ptr ( length ,capacity , pointer ) that is stored in the stack . which points the value in the heap

two things cannot points to the same data in heap from stack (Rihana example)
Rihana only dies when owner dies she will not die when borower dies 

(Borowing means - kind of passing referense)
like rihana can have multiple borower but in case of multiple borower she cannot do hanky panky with anyone but if she still wants to do hanky-panky with borower she is only allowed to have only one borower
note: if owner goes out of scope all the borower will die and if borower goes out of acope it doesn't effect the owner

# mutable referenses
only one mutable referense is allowed
if there is a mutable referense then immutable referense is not allowed (in case u are using the mutable referense but if u are not using the mutable referense then the immutable referense can exists) 

# Rules of Borrowing that makes it memory safe
multiple immutable referenses is allowed
single mutable referenses are allowed at a time (hanky panky)
if their is a mutable referense then their cannot be another immutable referenses either

This to avoid any data races/inconsistent behaviour
If someone makes an immutable reference , they don’t expect the value to change suddenly
If more than one mutable references happen, there is a possibility of a data race and synchronization issues
