use std::env;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>>{
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        println!("-s         To obtain a shared session key given an x(your private key),");
        println!("           y(someones' public key), and a common prime q.");
        println!("-p         To obtain your public key y given x(your private key), ");
        println!("           a(a primitive root mod q), and a prime q");
    } else if args[1] == "-p" {
        if args.len() < 5 {
            println!("Command must provide 3 arguments: x(your private key), a, q.");
        } else {
            let x: u64 = args[2].parse()?;
            let alpha: u64 = args[3].parse()?;
            let q: u64 = args[4].parse()?;

            let pub_key = get_public_key(x, alpha, q)?;
            println!("Your Public Key is: {}", &pub_key);
        }
    } else if args[1] == "-s" {
        if args.len() < 5 {
            println!("Command must provide 3 arguments: x(your private key), y(others' public key), q.");
        } else {
            let x: u64 = args[2].parse()?;
            let y: u64 = args[3].parse()?;
            let q: u64 = args[4].parse()?;

            let shared_key = shared_session_pub(&x, &y, &q)?;
            println!("The shared key is: {}", &shared_key);
        }
    } else {
        println!("Use: command <-s|-p> x(your private key) a|y q");
    }
    Ok(())
}

fn fast_exp(base: &u64, exp: &u64, q: &u64) -> Result<u64, String> {
    if *exp == 0 {
        return Ok(1 as u64)
    } else if *exp % 2 == 0 {
        return Ok(fast_exp(&((base * base) % q), &(exp / 2), q)?)
    } else if *exp % 2 == 1 {
        return Ok((base * fast_exp(base, &(exp - 1), q)?) % q)
    }
    Err(String::from("Exponente Invalido."))
}

fn get_public_key(x: u64, a: u64, q: u64) -> Result<u64, String> {
    Ok(fast_exp(&a, &x, &q)? % q)
}


fn shared_session_pub(my_priv: &u64, their_pub: &u64, q: &u64) -> Result<u64, String> {
    Ok(fast_exp(their_pub, my_priv, q)?)
}