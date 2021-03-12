const push_i8: u8 = 0;
const add: u8 = 1;

fn main() {
    let mut stack: Vec<u8> = vec![];

    let mut instruction: Option<u8> = None;

    loop {
        println!("0 to push i8");
        println!("1 to add");

        let byte_op = read_line();
        if byte_op == push_i8 {
            let i = read_line();
            stack.push(i);

            println!("Stack: {:#?}", stack);
        } else if byte_op == add {
            if stack.len() < 2 {
                println!("Stack underflow!");
            } else {
                let a = transmute_u8_i8(stack.pop().unwrap());
                let b = transmute_u8_i8(stack.pop().unwrap());

                let c = a + b;
                println!("Val: {:?}", c);
                stack.push(c as u8);
            }
        }
    }

    println!("Hello, world!");
}

fn transmute_i8_u8(i: i8) -> u8 {
    unsafe { std::mem::transmute::<i8, u8>(i) }
}

fn transmute_u8_i8(i: u8) -> i8 {
    unsafe { std::mem::transmute::<u8, i8>(i) }
}

fn read_line() -> u8 {
    println!("Input a u8:");
    use std::io::{self, BufRead};
    let mut line = String::new();
    let stdin = io::stdin();
    stdin.lock().read_line(&mut line).unwrap();
    let line = line.trim();
    match line.parse::<u8>() {
        Ok(u) => {
            println!("Read '{:?}'", line);
            return u;
        }
        Err(_) => {
            println!("Invalid: '{:?}' not a byte.", line);
            return read_line();
        }
    }
}
