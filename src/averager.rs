use std::collections::HashMap;

pub fn mean(numbers: Vec<usize>) {
  // Mean: (the average value)
  let mut total = 0;
  for number in &numbers {
    total += number
  }
  let numbers_length = numbers.len();
  println!("Mean: {}", total / numbers_length);
}

pub fn median(numbers: &mut Vec<usize>) {
  // Median (when sorted, the value in the middle position)
  let numbers_length = numbers.len();
  numbers.sort();
  if numbers_length % 2 == 1 {
    let median_index = (numbers_length - 1) / 2;
    println!("Median: {}", numbers[median_index]);
  } else {
    println!("Even Array, no median!");
  }
}

pub fn mode(numbers: Vec<usize>) {
  // Mode (the value that occurs most often; a hash map will be helpful here) of the list.
  let mut mode = HashMap::new();
  for number in &numbers {
    let count = mode.entry(number).or_insert(0);
    *count += 1;
  }
  println!("Mode: {:#?}", mode);
}

// TODO: add tests
