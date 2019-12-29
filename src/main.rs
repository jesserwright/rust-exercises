mod averager;
mod company;
mod pig_latin;
mod tic_tac_toe;
mod todo;

fn main() {
  company::company();
  todo::todo_app();
  tic_tac_toe::run();
  let numbers: Vec<usize> = [3, 32, 3, 4, 3, 2, 554].to_vec();
  averager::mean(numbers);
  let mut numbers2: Vec<usize> = [1, 22, 3, 4, 5].to_vec();
  averager::median(&mut numbers2);
  let numbers3: Vec<usize> = [1, 22, 3, 4, 5].to_vec();
  averager::mode(numbers3);
  pig_latin::pig_latin("esse-jay");
}
