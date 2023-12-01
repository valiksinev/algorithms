use std::{
    io,
    collections::BTreeMap,
};


pub fn execute() {
    let mut edges : BTreeMap<u32, Vec<u32>> = BTreeMap::new();

    let read = || -> (u32, u32) {
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        let (x, y) = s.trim().split_once(' ').unwrap();
        (x.parse().unwrap(), y.parse().unwrap())
    };

    let mut push = |a, b| {
        let vertices = edges.entry(a).or_insert(vec![]);
        vertices.push(b);
    };

    let (_n, m) = read();

    for _i in 0..m {
        let (u, v) = read();
        push(u, v);
        push(v, u);
    }

    let mut visited = BTreeMap::new();
    for u in edges.keys() {
        dfs(&edges, u, &mut visited, None);
    }

    println!("YES");
}


fn dfs(edges: &BTreeMap<u32, Vec<u32>>, u: &u32, visited: &mut BTreeMap<u32, bool>, color: Option<bool>) {

    if let Some(color_) = visited.get(u) {
        if color.unwrap_or(*color_) !=  *color_ {
            println!("NO");
            std::process::exit(0);
        }
    } else {
        let color = color.or(Some(false));
        let color_inv = color.map(|a| !a);

        visited.insert(*u, color.unwrap());
        for v in edges.get(u).unwrap() {
            dfs(edges, v, visited, color_inv);
        }
    }
}


