
const STARTING_MISSILES: i32 = 8;
const READY_AMOUNT: i32 = 2;

fn main() {
    //let mut missiles: i32 = STARTING_MISSILES;
    //let ready: i32 = READY_AMOUNT;
    let (mut missiles, ready): (i32, i32) = (STARTING_MISSILES, READY_AMOUNT);
    println!("Firing {ready} of my {missiles} missiles");
    missiles = missiles - ready;
    println!("{missiles} missiles left");

    // shadowing
    let x = 5;
    {
        let x = 99;
        // first value of x is not accessible from inner block once x is shadowed
        println!("x: {x}");
    }
    // inner x is dropped
    println!("x: {x}");
}
