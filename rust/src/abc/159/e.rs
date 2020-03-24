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

fn main() {
    input! {
        h: usize,
        w: usize,
        k: usize,
        board: [chars;h],
    }

    let mut ans: usize = usize::max_value();

    for bit in 0..(1 << (h - 1)) {
        let mut splited_board = std::vec::Vec::with_capacity(h);
        let mut latest_i = 0;
        for i in 0..h {
            if bit & 1 << i != 0 || i == h - 1 {
                let tmp = (0..w)
                    .map(|j| {
                        (&board[latest_i..(i + 1)])
                            .iter()
                            .map(|c| if c[j] == '1' { 1 } else { 0 })
                            .sum::<usize>()
                    })
                    .collect::<std::vec::Vec<_>>();
                splited_board.push(tmp);
                latest_i = i + 1;
            }
        }
        let mut split_count = splited_board.len() - 1;
        let mut sums = vec![0; splited_board.len()];
        'inner: for x in 0..(w - 1) {
            let mut over = false;
            for i in 0..splited_board.len() {
                if splited_board[i][x] > k {
                    split_count = usize::max_value();
                    break 'inner;
                }

                sums[i] += splited_board[i][x];
                if sums[i] + splited_board[i][x + 1] > k {
                    over = true;
                }
            }

            if over {
                split_count += 1;
                sums = vec![0; splited_board.len()];
            }

            if split_count > ans {
                split_count = usize::max_value();
                break;
            }
        }

        ans = std::cmp::min(split_count, ans);
    }
    println!("{:?}", ans);
}
