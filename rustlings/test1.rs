// test1.rs
// This is a test for the following sections:
// - Variables
// - Functions

// Mary is buying apples. One apple usually costs 2 dollars, but if you buy
// more than 40 at once, each apple only costs 1! Write a function that calculates
// the price of an order of apples given the order amount.

fn calculateprice(num: i32) -> i32 {
    let prc = if num>40 {1} else {2};
    prc * num
}

fn main() {
    let price1 = calculateprice(55);
    let price2 = calculateprice(40);

    // Don't modify this!
    if price1 == 55 && price2 == 80 {
        println!("Good job!");
    } else {
        panic!("Uh oh! Wrong price!");
    }
}
