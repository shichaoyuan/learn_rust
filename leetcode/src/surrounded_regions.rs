
use super::UF;


struct Solution;
impl Solution {
    pub fn solve(board: &mut Vec<Vec<char>>) {
        let m = board.len();
        let n = board[0].len();

        let mut uf = UF::new(m*n+1);
        let dummy = m * n;
        for i in 0..m {
            if board[i][0] == 'O' {
                uf.union(i*n, dummy);
            }
            if board[i][n-1] == 'O' {
                uf.union(i*n+n-1, dummy);
            }
        }
        for j in 0..n {
            if board[0][j] == 'O' {
                uf.union(j, dummy);
            }
            if board[m-1][j] == 'O' {
                uf.union(n*(m-1)+j, dummy);
            }
        }

        for i in 1..(m-1) {
            for j in 1..(n-1) {
                if board[i][j] == 'O' {
                    if board[i+1][j] == 'O' {
                        uf.union((i+1)*n+j, i*n+j);
                    }
                    if board[i][j+1] == 'O' {
                        uf.union(i*n+j+1, i*n+j);
                    }
                    if board[i-1][j] == 'O' {
                        uf.union((i-1)*n+j, i*n+j);
                    }
                    if board[i][j-1] == 'O' {
                        uf.union(i*n+j-1, i*n+j);
                    }
                }
            }
        }
        for i in 1..(m-1) {
            for j in 1..(n-1) {
                if !uf.connected(dummy, i*n+j) {
                    board[i][j] = 'X';
                }
            }
        }
    }
}