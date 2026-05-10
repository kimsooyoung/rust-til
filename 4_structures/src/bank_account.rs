//! Named-field struct used by `ex01_bank_account.rs` — models a simple bank account.

pub struct BankAccount {
    pub owner: String,
    pub balance: f64,
    pub is_active: bool,
}

impl BankAccount {
    pub fn new(owner: String, balance: f64) -> BankAccount {
        BankAccount {
            owner,
            balance,
            is_active: true,
        }
    }

    pub fn withdraw(&mut self, amount: f64) {
        self.balance -= amount;
        println!("Withdrew {} from {}", amount, self.owner);
    }

    pub fn deposit(&mut self, amount: f64) {
        self.balance += amount;
        println!("Deposited {} to {}", amount, self.owner);
    }

    pub fn activate(&mut self) {
        self.is_active = true;
        println!("Activated account for {}", self.owner);
    }

    pub fn deactivate(&mut self) {
        self.is_active = false;
        println!("Deactivated account for {}", self.owner);
    }

    pub fn check_balance(&self) {
        println!(
            "Balance for {} is {}, is active: {}",
            self.owner, self.balance, self.is_active
        );
    }
}

pub fn build_fake_account(account: BankAccount) -> BankAccount {
    BankAccount {
        owner: "Fake Account".to_string(),
        ..account
    }
}
