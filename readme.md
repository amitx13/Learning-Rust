# NOTES:
fn main() {
    println!("Hello, world!");
}
Four important details to notice here.
    First, Rust style is to indent with four spaces, not a tab.
    Second, println! calls a `Rust macro`.! means that you’re calling a macro instead of a normal function and that macros don’t always follow the same rules as functions.
    Third, you see the "Hello, world!" string.
    Fourth, we end the line with a `semicolon` (;)

Rust is an ahead-of-time compiled language, meaning you can compile a program and give the executable to someone else, and they can run it even without having Rust installed. If you give someone a .rb, .py, or .js file, they need to have a Ruby, Python, or JavaScript implementation installed (respectively). But in those languages, you only need one command to compile and run your program. Everything is a trade-off in language design.

Cargo is Rust’s build system and package manager
We can create a project using `cargo new`.
We can build a project using `cargo build`.
We can build and run a project in one step using `cargo run`.
We can build a project without producing a binary to check for errors using `cargo check`.
When your project is finally ready for release, you can use `cargo build --release` to compile it with optimizations

Input/Output
The `io` library comes from the standard library, known as `std:`
`use std::io;`

use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
The let mut guess = String::new(); line has created a mutable variable that is currently bound to a new, empty instance of a String. Whew!
like variables, references are immutable by default. Hence, you need to write `&mut guess` rather than `&guess` to make it mutable.

The purpose of these `Result` types is to encode `error-handling` information.
Result’s variants are `Ok` and `Err`. The `Ok` variant indicates the operation was successful, and inside Ok is the successfully generated value. The `Err` variant means the operation failed, and Err contains information about how or why the operation failed.

`{}` in println!() is a `placeholder` for a value

Using a `Crate` to Get More Functionality
Remember that a `crate` is a collection of Rust source code files. The project we’ve been building is a `binary crate`, which is an `executable`. The rand crate is a `library crate`, which contains code that is intended to be used in other programs and `can’t be executed on its own`.

Path Separator`::` - is a fundamental part of Rust's syntax for navigating and accessing the hierarchical structures of modules, items, and types within a Rust program.

# Systems language:
    It is intended to be used (but not restricted to) to do lower level things
    Building a Compiler
    Building a browser
    Working closer to the OS/kernel

# Generally faster:
    Rust has a separate compilation step (similar to C++) that spits out an optimised binary and does a lot of static analysis at compile time. 
    JS does JIT(Just-in-time) compilation. rust code => binary => run

# concurrency - running multiple threads on a single machine
Rust has built-in support for concurrent programming allowing multiple threads to perform tasks simultaneously without risking data races

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

In case of overflow Rust uses the term `panicking` when a program exits with an `error`

# Scalar Types:
A scalar type represents a single value. Rust has four primary scalar types: `integers`, `floating-point` numbers, `Booleans`, and `characters`. You may recognize these from other programming languages

# Compound Types:
Compound types can group multiple values into one type. Rust has two primitive compound types: `tuples` and `arrays`.

# Variables:
Binding and mutability
🌟 A variable can be used only if it has been initialized. Uninitialized variable caused error
🌟 Use mut to mark a variable as mutable.
    assert_eq!(x, 5); can be used to check equality

Constants aren’t just immutable by default—they’re `always immutable`. You declare constants using the `const`    

Shadowing generally used to convert a value from one type to another type.
when shadowing let must be used otherwise it will throw an error when shadowing one variable type and value can be changes into another type and value but in mut a variable can only change it's value not the type
You can declare a new variable with the same name as a previous variable and can also reinitialize it 
fn main() {
    let x: i32 = 5;
    {
        let x = 12; //Shadowing  here we can say the first one is shadowed by the second one.
        assert_eq!(x, 12);
    }

    assert_eq!(x, 5);

    let x = 42;
    println!("{}", x); // Prints "42".
}

Destructuring
// Fix the error below with least amount of modification
fn main() {
    let (mut x, y) = (1, 2);
    x += 2;

    assert_eq!(x, 3);
    assert_eq!(y, 2);

    println!("Success!");
}

Destructuring assignments
You can now use tuple, slice, and struct patterns as the left-hand side of an assignment.

fn main() {
    let (x, y);
    (x,..) = (3, 4); Tuple Assignment
    [.., y] = [1, 2]; Array Pattern Matching
    // Fill the blank to make the code work
    assert_eq!([x,y], [3, 2]);

    println!("Success!");
} 

# Number: i8 , i16 , i32(By Default) , i64 , i128

