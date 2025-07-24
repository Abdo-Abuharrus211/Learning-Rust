fn main() {
    // let guess: u32 = "42".parse().expect("Not a number!");
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("{}", guess);
    let val: bool = useless_fun(21);
    println!("{}", val);
    let bob = true;
    let word: &str = if bob {"yeah"} else { "no" };
    println!("{word}");
}

fn useless_fun(num: u32) -> bool {
    if num % 77 != 0 {
        return false;
    } else if num % 77 == 4 {
        return true;
    }
    match num {
        4 => true,
        _ => false,
    }
}

// fn shadowing_scope() {
//     let x = 5;
//     let x = x + 1;
//     {
//         let x = x * 2;
//         println!("The value of x in the inner scope is: {x}");
//     }
//     println!("The value of x is: {x}");
// }
