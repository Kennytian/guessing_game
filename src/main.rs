use std::cmp::Ordering;
use std::io;

use rand::Rng;

fn main() {
    println!("欢迎玩猜数游戏，请用键盘输入一个数：");

    // u32 -> unsigned 32
    let secret_number: u32 = rand::thread_rng().gen_range(1..5);

    loop {
        // mut 表示 mutable 可变的
        let mut _num = String::new();

        // expect 表示出错后的提示
        io::stdin().read_line(&mut _num).expect("无法读取行");

        println!("Your guessing is :{}", _num);

        let guess: u32 = match _num.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("太小了"),
            Ordering::Greater => println!("太大了"),
            Ordering::Equal => {
                println!("相等");
                break;
            }
        }
    }
}