doesn't , matter if the value is correct or not the type should also be similar 
fn main() {
    let x: i32 = 1;
    let y: f32 = 32.8;
    println!("{}", x);
}

error (
fn main() {
    let x: i32 = 5;
    let mut y: u32 = 5;

    y = x;
    
    let z = 10; // Type of z ? 
    println!("Success!");
}
)

casting a u8 value to u16 value 
fn main() {
    let v: u16 = 38_u8 as u16;

    println!("Success!");
}

# Floating-Point
By default f64 assigns

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

# The Tuple Type:
A tuple is a general way of `grouping` together a number of values with a `variety of types` into one compound type. Tuples have a `fixed length`: once declared, they cannot grow or shrink in size.

We create a tuple by writing a comma-separated list of values inside parentheses

eg. `let tup: (i32, f64, u8) = (500, 6.4, 1);`
destructure    `let (x, y, z) = tup;`
We can also access a tuple element directly by using a period (.) followed by the index of the value we want to access. For example:
    
    let x: (i32, f64, u8) = (500, 6.4, 1);
    
    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;

# The Array Type:

Another way to have a collection of multiple values is with an array. Unlike a tuple, every element of an array must have the `same type`. Unlike arrays in some other languages, arrays in Rust have a `fixed length`.

Arrays are useful when you want your data allocated on the `stack` rather than the `heap`
A `vector` is a similar collection type provided by the standard library that is allowed to `grow` or `shrink` in size.

let a: [i32; 5] = [1, 2, 3, 4, 5];
let a = [3; 5]; // = [3,3,3,3,3]

# Statements and Expressions:
Statements are instructions that perform `some action and do not return a value.`
statement example:
fn main() {
    let y = 6;
}

`Function` definitions are also statements; the entire preceding example is a statement in itself.

Statements do not return values. Therefore, you can’t assign a `let` statement to another variable, as the following code tries to do; you’ll get an `error`:
Error: `let x = (let y = 6);`
In other language you can write `x = y = 6` and have both x and y have the value `6`; that is `not the case in Rust.`

Expressions `evaluate to a resultant value.`
Consider a math operation, such as `5 + 6`, which is an expression that evaluates to the value `11`.
`Calling a function` is an expression. `Calling a macro` is an expression. A new `scope block` created with `curly brackets` is an expression
Example

fn five() -> i32 {
    5
}

fn main() {
    let x = five(); // Here the 5 is got returned becz it's an `expression` and expression returns a value. Note there is not a return type and also semicolon 
                    // (not statement) still is got returned that's the beauty of expression 
    println!("The value of x is: {x}");
}


Keep in mind:
Note that the `x + 1` line doesn’t have a `semicolon` at the end, which is unlike most of the lines you’ve seen so far. `Expressions do not include ending semicolons`. If you add a `semicolon` to the end of an `expression`, you turn it into a `statement`, and it will then not return a value.

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

