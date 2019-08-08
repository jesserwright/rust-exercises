#[allow(dead_code)]
pub fn todo_app() {
  // use std::collections::HashMap;
  // let mut scores = HashMap::new();

  // scores.insert(String::from("Blue"), 10);

  // scores.entry(String::from("Yellow")).or_insert(50);
  // scores.entry(String::from("Blue")).or_insert(50);

  // println!("{:?}", scores);

  // let text = "hello world wonderful world";

  // let mut map = HashMap::new();

  // for word in text.split_whitespace() {
  //   let count = map.entry(word).or_insert(0);
  //   println!("{}, {}", count, word);
  //   *count += 1;
  // }

  // println!("{:?}", map);

  #[derive(Debug)]
  struct TodoItem {
    id: i32,
    title: String,
    reusable: bool,
  }

  impl TodoItem {
    fn new(title: String) -> TodoItem {
      TodoItem {
        id: 1,
        title,
        reusable: true,
      }
    }
  }

  let mut todo_list: Vec<TodoItem> = Vec::new();

  let todo1 = TodoItem {
    id: 1,
    title: String::from("take out trash"),
    reusable: false,
  };

  todo_list.push(todo1);

  let todo2 = TodoItem::new(String::from("jesse"));
  todo_list.push(todo2);

  let second_item: &TodoItem = &todo_list[1];

  println!("{:#?}", second_item);
}
