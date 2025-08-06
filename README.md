# Verband - Loan Payment Calculator

Verband is a simple command-line tool written in Rust to calculate the monthly payment for a loan based on the principal amount, annual interest rate, and loan term.

## Usage

To use the tool, provide the principal, annual interest rate (as a percentage), and the loan term in years as command-line arguments.

```bash
verband <principal> <annual_rate_percent> <years>
```

### Example

```bash
verband 100000 5 30
```

This will calculate the monthly payment for a $100,000 loan with a 5% annual interest rate over 30 years.

## Building

To build the project, you need to have Rust and Cargo installed.

1. Clone the repository:
   ```bash
   git clone <repository-url>
   ```
2. Navigate to the project directory:
   ```bash
   cd verband
   ```
3. Build the project in release mode:
   ```bash
   cargo build --release
   ```

The executable will be located at `target/release/verband`.
