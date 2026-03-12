mod bank_account;

use bank_account::BankAccount;

fn main() {
    let mut account = BankAccount::new(100.0);

    account.deposit(50.0);
    account.withdraw(30.0);

    println!("Current balance: {}", account.balance());

    account.apply_interest(0.05);

    println!("Balance after interest: {}", account.balance());
}