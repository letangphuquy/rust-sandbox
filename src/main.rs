use ferris_says::say;
use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Welcome to guess the number game!");
    const LIMIT: u32 = 100 as u32;
    let secret = rand::thread_rng().gen_range(1..=LIMIT);

    loop {
        println!("Please input your guess: ");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read a line");
        
        let guess: u32 = match guess.trim().parse() {
            Ok(val) => val,
            Err(_) => {
                println!("Please input a valid number");   
                continue
            }
        }; 
        
        println!("You guessed {guess}");
        match guess.cmp(&secret) {
            Ordering::Less => println!("Small (call him)"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You guessed Correctly!");
                break;
            }
        }
    }
    println!("You won the game, now let's learn some basic syntax");
    basic_syntax();
    
    
}

type T = (i8, f32, char);

fn basic_syntax() {
    let stdout = io::stdout();
    let message = String::from("Hello the fellow Rustaceans!");
    let width = message.chars().count();

    let mut writer = io::BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();

    let mut x = 5;
    println!("The value of x = {}", x);
    x = 10;
    println!("Updated value of x = {}", x);

    let y:u32 = "10".parse().unwrap();
    println!("The value of y = {}", y);

    let c1 = 'a';
    let c2 = '5';
    let c3 = '\u{263A}';
    println!("c1 = {}, c2 = {}, c3 = {}", c1, c2, c3);

    let arr: [f32; 5];
    //get pi
    const PI:f32 = 3.1415926;
    arr = [PI; 5];
    let index = arr.len();
    println!("{}", arr[index-1]);

    let mut string = "Hello, World!".to_string();
    borrow(&string);
    string = "re-assigning successfully".to_string();
    println!("{}", string);
    let st = &string[0..string.len()];
    println!("{}", st);
    println!("Hello {}", fun(12345679, 9));

    let tup: T = (5, PI, 'h');
    swap_tuple(tup);

    let len = 1000234;
    println!("Size of segment tree for array of length {} is {}", len, break_return(len));

    println!("Address of int x = {:p}, of String S = {:?}", &x, string.as_ptr());

}

fn borrow(s: &String) {
    println!("Hello, I borrowed {}", s);
}

fn fun(a: i32, b: i32) -> i32 {
    a*b
}

fn swap_tuple(a: T) {
    println!("Tuple = {} {} {}", a.0, a.1, a.2)
}

fn break_return(n: i32) -> i32 {
    let mut i: i32 = 1;
    loop {
        i *= 2;
        if i >= n {
            break i
        }
        i += 1
    }
}
/*
struct Node {
    value: i32,
    next: &'static Node
}
 */