let number = if condition { 5 } else { 6 }; can also be used in assigning variable but the point here is it should be of same type 
like if condition { 5 } else { "six } will throw an error

# Loops:

loop{
    //works as infinite loop so break statement is necessary
}

fn main() {
    for number in (1..4).rev() { //.rev() to reverse the range:
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}

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

# Memory management:
# Garbage collector
1. Written by smart people
2. Usually no danging
pointers/memory issuse
3. You cant do manual
memory management
4. Examples - Java, IS

# Manual:
1. You allocate and
deallocate memory yourself
2. Can lead to danging
pointers/memory issuse
3. Learning curve is high
since you have to do manual
MM
Examples - c

# The rust way:
1. Rust has its own
ownership model for memory
management
2. Makes it extremely safe
to memory errors

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
mmutable data is inherently `thread-safe` because if no thread can alter the data, then no `synchronization` is needed when data is accessed concurrently.And no `race around` condition 
Knowing that certain data will not change allows the compiler to optimize code better. 

# What Is Ownership?
Ownership is a set of rules that govern how a Rust program manages memory. All programs have to
manage the way they use a computer's memory while running. Some languages have garbage
collection that regularly looks for no-longer-used memory as the program runs; in other languages,
the programmer must explicitly allocate and free the memory. Rust uses a third approach: memory
is managed through a system of ownership with a set of rules that the compiler checks. If any of the
rules are violated, the program won't compile. None of the features of ownership will slow down
your program while it's running.

# Heap and Memory 
When something is stored in the heap their is still a ptr ( length ,capacity , pointer ) that is stored in the stack . which points the value in the heap

(ownership)
two things cannot points to the same data in heap from stack (Rihana example)
Rihana only dies when owner dies she will not die when borower dies 

cloning is expansive for heap 

(Borowing means - kind of passing referense)
borowing can be done in two ways 
1. `mutable referenses`
2. `ImMutable referenses`
like rihana can have multiple borower but in case of multiple borower she cannot do hanky panky (`ImMutable referenses`) with anyone but if she still wants to do hanky-panky with borower she is only allowed to have only one borower(`mutable referenses`)
note: if owner goes out of scope all the borower will die and if borower goes out of acope it doesn't effect the owner

# mutable referenses
only one mutable referense is allowed
if there is a mutable referense then immutable referense is not allowed (in case u are using the mutable referense but if u are not using the mutable referense then the immutable referense can exists) 

# Rules of Borrowing that makes it memory safe
multiple immutable referenses is allowed
single mutable referenses are allowed at a time (hanky panky) (you cannot pass variable as mutable if variable itself is not mutable)
if their is a mutable referense then their cannot be another immutable or mutable referenses anymore

This to avoid any data races/inconsistent behaviour
If someone makes an immutable reference , they don’t expect the value to change suddenly
If more than one mutable references happen, there is a possibility of a data race and synchronization issues


# Rust by Practice:
`Ownership and Borowing`

fn main() {
    // Use as many approaches as you can to make it work
    let x = String::from("Hello world");
    let y = x.clone();
    println!("{}, {}",x, y);
}

Approach - .clone() that will create an another instance of x and point it to y

// Don't modify code in main!
fn main() {
    let s1 = String::from("Hello world");
    let s2 = take_ownership(s1);

    println!("{}", s2);
}

// Only modify the code below!
fn take_ownership(s: String) -> String {
    println!("{}", s);
    return s;
}

1.Approach returned the borrowed value 2.Retrun through expression 

# Implement

The `impl` keyword is used to define implementations for types, which can include methods, trait implementations, and associated functions. 
struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

fn main() {
    let Rectangle = Rectangle {
        width:20,
        height:30,
    };
    println!("The area of the rectangle is {}",Rectangle.area());
}

# Pattern Matching
`Rust` has an extremely powerful control flow construct called `match` that allows you to compare a value against a series of patterns and then execute code based on which pattern matches.
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

Matches Are `Exhaustive`
There’s one other aspect of match we need to discuss: the arms’ patterns must cover all possibilities. 
Consider this version of our plus_one function, which has a bug and won’t compile:

`error` 
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            Some(i) => Some(i + 1),
        }
    }
We didn’t handle the `None` case, so this code will cause a bug. 

# Error Handling:
Errors:
    Compilation error: no binary got spitt out of the original code.
    Runtime error: Final binary got created but while this binary was running something got wrong or it got crashed etc.

Error handling in rust is being done using `Result Enum` it is similar to try catch in javascript

Error handling in rust 
enum Result<A , B>{
    Ok(A),
    Err(B),
}
implemented using pattern matching

use std::fs

fn main(){
    let res = fs::read_to_string("example.txt");
    match res {
        Ok(content)=> {
            println!("file content : {}",content)
        }
        Err(err)=>{
            println!("Error :{}",err)
        }
    }
}

# Option Enum:
The Option enum was introduced in Rüst to handle the concept of `nullability` in a safe and
expressive way. Unlike many programming languages that use a null or similar keyword to
represent the absence of a value, Rust doesn't have null.

pub enum Option<T> {
    None,
    Some(T),
}

If you ever have a function that should return null, return an Option instead.

fn return_index(s:String) -> Option<i32>{
    for (index, character) in s.chars().enumerate() {
        if character == 'a' {
            return Some(index as i32);
        }
    }
    return None;
}

fn main(){
    let res = return_index("amit".to_string());
    match res {
        some(index) => println!("this is the index:{}",index);
        None => println!("None value found");
    }
}

# Crates.
Just like the nodejs ecosystem has npm, the rust ecosystem has cargo
Cargo is a package manager for rust, which means we can use it to bring packages (crates in case of rust) to our projects

https://crates.io/crates/chrono - Store time in a DB/as a variable
https://actix.rs/ - Extremely fast http server
https://serde.rs/ - Serializing and deserialising data in rust
https://tokio.rs/ - Asynchronous runtime in rust
https://docs.rs/reqwest/latest/reqwest/ - Send HTTP requests
https://docs.rs/sqlx/latest/sqlx/ - Connect to sql database

# Commaon Collections.
`collections` can contain multiple values. Unlike the built-in array and tuple types, the data these collections point to is stored on the heap, which means the amount of data does not need to be known at compile time and can grow or shrink as the program runs. 
Each kind of collection has different capabilities and costs, and choosing an appropriate one for your current situation is a skill you’ll develop over time

