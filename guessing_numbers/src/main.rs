

use std::io; // prelude
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("猜数游戏");
    println!("请猜测一个数字：");
    //can’t put rand method into loop , Or it will gen each time
    let secret_number = rand::thread_rng().gen_range(1,101);
    //println!("神秘数字是：{}", secret_number);
    loop{
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).
            expect("无法读取行");
        println!("你猜测的数字是 {}" , guess);
        let guess:u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),//arm
            Ordering::Greater => println!("Too big") ,
            Ordering::Equal => {
                println!("You win!!!");
                break;
            },

        }
    }

}
