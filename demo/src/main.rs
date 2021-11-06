use rand::Rng;
use std::cmp::Ordering;
use std::fs;
use std::fs::File;
use std::io;

fn main() {
    // readFromCommondline();
    // write_to_file();
    radnom_number();
}

// 测试命令行输入
fn read_from_commend_line() {
    let mut strBuf = String::new();
    std::io::stdin()
        .read_line(&mut strBuf)
        .expect("faild to read line");
    println!("your input line is {}", strBuf);
}

// 测试创建文件并写入文本
fn write_to_file() {
    let mut file = File::create("foo.txt").unwrap();
    // file.write(b"this is string").unwrap();
}

fn read_from_file() {
    let content = fs::read("foo.txt").unwrap();
    println!("content --> {:?}", content);
}

// 猜数游戏
fn radnom_number() {
    println!("guess number game!");
    let secret_number: u32 = rand::thread_rng().gen_range(1..101);
    println!("secret_number --> {}", secret_number);

    loop {
        println!("guess a number");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("无法读取行");
        if guess.trim() == "exit" {
            println!("exit game!");
            break;
        }

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(e) => {
                println!("please type a numer --> {}", e);
                continue;
            }
        };
        println!("你猜测的数字 --> {}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("too big"),
            Ordering::Equal => println!("you win"),
        }
    }
}

// 所有权测试
fn ownership_test() {
    let s = String::from("hello");

    //函数获取了s变量的所有权，在函数退出后，s变量将被回收，后续使用s将编译错误
    takes_ownership(s);

    // println!("test - {}", s);  编译错误

    let x = 5;
    // 值复制
    make_copy(x);

}
fn takes_ownership(s: String) {
    println!("takes ownership {}", s);
}

fn make_copy(i: i32) {
    println!("make copy {}", i);
}
