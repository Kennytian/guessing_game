use std::cmp::Ordering;
use std::io;

use rand::Rng;

fn main() {
    println!("猜数");
    println!("请用键盘输入一个数：");

    // u32 -> unsigned 32
    let secret_number: u32 = rand::thread_rng().gen_range(1..10);

    // {} 表示点位符
    println!("Random number is {}", secret_number);

    // mut 表示 mutable 可变的
    let mut _num = String::new();

    // expect 表示出错后的提示
    io::stdin().read_line(&mut _num).expect("无法读取行");

    println!("Your guessing is :{}", _num);

    let guess: u32 = _num.trim().parse().expect("无效的数字");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("太小了"),
        Ordering::Greater => println!("太大了"),
        Ordering::Equal => println!("相等"),
    }
}
