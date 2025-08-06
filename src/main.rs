use rust_decimal::prelude::*;
use std::env;
use std::process;
use verband::calculate_monthly_payment;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        eprintln!(
            "Usage: {} <principal> <annual_rate_percent> <years>",
            args[0]
        );
        process::exit(1);
    }

    let principal = match Decimal::from_str(&args[1]) {
        Ok(val) => val,
        Err(_) => {
            eprintln!("Invalid principal amount: {}", args[1]);
            process::exit(1);
        }
    };

    let annual_rate = match Decimal::from_str(&args[2]) {
        Ok(val) => val,
        Err(_) => {
            eprintln!("Invalid annual rate: {}", args[2]);
            process::exit(1);
        }
    };

    let years = match args[3].parse::<u32>() {
        Ok(val) => val,
        Err(_) => {
            eprintln!("Invalid years: {}", args[3]);
            process::exit(1);
        }
    };

    let payment = calculate_monthly_payment(principal, annual_rate, years);
    println!("Monthly payment: {}", payment.round_dp(2));
}
