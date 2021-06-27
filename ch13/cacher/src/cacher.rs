use std::collections::HashMap;
use std::hash::Hash;
use std::clone::Clone;

pub struct Cacher<T, K, V> 
where
    T: Fn(K) -> V,
    K: Eq + Hash + Copy,
    V: Clone
{
    calculation: T,
    map: HashMap<K, V>
}

impl<T, K, V> Cacher<T, K, V>
where
    T: Fn(K) -> V,
    K: Eq + Hash + Copy,
    V: Clone
{
    pub fn new(calculation: T) -> Cacher<T, K, V> {
        Cacher {
            calculation,
            map: HashMap::new(),
        }
    }

    pub fn value(&mut self, k: K) -> V {
        // like the book:
        // match self.map.get(&k) {
        //     Some(v) => v.clone(),
        //     None => {
        //         let v = (self.calculation)(k);
        //         self.map.insert(k, v.clone());
        //         v.clone()
        //     }
        // }

        // or, use the cool entry API: https://doc.rust-lang.org/std/collections/hash_map/enum.Entry.html
        let v = self.map.entry(k).or_insert((self.calculation)(k));
        v.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn call_with_different_values() {
        let mut c = Cacher::new(|x| x);
        let one = c.value(1);
        let two = c.value(2);
        assert_ne!(one, two);
    }

    #[test]
    fn can_differ_types() {	
        let mut c = Cacher::new(|_a: u64| String::from("hello"));
        let v = c.value(53278214);
        assert_eq!(v, "hello");
    }
}