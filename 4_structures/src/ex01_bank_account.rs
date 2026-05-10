//! Topic 1 — Named-field struct `BankAccount`, methods, struct update syntax, and moves.
//!
//! Run via: `cargo run --bin ex01_bank_account`

use crate::bank_account::{build_fake_account, BankAccount};

pub fn run() {
    let mut my_account = BankAccount::new("Sooyoung Kim".to_string(), 1500.0);
    let mut mom_account = BankAccount {
        owner: "Mom".to_string(),
        balance: 0.0,
        is_active: true,
    };

    my_account.activate();
    mom_account.activate();

    my_account.check_balance();
    mom_account.check_balance();

    my_account.withdraw(500.0);
    mom_account.deposit(500.0);

    my_account.check_balance();
    mom_account.check_balance();

    my_account.owner = "Daniel Kim".to_string();
    my_account.deactivate();
    my_account.check_balance();

    let fake_account = build_fake_account(my_account);
    fake_account.check_balance();
}
