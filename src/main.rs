use rust_decimal::prelude::*;
use rust_decimal_macros::dec;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        eprintln!("Usage: {} <principal> <annual_rate_percent> <years>", args[0]);
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

/// Calculates the monthly payment for a loan.
/// 
/// # Arguments
/// * `principal` - The loan amount (capital).
/// * `annual_rate_percent` - The yearly interest rate as a percent (e.g., 5.0 for 5%).
/// * `years` - The number of years for the loan.
/// 
/// # Returns
/// The monthly payment as a Decimal.
pub fn calculate_monthly_payment(
    principal: Decimal,
    annual_rate_percent: Decimal,
    years: u32,
) -> Decimal {
    let months = Decimal::from(years * 12);
    let monthly_rate = (annual_rate_percent / dec!(100)) / dec!(12);

    if monthly_rate.is_zero() {
        return principal / months;
    }

    let numerator = principal * monthly_rate;
    let base = (dec!(1) + monthly_rate).to_f64().unwrap();
    let exponent = -months.to_f64().unwrap();
    let denominator = dec!(1) - Decimal::from_f64(base.powf(exponent)).unwrap();

    numerator / denominator
}
