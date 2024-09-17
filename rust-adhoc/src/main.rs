//Wap to take a num as a Input and return True and False if num is even or odd

// use std::io;

// fn main(){
//     println!("Enter a number to check if it is even or not");
//     let mut num = String::new();

//     io::stdin()
//     .read_line(&mut num)
//     .expect("Failed to take Input try again!!");

//     let num: i32 = num.trim().parse().expect("Please enter a valid number");

//     if num % 2 == 0  {
//         println!("{} is even",num);
//     } else {
//         println!("{} is odd",num);
//     }
// }

// WAF fib that finds the fibbonacci of a num it takes as an input

// use std::io;

// fn find_fibbonacci(x:u32) -> u32{
//     if x == 0 {
//         0
//     } else if x == 1 {
//         1
//     } else {
//         find_fibbonacci(x - 1) + find_fibbonacci(x - 2)
//     }
// }
// fn main(){
//     let mut num = String::new();

//     println!("Enter a num to find the fibbonacci of a num");

//     io::stdin().read_line(&mut num).expect("Failed to take Input!!");

//     let num:u32 = num.trim().parse().expect("Please Enter a valid num");

//     println!("The {}th Fibonacci number is: {}", num, find_fibbonacci(num));
// }


//WAF that takes input as string and return it's length

// use std::io;

// fn get_string_length(str:String)-> usize{
//     // let mut count:u32 = 0;
//     // for i in str.chars(){
//     //     if i == ' '{
//     //         break;
//     //     }
//     //     count+= 1
//     // }
//     // return count

//     //or just use count 

//     str.chars().count()
// }
// fn main(){
//     let mut str = String::new();

//     println!("Enter a string to find it's length");

//     io::stdin().read_line(&mut str).expect("something went wrong!");

//     println!("Length of the entered string is : {}",get_string_length(str.trim().to_string()));
// }


//struct 
// struct Rect {
//     height: u32,
//     width: u32
// }

// impl Rect {
//     fn area(&self) -> u32 {
//         self.height * self.width
//     }
// }
// fn main(){
//     let rect = Rect {
//         height:10,
//         width:5
//     };

//     println!("The are of given rectangle is : {}",rect.area());
// }

// Enum
// enum Shape {
//     Rectangle(f64, f64), // variants with associated data 
//     Circle(f64)
// }

// fn main() {
//     let rect = Shape::Rectangle(8.0, 4.0);
//     match calculate_area(rect) {
//         Some(area) => println!("Area of rectangle is : {}", area),
//         None => println!("Failed to calculate area of rectangle"),
//     }

//     let circle = Shape::Circle(5.0);
//     match calculate_area(circle) {
//         Some(area) => println!("Area of circle is : {}", area),
//         None => println!("Failed to calculate area of circle"),
//     }
// }

// fn calculate_area(shape: Shape) -> Option<f64> {
//     match shape {
//         Shape::Rectangle(a, b) => Some(a * b),
//         Shape::Circle(r) => Some(3.14 * r * r),
//     }
// }

//WAP to read content of a file 

// use std::fs;

// fn main(){
//     let content = fs::read_to_string("src/note.txt");

//     match content {
//         Ok(c) => println!("{}", c),
//         Err(e) => println!("Error reading file: {}", e),
//     }
// }

//WAP to show borrowing in rust
// fn main(){
//     let str1 = String::from("apx");
//     print_str(&str1);
//     println!("{}",str1)
// }

// fn print_str(str:&String){
//     println!("{}",str);
// }

//Vector:

fn main(){
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    println!("{:?}",vec);
}