use std::{
    io,
    collections::{BTreeMap, BTreeSet},
};

pub fn execute() {
    let read = || -> (u32, u32) {
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        let (x, y) = s.trim().split_once(' ').unwrap();
        (x.trim().parse().unwrap(), y.trim().parse().unwrap())
    };

    let (n, m) = read();

    let mut edges:BTreeMap<u32, Vec<u32>> = BTreeMap::new();

    for _i in 0..m as usize {
        let (u, v) = read();
        let values = edges.entry(u).or_insert(vec![]);
        values.push(v);
    }


    let mut finished = BTreeSet::new(); // for fast check "contains"
    let mut finished_v = vec![]; // for store order.
    for i in 1..n+1 {
        let mut visited = BTreeSet::new();
        dfs(&i, &edges, &mut finished, &mut visited, &mut finished_v);
    }

    if finished_v.is_empty() {
        println!("-1");
    } else {
        finished_v.iter().rev().for_each(|&a| print!("{} ", &a));
    }
}


fn dfs (u: &u32, edges: &BTreeMap<u32, Vec<u32>>, finished: &mut BTreeSet<u32>, visited: &mut BTreeSet<u32>, finished_v: &mut Vec<u32>) {
    // cyclic case
    if finished.contains(u) {
        return;
    }
    if !visited.insert(*u) {
        println!("-1");
        std::process::exit(0);
    }

    if let Some(values) = edges.get(u) {
        for i in values {
            dfs(i, edges, finished, visited, finished_v);
        }
    }
    finished.insert(*u);
    finished_v.push(*u);
}
