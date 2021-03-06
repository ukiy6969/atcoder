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

fn solve(s: &[char]) -> bool {
    let a = &s[0..(s.len() / 2)];
    let b = &s[(s.len() / 2 + (s.len() % 2))..(s.len())];
    let b_clone = &mut vec!['0'; b.len()][..];
    b_clone.clone_from_slice(b);
    b_clone.reverse();
    // println!("{:?}, {:?}", a, b_clone);
    a == b_clone
}

fn main() {
    input! {
        s: chars,
    }
    let a = solve(&s[..]);
    let b = solve(&s[0..((s.len() - 1) / 2)]);
    let c = solve(&s[((s.len() + 3) / 2 - 1)..(s.len())]);
    // println!("{:?}", &s[((s.len() + 3) / 2 - 1)..(s.len())]);
    println!("{}", if a && b && c { "Yes" } else { "No" });
}
