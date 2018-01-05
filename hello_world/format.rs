fn main() {
    println!("{} oh shit", 31);

    println!("{0} more oh shit {1} {1} -- {0}", "alice", "bob");

    println!("{ohno} daaammn named {what}", ohno="ohhhh", what="arguments!");

    println!("me not binary: {} -- me binary: {:b}", 200, 200);

    // You can right-align text with a specified width. This will output
    // "     1". 5 white spaces and a "1".
    println!("{number:>width$}", number=1, width=6);

    // You can pad numbers with extra zeroes. This will output "000001".
    println!("{number:>0width$}", number=1, width=6);

    // It will even check to make sure the correct number of arguments are
    // used.
    println!("My name is {0}, {1} {0}", "Bond", "James");
    // FIXME ^ Add the missing argument: "James"
    
    // Create a structure which contains an `i32`. Name it `Structure`.
    #[allow(dead_code)]
    struct Structure(i32);

    // However, custom types such as this structure require more complicated
    // handling. This will not work.
    // println!("This struct `{}` won't print...", Structure(3));
    // FIXME ^ Comment out this line.

    let pi = 3.141592;
    println!("Pi is roughly {:.3}", pi);
}
