use std::collections::HashMap;
use std::io;
mod todo;
use todo::todo_app;
mod averager;

fn main() {
  company();
  todo::todo_app();

  // mean_median_mode()
  // assert_eq!("esse-jay", pig_latin("jesse"));
  // assert_eq!("apple-hay", pig_latin("apple"));
}

fn company() {
  // Using a hash map and vectors, create a text interface to allow a user to add
  // employee names to a department in a company. For example, “Add Sally to
  // Engineering” or “Add Amir to Sales.” Then let the user retrieve a list of
  // all people in a department or all people in the company by department, sorted alphabetically.
  // "Get All" and "Get [DEPARTMENT]"
  let mut departments = HashMap::new();
  departments.insert(Some("Sales"), Some("Chuck"));
  departments.insert(Some("R&D"), Some("Corey"));
  let mut input = String::new();

  println!("Input \"All\" to get all users.");
  println!("Input \"Add [USER] to [DEPARTMENT]\" to add user.");
  io::stdin().read_line(&mut input).expect("Failed");
  let mut commands = input.split_whitespace();
  let first_command = commands.next();
  let second_command = commands.next();
  let third_command = commands.next();
  let forth_command = commands.next();
  if first_command == Some("Add") && third_command == Some("to") {
    departments.insert(forth_command, second_command);
    println!(
      "{} added to {}",
      second_command.unwrap(),
      forth_command.unwrap()
    );
  }
  if first_command == Some("All") {
    println!("All Departments: {:?}", departments);
  }
}

// fn pig_latin(word: &str) -> String {
// Convert strings to pig latin. The first consonant of each word is moved to the end of the
// word and “ay” is added, so “first” becomes “irst-fay.” Words that start with a vowel have “hay”
// added to the end instead (“apple” becomes “apple-hay”). Keep in mind the details about UTF-8 encoding!
// think of the String type as a vector of the str type
//   let vowels = ["a", "e", "i", "o", "u"];

//   let mut word = String::from(word);

//   let first_letter = &word[..1];

//   for vowel in vowels.iter() {
//     if first_letter == vowel.to_string() {
//       word.push_str(&"-hay");
//       break;
//     } else {
//       let first_removed = word.remove(0).to_string();
//       word.push_str(&"-");
//       word.push_str(&first_removed);
//       word.push_str(&"ay");
//       break;
//     }
//   }
//   word // 🖐️🎤🔥
// }
