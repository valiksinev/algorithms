use std::{
    io,
    collections::{BTreeMap, BTreeSet},
};


pub fn execute() {
    let read = || -> (u32, u32) {
      let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        let (x, y) = s.trim().split_once(' ').unwrap();
        (x.parse().unwrap(), y.parse().unwrap())
    };

    let (n, m) = read();
    let mut edges : BTreeMap<u32, Vec<u32>> = BTreeMap::new();
    let mut push = |a, b| {
        let vertices = edges.entry(a).or_insert(vec![]);
        vertices.push(b);
    };

    for _i in 0..m {
        let (u, v) = read();
        push(u, v);
        push(v, u);
    }

    let mut visited = BTreeSet::new();
    let mut related_list = vec![];
    for u in edges.keys() {
        let mut related = BTreeSet::new();
        dfs(&edges, u, &mut visited, &mut related);
        if !related.is_empty() {
            related_list.push(related);
        }
    }

    let count_components = n as usize - visited.len() + related_list.len();
    println!("{}", count_components);
    for i in related_list {
        println!("{}", i.len());
        let s = format!("{:?}", i).replace("{", "").replace("}", "").replace(",", "");
        println!("{}", s);

    }
    for i in 1..n+1 {
        if visited.get(&i).is_none() {
            println!("1");
            println!("{}", i);
        }
    }
}


fn dfs(edges: &BTreeMap<u32, Vec<u32>>, u: &u32, visited: &mut BTreeSet<u32>, related: &mut BTreeSet<u32>) {
    if visited.insert(*u) {
        related.insert(*u);
        for v in edges.get(u).unwrap() {
            dfs(edges, v, visited, related);
        }
    }
}


