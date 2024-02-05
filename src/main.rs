extern crate bit_vec;

use bit_vec::BitVec;

#[derive(Debug)]
struct BloomFilter {
    size: usize,
    bloom_filter: BitVec,
    number_hash_functions: usize
}

impl BloomFilter {
    pub fn new(size: usize, number_hash_functions: usize) -> Self {
        let bloom_filter = BloomFilter {
            size,
            bloom_filter: BitVec::from_elem(size, false),
            number_hash_functions
        };

        bloom_filter
    }

    pub fn hash_string(&self, input: &mut String, number_hash_function: usize) -> usize {
        let mut hash: isize = 0;
        if input.len() == 0 {
            return hash.abs() as usize;
        }
        let number_hash_function_str = number_hash_function.to_string();

        input.push_str(number_hash_function_str.as_str());

        for char in input.chars() {
            hash = ((hash << 5) - hash) + char as isize;
        }

        (hash % self.size as isize).abs() as usize
    }

    pub fn add_to_filter(&mut self, mut item: String) {
        for i in 0..self.number_hash_functions {
            let index = self.hash_string(&mut item, i);
            self.bloom_filter.set(index, true);
        }
    }

    pub fn check_item_not_presented(&self, item: &mut String) -> bool {
        for i in 0..self.number_hash_functions {
            let index = self.hash_string(item, i);
            let is_exist_opt = self.bloom_filter.get(index);
            if let None = is_exist_opt {
                println!("is_exist_opt equal to 0");
                return false;
            }

            if is_exist_opt.unwrap() == true {
                return true;
            }
        }

        return false
    }
}

fn main() {
    let size = 10000;

    let mut s1 = "Apple".to_string();
    let mut s2 = "Pineapple".to_string();
    let mut s3 = "Banana".to_string();
    let mut s4 = "Raspberry".to_string();
    let mut s5 = "Strawberry".to_string();
    let mut s6 = "Orange".to_string();

    let mut bloom_filter = BloomFilter::new(size, 2);
    bloom_filter.add_to_filter("Apple".to_string());
    let is_exist_apple = bloom_filter.check_item_not_presented(&mut s1);
    let is_exist_pineapple = bloom_filter.check_item_not_presented(&mut s2);
    let is_exist_banana = bloom_filter.check_item_not_presented(&mut s3);
    let is_exist_raspberry = bloom_filter.check_item_not_presented(&mut s4);
    let is_exist_strawberry = bloom_filter.check_item_not_presented(&mut s5);
    bloom_filter.add_to_filter("Orange".to_string());
    let is_exist_orange = bloom_filter.check_item_not_presented(&mut s6);

    println!("{}", is_exist_apple);
    println!("{}", is_exist_pineapple);
    println!("{}", is_exist_banana);
    println!("{}", is_exist_raspberry);
    println!("{}", is_exist_strawberry);
    println!("{}", is_exist_orange);
}