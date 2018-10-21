fn main() {
    /// Rust's central feature is ownership
    /// 
    /// Memory is managed through a set of compiler checked
    /// rules for ownership
    /// 
    /// Stack and Heap are parts of memory available
    /// to the code at runtime
    /// 
    /// Stack - last in, first out. size constrained. fast
    /// Heap - allocated space for unknown sizes with pointers. slower
    /// 
    /// Ownership is for managing heap data. Like GC
    /// 
    /// Rules
    /// - Each value has a variable called its owner
    /// - Only one owner at a time
    /// - When the owner goes out of scope, the value will be dropped

    {                       // s is not in scope - invalid
        let _s = "helllo";  // s is a string literal - immutable
                            // s is in scope - valid
    }                       // s no longer in scope - invalid

    {
        let mut s = String::from("hello");  // mutable and memory on heap
        s.push_str(", world!");             // string takes more memory
        println!("{}", s);
    }                                       // out of scope - memory returned

    /// String made of 3 parts stored on the Stack:
    /// 1. pointer
    /// 2. length
    /// 3. capacity
    /// 
    /// String contents are stored on the Heap with:
    /// 1. index
    /// 2. value
    /// 
    /// String Stack data can be copied and point to the same Heap contents
    /// Rust invalidates the first variable (pointer)
    /// 
    /// You can use a clone method to copy heap data

    let s1  = String::from("heyo");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);

    /// Integers have a known size, are stored on the stack
    /// Therefore, copies are quick and allowed
    /// Based on a Copy trait. see ch10
    
    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);

    let __s = String::from("hello"); // __s into scope

    takes_ownership(__s); // value of __s moved into function. invalid in main

    // println!("ERROR ERROR: {}", __s)

    let __x = 5; // __x into scope

    makes_copy(__x); // value of __x moved into function as Copy

    println!("WORKS: {}", __x);

    let v1 = gives_ownership();

    println!("Thanks for the ownership: {}", v1);

    let v2 = String::from("hello");
    let v3 = takes_and_gives_back(v2);

    println!("Thanks for taking and returing: {}", v3);

    let o1 = String::from("hello");
    let (o2, len) = calculate_length(o1);

    println!("The length of '{}' is {}", o2, len);

    /// References and Borrowing
    /// Functions can take references instead of ownership
    /// & is used for references
    /// * is used for the opposite, dereferencing. see ch8 and ch15
    /// References are immutable by default
    
    let b1 = String::from("hello there");
    let blen = calculate_length_borrow(&b1);

    println!("The length of '{}' is {}", b1, blen);

    /// Initial variable can be made mutable
    /// The function can take a mutable reference
    /// Restriction - only one mutable reference per scope
    /// Another Restriction - cannot combine mut and immutable refs

    let mut r = String::from("hello");

    println!("Not Mutated: {}", r);
    change_ref(&mut r);
    println!("Mutated: {}", r);

    /// Curly braces can create new scopes for multiple mutable refs
    {
        let r1 = &mut r;
        println!("New scoped: {}", r1);
    }

    let r2 = &mut r;
    println!("Original scoped: {}", r2);

    /// Dangling References / Pointers
    /// Pointer referencing location in memory given to someone else
    /// Rust ensures data does not go out of scope before ref to data does
    
    // let reference_to_nothing = dangle();

    let reference_to_something = no_dangle();

    /// Rules of References
    /// 1. You can have either one mutable reference or any number of immutable
    /// 2. References must always be valid


}

fn takes_ownership(some_string: String) {
    println!("some string: {}", some_string);
} //some_string out of scope, 'drop' called, mem freed

fn makes_copy(some_integer: i32) {
    println!("some integer: {}", some_integer);
} // some_integer out of scope, nothing special

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
} // some_string moved to calling function

fn takes_and_gives_back(a_string: String) -> String {
    a_string
} // a string is given by calling function
// and returned to calling function

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // returns length of the string
    (s, length)
}

fn calculate_length_borrow(s: &String) -> usize {
    s.len()
}

fn change_ref(some_string: &mut String) {
    some_string.push_str(", world");
}

// fn dangle() -> &String { // returns a ref to a string
    // let s = String::from("hello"); // new string
    // &s // return a ref to s which goes out of scope when function is done
// }

fn no_dangle() -> String {
    let s = String::from("hello");
    s
}
