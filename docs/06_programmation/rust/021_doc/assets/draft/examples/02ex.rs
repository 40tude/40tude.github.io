// cargo run --example 02ex

fn main() {
    let numbers = vec![Some(1), Some(15), Some(25), None, Some(5)];
    let mut my_iter = numbers.iter();
    my_iter.next();
    println!("{:?}", my_iter.as_slice());
    println!("{:?}", numbers);
}
