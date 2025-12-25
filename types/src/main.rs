fn print_string(s: &str) {
    println!("print_string: {}", s);
}

struct BankAccount {
    owner: String,
    balance: f64,
    is_active: bool,
}

impl BankAccount {
    fn new(owner: String, balance: f64) -> BankAccount {
        BankAccount { owner, balance, is_active: true }
    }

    // This should be mutable because we are calling the withdraw method
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

    // This should be immutable because we are not calling the withdraw method
    fn check_balance(&self) {
        println!("Balance for {} is {}, is active: {}", self.owner, self.balance, self.is_active);
    }
}

fn build_fake_account(account: BankAccount) -> BankAccount {
    BankAccount {
        owner: "Fake Account".to_string(),
        ..account
    }
}

fn main() {
    // signed / unsigned integers
    let x: i64 = 100;
    let y: u64 = 100;
    println!("signed x: {}, unsigned y: {}", x, y);

    // Array [T; N]
    // - Fixed size at compile time (size N is part of the type)
    // - Stored on the stack and owns its data
    // - [i32; 3] and [i32; 5] are different types
    let num_arr: [i32; 3] = [1, 2, 3];
    println!("num_arr: {:?}", num_arr);
    
    // Mix array is not allowed
    // let mix_arr = [1, 2, 3, "Hello", true];
    // println!("mix_arr: {:?}", mix_arr);

    let str_arr: [&str; 3] = ["Hello", "World", "Rust"];
    println!("str_arr: {:?}", str_arr);

    let str_arr: [String; 3] = ["Hello".to_string(), "World".to_string(), "Rust".to_string()];
    println!("str_arr: {:?}", str_arr);

    // Tuple
    let mix_tup: (i32, f32, bool) = (1, 2.0, true);
    println!("mix_tup: {:?}", mix_tup);

    // Slices &[T]
    // - Dynamically sized (size is NOT part of the type)
    // - Always a reference (borrowed view) into existing data
    // - Stored as a "fat pointer" (pointer + length) on the stack
    // - More flexible for function params: fn foo(nums: &[i32]) accepts any array size
    let num_slice: &[i32] = &[1, 2, 3, 4, 5];
    println!("num_slice: {:?}", num_slice);

    let str_slice: &[&str] = &["Hello", "World", "Rust"];
    println!("str_slice: {:?}", str_slice);

    // String vs String Slice (&str)
    let mut my_string = String::from("Hello, world!");
    my_string.push_str(", Rust!");
    println!("my_string: {}", my_string);

    let my_string_slice: &str = &my_string[0..5];
    // my_string.push_str(", Rust!");
    print_string(my_string_slice);

    // =========================================================================
    // Struct
    // =========================================================================

    // This should be mutable because we are calling the withdraw method
    let mut my_account = BankAccount::new("Sooyoung Kim".to_string(), 1500.0);
    let mut mom_account = BankAccount {
        owner: "Mom".to_string(),
        balance: 0.0,
        is_active: true,
    };

    my_account.check_balance();
    mom_account.check_balance();
    
    my_account.withdraw(500.0);
    mom_account.deposit(500.0);

    my_account.check_balance();
    mom_account.check_balance();

    my_account.owner = "Daniel Kim".to_string();
    my_account.deactivate();
    my_account.check_balance();
    
    // make fake accounts
    let fake_account = build_fake_account(my_account);
    fake_account.check_balance();

    // Tuple Struct 
    // This derives the Debug trait so that Position can be printed using {:?}
    #[derive(Debug)]
    struct Position(i32, i32, i32);
    let position = Position(10, 20, 30);
    println!("position: {:?}", position);

    // Unit Like Struct
    #[derive(Debug)]
    struct UnitLikeStruct;
    let unit_like_struct = UnitLikeStruct;
    println!("unit_like_struct: {:?}", unit_like_struct);

    // =========================================================================
    // Variables with Control Flow
    // =========================================================================

    let condition = true;
    let cond_var = if condition {5} else {6};
    println!("cond_var: {}", cond_var);

    // This does not work because the else block must return the same type as the if block
    // let cond_var_incorrect = if condition {
    //     "Hello"
    // } else {
    //     5
    // };
    
}