`vector`
`String`
`hash map`

# Vector
`Vec<T>`, also known as a vector
 Vectors allow you to store more than one value in a single data structure that puts all the values next to each other in memory. Vectors can only store values of the same type. They are useful when you have a list of items, such as the lines of text in a file or the prices of items in a shopping cart.

Creating a New Vector
    let v: Vec<i32> = Vec::new();

Initialization
    let v = vec![1, 2, 3, 4, 5];

Take a look at this example:

perfect use of `immutable reference` , `shadowing` , `Option enum` , `Pattern matching`

    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }


`Borrowing` in case of `Vector`

    let mut v = vec![1, 2, 3, 4, 5];

    let first = &v[0]; //Immutabley borrowed

    v.push(6); //mutabley borrowed 

    println!("The first element is: {first}");


But you may think both the index are different and hence both of the memory referenses are different so why should a reference to the first element care about changes at the end of the vector? 

Well the error is due to the way Vector works :
Because vectors put the values next to each other in memory, adding a new element onto the end of the vector might require allocating new memory and copying the old elements to the new space, if there isn’t enough room to put all the elements next to each other where the vector is currently stored. In that case, the reference to the first element would be pointing to deallocated memory. The borrowing rules prevent programs from ending up in that situation.

`Iteration` in Vector:

    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }


# String

Indexng is not supported in rust like it's supported in rust so we ca use string slicing in case of indexing

JS:
    const s = "apx13"
    console.log(s[0]) // here a will get printed

Rust:
    let s = String::from("apx13");
    println!("{}",s[0]) // is not possible in rust Becz A String is a wrapper over a Vec<u8> and vector stores the string "apx13” is form of bytes
    let s1 = &s[0..4]; //here ap got coppied brcause each char is of 1 byte and hense it contain first 4 bytes of the string i.e apx1 

Methods for `Iterating` Over Strings;

    for c in "Зд".chars() {
        println!("{c}");
    }

    for b in "Зд".bytes() {
        println!("{b}");
    }

`Difference between String / &str`

    let str1:&str = "apx13";
    print!("str => {}\n",str1);
    
    let str2 = String::from("apx13_");
    print!("str2 => {}\n",str2);

`Ownership`: String owns the string data, while &str borrows it.
`Mutability`: String is mutable; &str is immutable.
`Memory`: String data is stored on the heap, while &str typically points to data on the stack or within a String.
`Use cases`: Use String when you need ownership and mutability. Use &str when you need to borrow string data without taking ownership or modifying it.


# Hash-Map
it is `Key-value` pair just like object in js or dictonary in python

Creating a New Hash Map

    use std::collections::Hashmap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    //accessing value in Hashmap
    let alice = score.get("Blue")

    // Iterate over key-value pairs
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // Check if a key exists
    if scores.contains_key("Blue") {
        println!("Blue is in the HashMap");
    }

    //Remove a key
    scores.remove("Blue");

    //Overwriting a Value
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{scores:?}");

# Unwrap
The `unwrap` method is used to handle Option and Result types,
which are commonly used for error handling and dealing with values that may or may not be present. 
The unwrap method is used to `extract` the value inside an `Option` or `Result`

If the Option is Some or the Result is Ok, it returns the contained value. If the Option is None or the Result is Err, it causes the program to panic and terminate with an error message.

usecase:
when you are sure that the value will be present and want to ignore error handling temporarily.
unwrap can lead to runtime panics if the value is None or Err, making the program crash. Therefore, it should be used with caution

what's rust preffered to use `Pattern Matching`, `Combinators (unwrap_or, unwrap_or_else, and_then, etc.)`, `Expect`

# Generic
Generic are same as Ts 
ex code:

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

fn main() {
    let p = Point { x: 5, y: 10 };

    println!("p.x = {}", p.x());
}

By declaring <T> after impl, we inform Rust that T is a generic type parameter.
This declaration is necessary so that Rust knows that Point<T> in the impl block refers to the generic Point<T> struct rather than a Point with some concrete type T.

# Traits
Traits in Rust and abstract classes in Java both define a set of methods that must be implemented by any type that wants to adhere to the trait or extend the abstract class.
Both are used to specify behavior that multiple types can share.

• Set of methods that can be implemented for multiple types in order to provide common functionality and behaviour between them
• Traits constist of method signatures only, which then have to be implemented by the target type
• Similar to "classes"(abstract class) in other languages, not quite the same though
• Defines shared behaviour in an abstract way
A trait can have multiple methods in its body: the method signatures are listed one per line and each line ends in a semicolon.

