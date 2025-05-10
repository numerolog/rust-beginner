

use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use rand::random;

fn main()
{
    println!("Hello, world!");
    let numbers = [1, 3, 4];
    for mut i in numbers {
        println!("i={}", i);
        i *= 2;
        println!("\tHello, world! {}", i);
        if (i == 2)
        {
            println!("\teq2");
        }
        if i % 2 == 0
        {
            println!("\teven");
        }
        if (i == 6)
        {
            subrutine();
        } else 
        {
            println!("\tno subrutine call");
        }
    }
}

fn subrutine() {
    // todo!()

    println!("HELLO FROM SUBRUTINE");
}




