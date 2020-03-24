#[allow(unused_macros)]
macro_rules! input {
    (source = $s:expr, $($r:tt)*) => {
        let mut iter = $s.split_whitespace();
        let mut next = || { iter.next().unwrap() };
        input_inner!{next, $($r)*}
    };
    ($($r:tt)*) => {
        let stdin = std::io::stdin();
        let mut bytes = std::io::Read::bytes(std::io::BufReader::new(stdin.lock()));
        let mut next = move || -> String{
            bytes
                .by_ref()
                .map(|r|r.unwrap() as char)
                .skip_while(|c|c.is_whitespace())
                .take_while(|c|!c.is_whitespace())
                .collect()
        };
        input_inner!{next, $($r)*}
    };
}

macro_rules! input_inner {
    ($next:expr) => {};
    ($next:expr, ) => {};

    ($next:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };
}

macro_rules! read_value {
    ($next:expr, ( $($t:tt),* )) => {
        ( $(read_value!($next, $t)),* )
    };

    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };

    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };

    ($next:expr, usize1) => {
        read_value!($next, usize) - 1
    };

    ($next:expr, $t:ty) => {
        $next().parse::<$t>().expect("Parse error")
    };
}

fn solve(
    board: &std::vec::Vec<std::vec::Vec<char>>,
    (r, c): (usize, usize),
    mem: &mut std::vec::Vec<std::vec::Vec<usize>>,
) -> usize {
    if (board.len() - 1, board[0].len() - 1) == (r, c) {
        return 0;
    }
    let mut l = vec![usize::max_value(); 2];
    for (dx, dy) in vec![(1, 0), (0, 1)] {
        if board.len() > r + dx && board[0].len() > c + dy {
            let add = if board[r][c] == '.' && board[r + dx][c + dy] == '#' {
                1
            } else {
                0
            };
            l.push(if mem[r + dx][c + dy] != usize::max_value() {
                add + mem[r + dx][c + dy]
            } else {
                add + solve(board, (r + dx, c + dy), mem)
            });
        }
    }
    // println!("{}, {}", a, b);
    mem[r][c] = l.into_iter().min().unwrap()
        + if (r, c) == (0, 0) && board[r][c] == '#' {
            1
        } else {
            0
        };
    mem[r][c]
}

fn main() {
    input! {
        h: usize,
        w: usize,
        board: [chars; h],
    }
    let mut mem: std::vec::Vec<std::vec::Vec<usize>> = vec![vec![usize::max_value(); w]; h];
    let res = solve(&board, (0, 0), &mut mem);
    // println!("{:?}, {}", board, res);
    println!("{}", res);
}
