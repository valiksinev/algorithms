use std::collections::{BTreeMap, BTreeSet};
use std::io;

pub fn execute() {
    let mut s = String::new();

    //n
    io::stdin().read_line(&mut s).unwrap();
    let n = s.trim().parse::<usize>().unwrap();

    // x, y
    let mut xy: Vec<(i32, i32)> = vec![];
    for _i in 0..n {
        s.clear();
        io::stdin().read_line(&mut s).unwrap();
        if let Some((x, y)) = s.trim().split_once(' ').map(|(a, b)| (a.parse::<i32>().unwrap(), b.parse::<i32>().unwrap())) {
            xy.push((x, y));
        } else {
            println!("input error");
            return;
        }
    }

    //k
    s.clear();
    io::stdin().read_line(&mut s).unwrap();
    let k: u32 =  s.trim().parse().unwrap();

    //from, to
    s.clear();
    io::stdin().read_line(&mut s).unwrap();
    let from_to: Option<(usize, usize)>  = s.trim().split_once(' ').map(|(a, b)| (a.parse::<usize>().unwrap(), b.parse::<usize>().unwrap()));
    if from_to.is_none() {
        println!("input error");
        return
    }
    let (from, to) = from_to.unwrap();
    assert!(from > 0);
    assert!(to > 0);
    let from= from -1;  // we will use indexes starting from 0
    let to = to -1;

    // Graph's adjacenсу table
    let mut adjacency: BTreeMap<usize, Vec<usize>> = BTreeMap::new();

    for town1 in 0..n{
        for town2 in town1+1..n {
            let (x1, y1) = xy[town1].clone();
            let (x2, y2) = xy[town2].clone();
            let length = x1.abs_diff(x2) + y1.abs_diff(y2);
            // amount of the petrol will be enought
            if length <= k {
                let mut push = |u, v| {
                    let nodes = adjacency.entry(u).or_insert(vec![]);
                    nodes.push(v);
                };
                push(town1, town2);
                push(town2, town1);
            }
        }
    }

    let mut visited = BTreeSet::new();

    let res = bfs(vec![from], &mut visited,  &adjacency, to);
    println!("{res}");
}

fn bfs (nodes: Vec<usize>, visited: &mut BTreeSet<usize>, adjacency: &BTreeMap<usize, Vec<usize>>, to: usize) -> i32 {
    let mut childs : Vec<usize> = vec![];

    for parent in nodes {
        if parent == to {
            return 0;
        }
        if visited.insert(parent) {
            if let Some(child) = adjacency.get(&parent) {
                childs.append(&mut child.clone());
            }
        }
    }

    if childs.is_empty() {
        return -1
    } else {
        let res = bfs(childs, visited, adjacency, to);
        if res != -1 {
            res + 1
        } else {
            res
        }
    }
}