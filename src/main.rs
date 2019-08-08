// mod averager;
// use averager::{mean, median, mode};
// mod company;
// mod pig_latin;
// mod todo;
mod tic_tac_toe;
use tic_tac_toe::run;

fn main() {
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
  run();
}
