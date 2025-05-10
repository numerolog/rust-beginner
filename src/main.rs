use std::collections::HashMap;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use rand::random;

fn subrutine6() {
    // todo!()

    println!("HELLO FROM SUBRUTINE 6");
}

fn subrutine7() {
    // todo!()

    println!("HELLO FROM SUBRUTINE 7");
}
fn subrutine8() {
    // todo!()

    println!("\tHELLO FROM SUBRUTINE 8");
}

fn main()
{
    println!("Hello, world!");
    let numbers = [1, 2, 3, 4, 6, 7, 8];
    // [(usize, fn()); 3]
    // let functions = [(6, subrutine6), (7, subrutine7), (8, subrutine8)];
    let array_of_tuple_funcs: [(usize, fn()); 3] = [(6, subrutine6), (7, subrutine7), (8, subrutine8)];
    /*let kv_of_funcs : HashMap<usize, fn()> = {
        6 => subrutine6
    };*/
    
    
    
    for mut i in numbers {
        println!("i={}", i);
       
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
            subrutine6();
        } else
        {
            println!("\tno manual subrutine call");
        }
        
        /*
        if functions[i].0 == i
        {
            
        }*/
        let mut has_sub_call = false;
        for tuple in array_of_tuple_funcs
        {
            if tuple.0 == i 
            {
                has_sub_call = true;
                tuple.1();
                break;
            }
        }
        if !has_sub_call
        {
            println!("\tno tuple-array subrutine call");
        }
        
        {
            i *= 2;
            println!("\tHello, world! {}", i);
        }
        
        
    }
}





