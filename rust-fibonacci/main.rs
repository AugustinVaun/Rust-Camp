
fn main(){

    println!("Fibonacci Series");
    println!("Please enter a number to end the series: ");

    let first = 0;
    let second = 1;
    let mut end = 0;

    let mut input = String::new();

    std::io::stdin().read_line(&mut input).expect("Failed to read line");

    end = input.trim().parse().expect("Please type a number!");

    for n in 1..end {
        println!("{}", first);
        let next = first + second;
        first = second;
        second = next;
    }
}