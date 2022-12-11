fn main() {
    // Mutable variable
    let mut x = 5;
    println!("The value of x is {x}");
    x = 6;
    println!("The value of x is {x}");
    // Const always immutable
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("{}", THREE_HOURS_IN_SECONDS);

    // Shadowing
    let _y = 10;

    let y = 10 + 5;

    {
        let y = y * 2;
        println!("The value of the y in this scope is {y}");
    }

    println!("The value of y is {y}");

    //Shadowing
    //Change the type of data
    let space = "  ";
    let space = space.len();
    println!("Shadowing {space}");
    // mut
    // Cannot change the type of data
    let mut space2 = "mutable space2";
    // Error
    // space2 = space2.len();
    // Right
    println!("Mutable {space2}");
    space2 = "mutated space2";
    println!("Mutable {space2}");
}
