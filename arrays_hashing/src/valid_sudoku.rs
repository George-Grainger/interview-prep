// Solution for: https://leetcode.com/problems/valid-sudoku/

#[test]
fn test_is_valid_sudoku() {
    let grid1: Vec<Vec<char>> = vec![
        vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
        vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
        vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
        vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
        vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
        vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
        vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
        vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
        vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
    ];
    assert!(is_valid_sudoku(grid1));

    let grid2: Vec<Vec<char>> = vec![
        vec!['8', '3', '.', '.', '7', '.', '.', '.', '.'],
        vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
        vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
        vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
        vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
        vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
        vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
        vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
        vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
    ];
    assert!(!is_valid_sudoku(grid2));
}

pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
    const SIZE: usize = 9;
    let mut rows = vec![std::collections::HashSet::with_capacity(9); SIZE];
    let mut cols = vec![std::collections::HashSet::with_capacity(9); SIZE];
    let mut boxes = vec![std::collections::HashSet::with_capacity(9); SIZE];
    for i in 0..SIZE {
        for j in 0..SIZE {
            let digit = board[i][j];
            if digit != '.' {
                if !rows[i].insert(digit)
                    || !cols[j].insert(digit)
                    || !boxes[(i / 3) * 3 + j / 3].insert(digit)
                {
                    return false;
                }
            }
        }
    }
    true
}
