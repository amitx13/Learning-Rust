//Struct
// struct User{
//     username:String,
//     email:String,
// }

enum Shape{
    Circle(f32), //variend with associated data (radius)
    Square(f32), //variend with associated data (side length)
    Rectangle(f32 , f32) //varient with associated data (width , height )
}
fn main() {
    // let x:i8 = 13;
    // print!("x => {}\n",x);

    // let f:f32 = 1.3;
    // print!("f => {}\n",f);

    // //&str and string are different
    // let str1:&str = "apx13";
    // print!("str => {}\n",str1);

    // let str2 = String::from("apx13_");
    // print!("str2 => {}\n",str2);

    // let char = str2.chars().nth(9); //similar to console.log(str[0])

    // match char { //patern matching
    //     Some(c) => {println!("char {}", c)},
    //     None => println!("exceeding total numbers of characters..."),
    // }

    // let str3: &str = "_legend";

    // let combine: String = format!("{}{}",str1,str3);
    // println!("combine => {}", combine);

    //mutable referense
    // let mut name = String::from("apx13");
    // update_string(&mut name);
    // println!("{}",name);

    //Referenses
    // let test = String::from("hello");
    // referense_test(&test);
    // println!("{}", test);

    //struct
    // let user = User{
    //     username:String::from("apx13_"),
    //     email:String::from("amitkvs981@gmail.com"),
    // };
    // println!("{}|{}",user.username,user.email);

    //Enums
    let circle = Shape::Circle(3.0);
    let square = Shape::Square(4.0);
    let rectangle = Shape::Rectangle(4.0, 2.0);

    println!("circle area = {}",calculate_area(circle));
    println!("Square area = {}",calculate_area(square));   
    println!("Rectangle area = {}",calculate_area(rectangle));
}

fn calculate_area(shape:Shape) -> f32 {
    match shape{
        Shape::Circle(radius)=> 3.24 * radius *radius,
        Shape::Square(length)=> length*length,
        Shape::Rectangle(w,h )=> w*h
    }
}

// fn update_string(str: &mut String) { //mutable referense
//     str.push_str("_hi")
// }

// fn referense_test(str: &String) { //immutable referense
//     println!("{}", str);
// }
