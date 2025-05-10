use std::collections::HashMap;
/*
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
*/

fn subroutine_6()
{
    println!("HELLO FROM SUBRUTINE 6");
}

fn subroutine_7()
{
    println!("HELLO FROM SUBRUTINE 7");
}
fn subroutine_8()
{
    println!("\tHELLO FROM SUBRUTINE 8");
}

fn subroutine_9()
{
    println!("\tHELLO FROM SUBRUTINE 8");
}

fn subroutine_n()
{
    println!("\tHELLO FROM SUBRUTINE N");
}

fn subroutine_mutter_substract4_and_mut_to_20(i: &mut usize) -> usize
{
    let ret = *i - 4;
    *i = 20;
    return ret;
}

fn main()
{
    println!("Hello, world!");
    let numbers = [1, 2, 3, 4, 6, 7, 8, 9];
    // [(usize, fn()); 3]
    // let functions = [(6, subrutine6), (7, subrutine7), (8, subrutine8)];
    let array_of_tuple_funcs: [(usize, fn()); 3] = [(6, subroutine_6), (7, subroutine_7), (8, subroutine_8)];
    let mut kv_of_funcs : HashMap<usize, fn()> = HashMap::new();
    kv_of_funcs.insert(6, subroutine_6);
    kv_of_funcs.insert(7, subroutine_7);
    kv_of_funcs.insert(8, subroutine_8);

    kv_of_funcs.insert(9, subroutine_9);
    kv_of_funcs.remove(&9);


    let mut kv_of_mut_funcs : HashMap<usize, fn(&mut usize) -> usize> = HashMap::new();
    kv_of_mut_funcs.insert(18, subroutine_mutter_substract4_and_mut_to_20);


    for mut i in numbers
    {
        println!("i={}", i);

        if i == 2
        {
            println!("\teq2");
        }
        if i % 2 == 0
        {
            println!("\teven");
        }
        if i == 6
        {
            subroutine_6();
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


        has_sub_call = false;

        let opt_sub =  kv_of_funcs.get(&i);
        let sub = opt_sub.unwrap_or(&(subroutine_n as fn()));
        if opt_sub.is_none()
        {
            println!("\tkv no subrutine func 1");
            // opt_sub.expect("no subrutine func");
        }
        else if opt_sub.is_some()
        {
            println!("\tkv has subrutine func");
            has_sub_call = true;
        }
        else
        {
            println!("\tkv no subrutine func 2");
        }
        sub();
        
        
        if has_sub_call
        {
            println!("\tno kv subrutine call");
        }

        {
            i *= 2;
            println!("\tHello, world! {}", i);
        }


        {
            let mutter = kv_of_mut_funcs.get(&i);
            if mutter.is_some()
            {
                let mut ret = mutter.unwrap()(&mut i);
                println!("\ti={}", i);
                println!("\tret={}", ret);
                ret = mutter.unwrap()(&mut i);
                println!("\ti={}", i);
                println!("\tret={}", ret);
            }
        }

    }
}





