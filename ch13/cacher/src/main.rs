mod cacher;

use std::thread;
use std::time::Duration;

use rand::Rng;

use cacher::Cacher;

fn main() {
    let simulated_user_specified_value = rand::thread_rng().gen_range(1..51);
    let simulated_random_number = rand::thread_rng().gen_range(1..10);

    println!("intensity: {}, random: {}", simulated_user_specified_value, simulated_random_number);

    generate_workout(simulated_user_specified_value, simulated_random_number);
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}