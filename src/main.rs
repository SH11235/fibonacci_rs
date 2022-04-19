use std::io;
fn main() {
    let fibo_num_1 = 1;
    let fibo_num_2 = 2;
    println!("Please input the number you want in the Fibonacci sequence");
    let mut input_num = String::new();
    io::stdin()
        .read_line(&mut input_num)
        .expect("Failed to read line");
    let input_num: u32 = input_num.trim().parse().expect("Please type a number!");
    let fibonacci_num = fibonacci(fibo_num_1, fibo_num_2, input_num);
    println!("{}", fibonacci_num);
}

fn fibonacci(first_term: i64, second_term: i64, index: u32) -> i64 {
    return match index {
        1 => return first_term,
        2 => return second_term,
        _ => calc(first_term, second_term, index)
    };
    
    fn calc(first_term: i64, second_term: i64, index: u32) -> i64 {
        let mut fibo_num = second_term;
        let mut fibo_num_next = first_term + second_term;
        for _ in 3..index-1 {
            let new_fibo_num = fibo_num_next + fibo_num;
            fibo_num = fibo_num_next;
            fibo_num_next = new_fibo_num;
        }
        fibo_num_next
    }
}
