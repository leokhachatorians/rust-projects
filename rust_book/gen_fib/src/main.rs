use std::io;


fn main() {
    loop {
        println!("Which n'th fib number to generate?");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice)
            .expect("Something went wrong bro");

        let choice: u32 = match choice.trim().parse() {
            Ok(num) => {
                if num == 0 {
                    println!("0? Nah homie");
                    continue
                }
                num
            },
            Err(_) => continue,
        };
        
        println!("{}", gen_fib(choice));

        break;

    }
}

fn gen_fib(choice: u32) -> u32 {

    let mut a = 0;
    let mut b = 1;
    let mut temp;

    for _ in 0..choice {
        temp = b;
        b = a + b;
        a = temp;
    }

    return a;
}
