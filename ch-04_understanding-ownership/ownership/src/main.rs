fn main() {

    // 1.
    // ---------------------------------
    // let mut s0 = String::from("hello");
    // s0.push_str(", world!"); // push_str() appends a literal to a String
    // println!("{}", s0); // This will print `hello, world!`

    // 2.
    // ---------------------------------
    // {
    //     let s1 = String::from("hello");  // s is valid from this point forward
    //     // do stuff with s1
    // }                                   // this scope is now over, and s is no
    //                                     // longer valid
    
    // 3. This is like a shallow copy.
    // Note: This code does not compile!
    // ---------------------------------
    // let s1 = String::from("hello");
    // let s2 = s1;

    // println!("{}, world!", s1);

    // 4.
    // ---------------------------------
    // let s1 = String::from("hello");
    // let s2 = s1.clone();

    // println!("s1 = {}, s2 = {}", s1, s2);

    // 5. But this code works!?!?
    // ---------------------------------
    // This is bc integer types are stored in the stack.
    // let x = 5;
    // let y = x;

    // println!("x = {}, y = {}", x, y);


    // 6. Ownership and Functions
    // ---------------------------------
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);      // s's value moves into the function...
                                    // ... and so is no longer valid here

    // print!("This will not work s = {}", s);     // Does not work

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it's okay to still
                                    // use x afterward

    print!("This works: x = {}", x);

}

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

