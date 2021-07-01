use add_one;
use rand::Rng;

fn main() {
    let num = rand::thread_rng().gen_range(1..101);
    println!(
        "{} plus one is {}!",
        num,
        add_one::add_one(num)
    );

    let num2 = rand::thread_rng().gen_range(1..101);
    println!(
        "{} plus two is {}!",
        num2,
        add_one::add_two(num2)
    );

    println!(
        "{} plus a random number is {}!",
        1,
        add_one::add_rand(1)
    );
}
