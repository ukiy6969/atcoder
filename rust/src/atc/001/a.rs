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
    board: &mut std::vec::Vec<std::vec::Vec<char>>,
    w: usize,
    h: usize,
    s: (usize, usize),
    g: (usize, usize),
) -> bool {
    if s == g {
        return true;
    }
    let (sw, sh) = s;
    board[sh][sw] = '#';
    let sw = sw as isize;
    let sh = sh as isize;
    [(0, -1), (1, 0), (0, 1), (-1, 0)]
        .iter()
        .filter(|&&(dw, dh)| {
            0 <= sw + dw && sw + dw < w as isize && 0 <= sh + dh && sh + dh < h as isize
        })
        .map(|&(dw, dh)| ((sw + dw) as usize, (sh + dh) as usize))
        .skip_while(|&(sw, sh)| board[sh][sw] != '.' || !solve(board, w, h, (sw, sh), g))
        .any(|_| true)
}

fn main() {
    input! {
        h: usize,
        w: usize,
        board: [chars; h],
    }
    let mut s = (0, 0);
    let mut g = (0, 0);
    let mut b = vec![vec!['#'; w]; h];
    for j in 0..h {
        for i in 0..w {
            if board[j][i] == 's' {
                s = (i, j);
            }
            if board[j][i] == 'g' {
                g = (i, j);
            }
            if board[j][i] != '#' {
                b[j][i] = '.'
            }
        }
    }
    let ans = solve(&mut b, w, h, s, g);
    println!("{}", if ans { "Yes" } else { "No" });
}
