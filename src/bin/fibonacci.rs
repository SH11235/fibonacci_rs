use std::io;

#[derive(Debug)]
struct Fibonacci {
    num_1: i64,
    num_2: i64,
}

impl Fibonacci {
    fn calc_nth_num(&self, index: u32) -> i64 {
        return match index {
            1 => return self.num_1,
            2 => return self.num_2,
            _ => calc(self.num_1, self.num_2, index)
        };
        
        fn calc(first_term: i64, second_term: i64, index: u32) -> i64 {
            let mut fibo_num = second_term;
            let mut fibo_num_next = first_term + second_term;
            for _ in 3..index {
                let new_fibo_num = fibo_num_next + fibo_num;
                fibo_num = fibo_num_next;
                fibo_num_next = new_fibo_num;
            }
            fibo_num_next
        }
    }
}

fn main() {
    let fibo = Fibonacci {
        num_1: 1,
        num_2: 1,
    };
    println!("Please input the number you want in the Fibonacci sequence");
    let mut input_num = String::new();
    io::stdin()
        .read_line(&mut input_num)
        .expect("Failed to read line");
    let input_num: u32 = input_num.trim().parse().expect("Please type a number!");
    let fibonacci_num = fibo.calc_nth_num(input_num);
    println!("{}", fibonacci_num);
}
