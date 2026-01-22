use std::cell::Cell;
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

struct BoxedValue<T> {
    value: T,
}

impl<T> BoxedValue<T> {
    fn new(value: T) -> Self {
        BoxedValue { value }
    }
}

impl<T> Deref for BoxedValue<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

// Parameter is not a BoxedValue, but a reference to an i32
fn print_value(value: &i32) {
    println!("value: {}", value);
}

struct Person {
    name: String,
    age: Cell<i32>,
}

impl Person {
    fn increase_age(&self) -> i32 {
        self.age.set(self.age.get() + 1);
        self.age.get()
    }
}

fn main() {
    // Box is used to create a pointer to a value on the heap
    // most values are usually stored on the stack,
    // but some values are stored on the heap
    let age = Box::new(22);
    let twice = *age * 2;
    println!("twice: {}", twice);

    // Implementing Deref for a custom type
    //
    // Difference between deref and * operator
    // * Provides access to the value being pointed to by copying the value.
    // deref() Returns an immutable reference to the inner value.
    let age_boxed = BoxedValue::new(22);
    let twice_boxed_deref = age_boxed.deref() * 2;
    let twice_boxed_ast = *age_boxed * 2;
    println!("twice_boxed_deref: {}", twice_boxed_deref);
    println!("twice_boxed_ast: {}", twice_boxed_ast);

    // Pointers with Functions
    let my_value = BoxedValue::new(10);
    print_value(&my_value);

    // RC Counter
    // Count the number of references to a value
    let arr = vec!["Hello".to_string(), "World".to_string(), "Rust".to_string()];
    // Strong Count: When Rc::new(arr) is called,
    // the arr vector is moved onto the heap, and the strong reference count
    // for this allocated memory is initialized to 1.
    let rc = Rc::new(arr);
    // A Weak<T> pointer is a non-owning reference.
    // This is the critical distinction:
    // a Weak pointer does not contribute to the strong reference count,
    // meaning it will not prevent the inner value (arr) from being dropped
    let _weak = Rc::downgrade(&rc);
    drop(rc);
    // This will cause a panic because the inner value (arr) is already dropped
    // let value = _weak.upgrade().unwrap();
    // println!("value: {:?}", value);

    let new_arr = vec!["Hello".to_string(), "World".to_string(), "Rust".to_string()];
    let rc = Rc::new(new_arr);
    let rc2 = rc.clone();
    drop(rc);
    println!("rc2: {:?}", rc2);

    // Rc cannot modify the inner value
    // Rust provides Cell, which allows you to modify the inner value
    let me = Person {
        name: String::from("Sooyoung"),
        age: Cell::new(28),
    };
    // Even for Immutable struct, you can modify the inner value
    // because Cell is a wrapper around a mutable value
    // Which is really dangerous because it can cause data races
    let age = me.increase_age();
    println!("age: {}", age);

    // RefCell is a mutable reference to a value on the heap
    let ref_cell = RefCell::new(vec![1, 2, 3]);

    let mut mutable_ref = ref_cell.borrow_mut();
    mutable_ref.push(10);
    println!("mutable_ref: {:?}", mutable_ref);

    // This will cause a panic because the inner value is already borrowed
    let len = ref_cell.borrow().len();
    println!("len: {}", len);
}
