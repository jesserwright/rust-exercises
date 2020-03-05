pub fn run() -> &'static str {
    let board = [["O", "", "O"], ["", "O", ""], ["O", "", ""]];

    for (i, row) in board.iter().enumerate() {
        // row
        if row[0] == row[1] && row[0] == row[2] {
            return row[0];
        }
        // column
        if board[0][i] == board[1][i] && board[0][i] == board[2][i] {
            return board[0][i];
        }
    }

    // right cross
    if board[0][0] == board[1][1] && board[0][0] == board[2][2] {
        return board[0][0];
    }
    // left cross
    if board[0][2] == board[1][1] && board[0][2] == board[2][0] {
        return board[0][2];
    }

    "cat's game"
}

// TODO: add tests
