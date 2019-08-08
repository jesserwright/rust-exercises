use std::collections::HashMap;
use std::io;
pub fn company() {
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
