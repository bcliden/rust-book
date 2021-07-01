use rand::Rng;

pub fn add_one(x: i32) -> i32 {
    x + 1
}

pub fn add_two(x: i32) -> i32 {
    x + 2
}

pub fn add_rand(x: i32) -> i32 {
    let num = rand::thread_rng().gen_range(1..101);
    x + num
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_adds_one() {
        assert_eq!(3, add_one(2));
    }

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn it_adds_random() {
        assert_ne!(6, add_rand(6));
    }
}