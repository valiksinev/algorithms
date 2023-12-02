use std::{
    io,
    collections::BTreeMap,
};

pub fn execute() {
    let read = || -> (u32, u32) {
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        let (x, y) = s.trim().split_once(' ').unwrap();
        (x.parse().unwrap(), y.parse().unwrap())
    };

    let (_n, m) = read();

    let mut edges:BTreeMap<u32, Vec<u32>> = BTreeMap::new();

    for _i in 0..m as usize {
        let (u, v) = read();
        let values = edges.entry(u).or_insert(vec![]);
        values.push(v);
    }

    let mut finished = vec![];

    for i in edges.keys() {
        dfs(i, &edges, &mut finished);
    }

    finished.iter().rev().for_each(|a| print!("{} ", a));
}


fn dfs (u: &u32, edges: &BTreeMap<u32, Vec<u32>>, finished: &mut Vec<u32>) {
    if !finished.contains(u){
        if let Some(values) = edges.get(u) {
            for i in values {
                dfs(i, edges, finished);
            }
        }
        finished.push(*u);
    }
}