Traits are used to define methods that can be implemented by any type.
Traits can contain default method implementations.
A type can implement multiple traits (Rust supports multiple inheritance through traits).
Traits can be used as bounds to specify that a generic type must implement certain behavior.
Traits can’t contain fields, only methods.

Derivable Traits
• Trait that can be automatically implemented for a struct or an enum by the Rust compiler
• Called "derivable" because they can be derived automatically.
• Most common derivable traits:
- Debug: Allowing to output content via "{:?}"
- Clone: Enables type to be duplicated with "clone()" method
- Copy: Enables type to be copied implicity, without recquiring explicit "clone()" method
- PartialEq: Enables comparison

Traits as Parameters
pub fn notify(item: &impl Summary) {
    println! {"Breaking news! {}", item. summarize()};
}
Traits can be used as parameters for functions.
The function notify () takes as argument any type that has
implemented the Summary trait.

pub trait Animal {
    fn sound(&self);
}

struct Dog;

impl Animal for Dog {
    fn sound(&self){
        println!("{}","woof! Woof!");
    }
}

fn main(){
    let dog = Dog;
    dog.sound();
}

Adding Default implementation 

pub trait Animal {
    fn make_sound(&self);
    fn Breed(&self) -> &str {
        "unknown"
    }

    fn Color(&self) -> &str {
        "unknown"
    }
}

struct Dog;
struct Cat;

impl Animal for Dog {
    fn make_sound(&self){
        println!("{}","woof! woof!");
    }
    fn Breed(&self) -> &str {
        "Golden Retriver"
    }
}

impl Animal for Cat {
    fn make_sound(&self){
        println!("{}","Meow");
    }
    fn Color(&self) -> &self{
        "Black"
    }
}

fn main(){
    let dog = Dog;
    let cat = Cat;
    dog.make_sound();
    cat.make_sound();
    println!("Breed:{} \n Color:{}",dog.Breed , dog.Color);
    println!("Breed:{} \n Color:{}",cat.Breed , cat.Color);
}

//Output
Woof! Woof!
Meow
Breed: Golden Retriver
Color: unknown
Breed: unknown
Color: Black

now in the above example you can see we have not folowed the DRY principle that means the code is not perfect it's still needs modifications
now here `trait as a parameter` comes into the picture
where we define a fn that takes traits as a parameter 

fn describe_animal( animal:&impl Animal ){
    animal.make_sound();
    println!("Breed:{} \n Color:{}",animal.Breed , animal.Color);
}

now you will see the main fn will look way cleaner than before

fn main() {
    let dog = Dog;
    let cat = Cat;
    let cow = Cow;

    describe_animal(&dog);
    describe_animal(&cat);
    describe_animal(&cow);
}

# Trait Bound Syntax:

# Single:
trait Animal {
    fn makeSound(&self);
}

//`Basic way`
simplest way to specify a trait bound is by using the impl Trait syntax.

fn describeAnimal(animal:&impl Animal){
    animal.makeSound();
}

//`Generic way`
fn describeAnimal<T:Animal>(animal:&T){
    animal.makeSound();
}

# Multiple:

trait Animal {
    fn makeSound(&self);
}
trait Name{
    fn Name(&self) -> &str;
}

//`Basic way || Inline syntax`
fn describeAnimal(animal:(&impl Animal + Name)){
    animal.makeSound();
    println!("name is : {}",animal.Name());
}

//`Generic type parameter`
fn describeAnimal<T>(animal:&T)
where
    T:Animal+Name,
{
    animal.makeSound();
    println!("name is : {}",animal.name());
}

`we can also return trait from function`

trait Animal {
    fn make_sound(&self);
}

struct Dog;
struct Cat;

impl Animal for Dog {
    fn make_sound(&self){
        println!("{}","woof!");
    }
}

impl Animal for Cat {
    fn makeSound(&self){
        println!("{}","meow");
    }
}

fn create_animal(name:&str) -> impl Animal {
    if name == "dog" {
        Dog
    }
    else{
        Cat
    }
}

fn main(){
    let animal = create_animal("dog");
    animal.makeSound();
}

# LifeTimes: (Generic Lifetime Annotations)
`Dangaling referenses`- Referense that points to invalid data and rust dont allow dangaling refrenses
eg code:
fn main(){
    let r;
    {
        let x = 8;
        r = &x; //In rust we can refrense primitive value  
    }
    println!("{}",r);
}

