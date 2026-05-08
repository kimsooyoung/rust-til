// What are Structs?
// - Structs are custom data types that group related data together
// - Regular structs use named fields; tuple structs use positional fields; unit structs have no fields
// - Structs can have methods defined using impl blocks
// - There are three kinds: regular structs, tuple structs, and unit-like structs

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

    // &mut self: mutable borrow so we can change balance
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

    // &self: shared (immutable) borrow; we only read fields
    fn check_balance(&self) {
        println!(
            "Balance for {} is {}, is active: {}",
            self.owner, self.balance, self.is_active
        );
    }
}

// Struct update syntax: set some fields, copy the rest from `account` with `..account`
// (this moves `account`; the caller no longer owns it afterward)
fn build_fake_account(account: BankAccount) -> BankAccount {
    BankAccount {
        owner: "Fake Account".to_string(),
        ..account
    }
}

// Tuple struct: a distinct type from (i32, i32, i32); fields are self.0, self.1, self.2
#[derive(Debug)]
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

    // `mut`: needed for field assignment, methods that take &mut self, and passing owned value to build_fake_account
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

    // Fake account via struct update syntax (my_account is moved into the function)
    let fake_account = build_fake_account(my_account);
    fake_account.check_balance();

    // =========================================================================
    // Tuple Struct
    // =========================================================================
    // - Similar to tuples but with a type name (not interchangeable with (i32, i32, i32))
    // - `#[derive(Debug)]` on Position enables printing with {:?}
    let position = Position(10, 20, 30);
    println!("position debug: {:?}", position);
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
    // Unit-like struct
    // =========================================================================
    // - Structs without any fields
    // - Useful for implementing traits on types that don't need to store data
    #[derive(Debug)]
    struct UnitLikeStruct;
    let unit_like_struct = UnitLikeStruct;
    println!("unit_like_struct: {:?}", unit_like_struct);
}
