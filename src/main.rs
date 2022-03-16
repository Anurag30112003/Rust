// fn main() {
//    let a = 10;
//    let b = 10;
//    if a == b
//    {
//        println!("a is equal to b");
//    }
//    else if a>b
//     {
//          println!("a is greater to b");
//     }
//     else if a<b
//     {
//         println!("a is less to b");
//     }
//     else {
        
//     }
// }
// fn main(){
//     let mut line = String::new();
//     println!("Enter your name :");
//     let b1 = std::io::stdin().read_line(&mut line).unwrap();
//     println!("Hello , {}", line);
//     println!("no of bytes read , {}", b1);
//  }
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
