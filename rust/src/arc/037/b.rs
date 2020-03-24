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

use std::vec::*;

fn dfs(v: usize, p: Option<usize>, edges: &Vec<Vec<usize>>, flags: &mut Vec<bool>) -> bool {
    if flags[v] {
        return false;
    }
    flags[v] = true;
    edges[v]
        .iter()
        .filter(|&&v1| p.map(|p| v1 != p).unwrap_or(true))
        // .inspect(|x| println!("v = {}, edge = {:?}", v, x))
        .map(|&v1| dfs(v1, Some(v), edges, flags))
        .all(|b| b)
}

fn main() {
    input! {
        n: usize,
        m: usize,
        edges: [(usize, usize); m],
    }
    let mut flags = vec![false; n];
    let mut g = vec![Vec::with_capacity(n); n];
    for (v1, v2) in edges {
        g[v1 - 1].push(v2 - 1);
        g[v2 - 1].push(v1 - 1);
    }
    let mut ans = 0;
    for i in 0..n {
        if !flags[i] {
            if dfs(i, None, &g, &mut flags) {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}
