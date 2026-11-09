fn main() {
    /*
        Rust have three types of loops:
        * loop
            The loop keyword tells Rust to execute a block of code over and over again
            either forever or until you explicitly tell it to stop
        * while
        * for
    */

    // Repeating code with loop
    /*
    loop {
        println!("Again");
    }
     */

    // Returning value from loops
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("{result}");

    /*
    Disambiguating with Loop Labels
    If you have loops within loops, break and continue apply to the innermost
    loop at that point. You can optionally specify a loop label on a loop that you
    can then use with break or continue to specify that those keywords apply
    to the labeled loop instead of the innermost loop. Loop labels must begin
    with a single quote. Here’s an example with two nested loops:

     */

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remainig = 10;
        loop {
            println!("remaining = {remainig}");
            if remainig == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remainig -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");

    // while loop

    let mut num = 3;
    while num != 0 {
        println!("{num}");
        num -= 1;
    }

    let arr = [1, 2, 3, 4, 5];
    for element in arr {
        println!("{element}");
    }
}
