use rust_decimal::prelude::*;
use rust_decimal_macros::dec;

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
    if years == 0 {
        return principal;
    }
    let months = Decimal::from(years * 12);
    if months.is_zero() {
        return principal;
    }
    let monthly_rate = (annual_rate_percent / dec!(100)) / dec!(12);

    if monthly_rate.is_zero() {
        return principal / months;
    }

    let numerator = principal * monthly_rate;
    let one = dec!(1);
    let base = one + monthly_rate;
    let exponent = -months;
    let denominator = one - base.powd(exponent);

    numerator / denominator
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn test_calculate_monthly_payment() {
        // Test case 1: Standard loan
        let principal = dec!(200000);
        let annual_rate_percent = dec!(5.0);
        let years = 30;
        let payment = calculate_monthly_payment(principal, annual_rate_percent, years);
        assert_eq!(payment.round_dp(2), dec!(1073.64));

        // Test case 2: Zero interest rate
        let principal = dec!(100000);
        let annual_rate_percent = dec!(0.0);
        let years = 10;
        let payment = calculate_monthly_payment(principal, annual_rate_percent, years);
        assert_eq!(payment.round_dp(2), dec!(833.33));

        // Test case 3: Zero years
        let principal = dec!(50000);
        let annual_rate_percent = dec!(7.5);
        let years = 0;
        let payment = calculate_monthly_payment(principal, annual_rate_percent, years);
        assert_eq!(payment.round_dp(2), dec!(50000));

        // Test case 4: Zero principal
        let principal = dec!(0);
        let annual_rate_percent = dec!(5.0);
        let years = 20;
        let payment = calculate_monthly_payment(principal, annual_rate_percent, years);
        assert_eq!(payment.round_dp(2), dec!(0));
    }
}
