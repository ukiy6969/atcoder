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

fn combination_two(n: usize) -> usize {
    if n == 2 {
        return 1;
    }
    if n < 2 {
        return 0;
    }
    n * (n - 1) / 2
}

fn main() {
    input! {
        n: usize,
        a: [usize;n],
    }
    let mut mem: std::vec::Vec<usize> = vec![0; n + 1];
    let mut map: std::collections::HashMap<usize, usize> = std::collections::HashMap::new();
    for &i in a.iter() {
        let default = 0;
        let &v = map.get(&i).unwrap_or(&default);
        map.insert(i, v + 1);
    }
    let mut sum = 0;
    for (i, v) in map {
        mem[i] = combination_two(v) - combination_two(v - 1);
        sum = sum + combination_two(v);
    }
    // println!("{:?}", mem);
    for k in 0..n {
        println!("{}", sum - mem[a[k]]);
    }
}
