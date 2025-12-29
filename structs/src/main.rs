// What are Structs?
// - Structs are custom data types that group related data together
// - They allow you to create your own types with named fields
// - Structs can have methods defined using impl blocks
// - There are three types of structs: regular structs, tuple structs, and unit-like structs

// Regular Struct
// - Has named fields
// - Each field can have different types
struct BankAccount {
    owner: String,
    balance: f64,
    is_active: bool,
}

impl BankAccount {
    // Associated function (like a constructor)
    fn new(owner: String, balance: f64) -> BankAccount {
        BankAccount {
            owner,
            balance,
            is_active: true,
        }
    }

    // This should be mutable because we are modifying the balance
    fn withdraw(&mut self, amount: f64) {
        self.balance -= amount;
        println!("Withdrew {} from {}", amount, self.owner);
    }

    fn deposit(&mut self, amount: f64) {
        self.balance += amount;
        println!("Deposited {} to {}", amount, self.owner);
    }

    fn activate(&mut self) {
        self.is_active = true;
        println!("Activated account for {}", self.owner);
    }

    fn deactivate(&mut self) {
        self.is_active = false;
        println!("Deactivated account for {}", self.owner);
    }

    // This should be immutable because we are only reading, not modifying
    fn check_balance(&self) {
        println!(
            "Balance for {} is {}, is active: {}",
            self.owner, self.balance, self.is_active
        );
    }
}

// Function that creates a new struct using struct update syntax
fn build_fake_account(account: BankAccount) -> BankAccount {
    BankAccount {
        owner: "Fake Account".to_string(),
        ..account
    }
}

// Tuple Struct
struct Position(i32, i32, i32);

impl Position {
    fn twice(&self) -> Position {
        Position(self.0 * 2, self.1 * 2, self.2 * 2)
    }

    fn make_twice(&mut self) {
        self.0 *= 2;
        self.1 *= 2;
        self.2 *= 2;
    }

    fn describe(&self) {
        println!("Position is at ({}, {}, {})", self.0, self.1, self.2);
    }

    fn zero() -> Position {
        Position(0, 0, 0)
    }
}

fn main() {
    // =========================================================================
    // Regular Struct
    // =========================================================================

    // This should be mutable because we are calling methods that modify the struct
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

    // make fake accounts using struct update syntax
    let fake_account = build_fake_account(my_account);
    fake_account.check_balance();

    // =========================================================================
    // Tuple Struct
    // =========================================================================
    // - Similar to tuples but with a name
    // - Useful when you want to give a tuple a name and make it a distinct type
    // - This derives the Debug trait so that Position can be printed using {:?}
    let position = Position(10, 20, 30);
    position.describe();
    let mut position_twice = position.twice();
    position_twice.describe();
    position_twice.make_twice();
    position_twice.describe();

    let position2 = Position::zero();
    let position3 = Position::zero();
    let position4 = Position::zero();

    position2.describe();
    position3.describe();
    position4.describe();

    // =========================================================================
    // Unit Like Struct
    // =========================================================================
    // - Structs without any fields
    // - Useful for implementing traits on types that don't need to store data
    #[derive(Debug)]
    struct UnitLikeStruct;
    let unit_like_struct = UnitLikeStruct;
    println!("unit_like_struct: {:?}", unit_like_struct);
}
