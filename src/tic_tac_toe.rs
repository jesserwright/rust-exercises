pub fn run() {
  let board = [["X", "", "X"], ["X", "O", "X"], ["", "", ""]];

  for row in board.iter() {
    col1 = [row[0], row[0], row[0]];
    println!("{:?}", col1[0]);
  }
}
