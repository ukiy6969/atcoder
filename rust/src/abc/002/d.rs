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
        n: usize,
        m: usize,
        g: [(usize, usize); m],
    }
    let mut con: std::vec::Vec<std::vec::Vec<bool>> = vec![vec![false; n]; n];
    for (x, y) in g {
        con[x - 1][y - 1] = true;
    }
    // println!("{:?}", con);
    let mut ans = 0;
    for bit in 1..(1 << n) {
        let t = (0..(n + 1)).fold(0, |acc, x| if bit & (1 << x) != 0 { acc + 1 } else { acc });
        // println!("bit = {:b}, t = {}", bit, t);
        if t < ans {
            continue;
        }
        let mut ok = true;
        for i in 0..n {
            for j in (i + 1)..n {
                // println!(
                //     "i = {}, j = {}, i_bit = {}, j_bit = {}, con = {}",
                //     i,
                //     j,
                //     bit & (1 << i) != 0,
                //     bit & (1 << j) != 0,
                //     con[i][j]
                // );
                if bit & (1 << i) != 0 && bit & (1 << j) != 0 && !con[i][j] {
                    ok = false;
                }
            }
        }
        if ok {
            // println!("ok_bit = {}", bit);
            ans = t;
        }
        // println!("");
    }
    println!("{}", ans);
}
