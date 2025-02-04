CLI Calculator v1.0

Overview

This is a simple command-line calculator written in Rust. It allows users to perform basic arithmetic operations (+, -, *, /) and logs the calculation history to a file (history.txt).

Features

Supports addition, subtraction, multiplication, and division

Handles division by zero errors

Logs all calculations to history.txt

Gracefully handles invalid inputs

Allows users to exit by typing exit

Prerequisites

Rust installed on your system (Install Rust)

Installation

Clone the repository:

git clone <repository_url>
cd cli_calculator_rust

Build the project:

cargo build --release

Usage

Run the calculator with:

cargo run

Enter calculations in the format:

<number> <operator> <number>

Example:

5 + 3

Output:

Result: 8

To exit, type:

exit

Logging

All calculations are saved in history.txt. The file is created if it does not exist and appended with new calculations.

License

This project is licensed under the MIT License.

Contributions

Contributions are welcome! Feel free to open issues or submit pull requests.

Author

Nicholas Connelly
