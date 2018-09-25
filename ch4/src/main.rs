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
}
