use std::io;

fn add_big(a: &[u8], b: &[u8]) -> Vec<u8> {
    let mut result = vec![0; a.len()+1];
    let mut carry = 0;
    for i in 0..a.len() {
        let sum = a[i] + b[i] + carry;
        result[i] = sum % 10;
        carry = sum / 10;
    }
    if carry > 0 {
        result[a.len()] = carry;
    }
    result
}

fn main() {
    println!("Pick a fib number, world!");

    let mut fib_num = String::new();

    // Get stdin input to fibNum
    io::stdin()
        .read_line(&mut fib_num)
        .expect("Failed to read line");

    // Parse input to an integer
    let fib_num: u32 = match fib_num.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };

    if fib_num == 0 {
        println!("0");
    } else if fib_num == 1 {
        println!("1");
    } else {
        let mut fib1 = [0u8; 1024];
        fib1[0] = 1;
        let mut fib2 = [0u8; 1025];
        fib2[0] = 1;
        for _ in 2..fib_num {
            let temp = fib1;
            fib1 = fib2[0..1024].try_into().expect("slice with incorrect length");
            let fib2_vec = add_big(&temp, &fib2);
            fib2 = fib2_vec
                .try_into()
                .unwrap_or_else(|fib2_vec: Vec<u8>| panic!("Expected a Vec of length {} but it was {}", 1024, fib2_vec.len()))
        }
        println!("{}", fib2.iter().rev().map( |&id| id.to_string() + "").collect::<String>());
    }
}
