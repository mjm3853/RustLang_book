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
