// mod averager;
// use averager::{mean, median, mode};
// mod company;
// mod pig_latin;
// mod todo;
// mod tic_tac_toe;
// use tic_tac_toe::run;

// company::company();
// todo::todo_app();
// let numbers: Vec<usize> = [3, 32, 3, 4, 3, 2, 554].to_vec();
// mean(numbers);
// let mut numbers2: Vec<usize> = [1, 22, 3, 4, 5].to_vec();
// median(&mut numbers2);
// let numbers3: Vec<usize> = [1, 22, 3, 4, 5].to_vec();
// mode(numbers3);
// pig_latin::pig_latin("esse-jay");
// assert_eq!("esse-jay", pig_latin::pig_latin("jesse"));
// assert_eq!("apple-hay", pig_latin::pig_latin("apple"));

use cacher::Cacher;
use std::{thread, time};

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
