// use rand::random;
// fn main(){
//    let number= random::<u64>();
//    println!("Random is {}",number);
// }

//guessing game
use rand::{thread_rng,Rng};
use std::io;
use std:: cmp::Ordering;
fn main(){
    println!("welcome to the guessing game");
    let secret_number= rand::thread_rng().gen_range(1..101);
    // println!("secret number is {}",secret_number);
    loop{
    println!("please input your guess");
    let mut guess= String::new();
    io::stdin().read_line(&mut guess).expect("failed to read line");
    println!("your Guesses {}", guess);

    let guess:u32=guess.trim().parse().expect("type a integer");//converting to integer
    // println!("{}",guess+1);
 

    match guess.cmp(&secret_number){
        Ordering::Less=>println!("to small"),
        Ordering::Greater=>println!("to large"),

        Ordering::Equal=>{
            println!("you win ");
            break;
        }

    }

 }
   

}