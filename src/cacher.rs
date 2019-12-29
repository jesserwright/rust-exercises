use std::collections::HashMap;
use std::{thread, time};

pub struct Cacher<T, I, R>
where
    T: Fn(I) -> R,
{
    calculation: T,
    arg_map: HashMap<I, R>,
}

impl<T, I, R> Cacher<T, I, R>
where
    T: Fn(I) -> R,
{
    pub fn new(calculation: T) -> Cacher<T, I, R> {
        let hmap: HashMap<I, R> = HashMap::new();
        Cacher {
            calculation,
            arg_map: hmap,
        }
    }

    pub fn value(&mut self, arg: I) -> R {
        match self.arg_map.get(&arg) {
            Some(v) => *v,
            None => {
                let v = (self.calculation)(arg);
                self.arg_map.insert(arg, v);
                v
            }
        }
    }
}

fn main() {
    fn generate_workout(intensity: u32, random_number: u32) {
        let mut expensive_result = Cacher::new(|num| {
            println!("calculating slowly...");
            thread::sleep(time::Duration::from_secs(2));
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

    generate_workout(45, 43);
}

#[test]
fn call_with_different_values() {
    let mut c = Cacher::new(|a| a);

    let v1 = c.value(1);
    let v2 = c.value(2);

    assert_eq!(v2, 2);
}

// TODO: cacher needs to be able to take any types!
#[test]
fn check_generic_values() {
    let mut c = Cacher::new(|a| a);
}
