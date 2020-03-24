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
        d: usize,
        g: i32,
        v: [(i32, i32); d],
    }
    let mut ans = i32::max_value();
    for bit in 0..(1 << d) {
        // println!("{:?}", bit);
        let mut sum = 0;
        let mut num = 0;
        let mut rest_max = usize::max_value();
        for i in 0..d {
            if bit & 1 << i != 0 {
                sum += 100 * (i as i32 + 1) * v[i].0 + v[i].1;
                num += v[i].0;
            // print!("{} ", 1);
            } else {
                // print!("{} ", 0);
                rest_max = i;
            }
        }
        if sum < g {
            let s1 = 100 * (rest_max + 1) as i32;
            let need = (g - sum + s1 - 1) / s1;
            // println!("need = {}", need);
            if need >= v[rest_max].0 {
                continue;
            }
            num += need;
        }
        // println!("end sum = {}, num = {}, rest_max = {}", sum, num, rest_max);
        ans = std::cmp::min(ans, num);
    }
    println!("{}", ans);
}
