// https://doc.rust-jp.rs/book-ja/ch08-03-hash-maps.html
// 整数のリストが与えられ、ベクタを使ってmean(平均値)、
// median(ソートされた時に真ん中に来る値)、 mode(最も頻繁に出現する値; ハッシュマップがここでは有効活用できるでしょう)
// を返してください。

struct NumList {
    list: Vec<i64>,
}

fn main() {
    let num_list = NumList {
        list: get_num_list(),
    };
    let sum = num_list.get_sum();
    let length = num_list.get_vec_len();
    let mean = num_list.get_mean();
    let median = num_list.get_median();
    let mode = num_list.get_mode();

    println!("sum: {}", sum);
    println!("length: {}", length);
    println!("mean: {}", mean);
    println!("median: {}", median);
    println!("mode: {}", mode);
}

impl NumList {
    fn get_sum(&self) -> i64 {
        // iteratorを知らないとき
        // let mut sum:i64 = 0;
        // for num in &self.list {
        //     sum += num
        // }
        // sum

        // iteratorを知っているとき
        self.list.iter().fold(0, |acc, x| acc + x)
    }

    fn get_vec_len(&self) -> usize {
        self.list.len()
    }

    fn get_mean(&self) -> f64 {
        self.get_sum() as f64 / self.get_vec_len() as f64
    }

    fn get_median(&self) -> f64 {
        let length = self.get_vec_len();
        let median_index = length / 2;
        if length % 2 == 0 {
            (self.list[median_index - 1] + self.list[median_index]) as f64 / 2.0
        } else {
            self.list[median_index] as f64
        }
    }

    fn get_mode(&self) -> i64 {
        use std::collections::HashMap;
        let mut map: HashMap<&i64, i32> = HashMap::new();
        for num in &self.list {
            let count = map.entry(num).or_insert(0);
            *count += 1
        }

        // iteratorを知らないとき
        // let mut mode_num = 0;
        // let mut max = 0;
        // for (num, count) in &map {
        //     if *count > max {
        //         mode_num = **num;
        //         max = *count;
        //     }
        // }
        // mode_num

        let max = map.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap();
        **max.0
    }
}

fn get_num_list() -> Vec<i64> {
    vec![
        1,
        1,
        2,
        3,
        5,
        8,
        13,
        21,
        34,
        55,
        // 89,
        // 144,
        // 233,
        // 377,
        // 610,
        // 987,
        // 1597,
        // 2584,
        // 4181,
        // 6765,
        // 10946,
        // 17711,
        // 28657,
        // 46368,
        // 75025,
        // 121393,
        // 196418,
        // 317811,
        // 514229,
        // 832040,
        // 1346269,
        // 2178309,
        // 3524578,
        // 5702887,
        // 9227465,
        // 14930352,
        // 24157817,
        // 39088169,
        // 63245986,
        // 102334155,
        // 165580141,
        // 267914296,
        // 433494437,
        // 701408733,
        // 1134903170,
        // 1836311903,
        // 2971215073,
        // 4807526976,
        // 7778742049,
        // 12586269025,
        // 20365011074,
        // 32951280099,
        // 53316291173,
        // 86267571272,
        // 139583862445,
        // 225851433717,
        // 365435296162,
        // 591286729879,
        // 956722026041,
        // 1548008755920,
        // 2504730781961,
        // 4052739537881,
        // 6557470319842,
        // 10610209857723,
        // 17167680177565,
        // 27777890035288,
        // 44945570212853,
        // 72723460248141,
        // 117669030460994,
        // 190392490709135,
        // 308061521170129,
        // 498454011879264,
        // 806515533049393,
        // 1304969544928657,
        // 2111485077978050,
        // 3416454622906707,
        // 5527939700884757,
        // 8944394323791464,
        // 14472334024676221,
        // 23416728348467685,
        // 37889062373143906,
        // 61305790721611591,
        // 99194853094755497,
        // 160500643816367088,
        // 259695496911122585,
        // 420196140727489673,
        // 679891637638612258,
    ]
}
