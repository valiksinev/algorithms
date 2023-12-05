use std::{
    io,
    collections::BTreeSet,
};



pub fn execute() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    let n: usize = s.trim().parse().unwrap();
    let mut g = vec![vec![]; n];

    for i in 0..n {
        s.clear();
        io::stdin().read_line(&mut s).unwrap();
        let v :Vec<u32> = s.trim().split_whitespace().map(|a| a.parse().unwrap()).collect();
        for j in 0..n {
            if v[j] == 1 {
                g[i].push(j as u32);
            }
        }
    }

    let mut visited = BTreeSet::new();
    let mut result = vec![];
    for i in 0..n {
        if !visited.contains(&(i as u32) ) {
            dfs(i as u32, &g, &mut visited, None, &mut result);
        }
    }
    println!("NO");
}

fn dfs (i: u32, g: &Vec<Vec<u32>>, visited: &mut BTreeSet<u32>, parent: Option<u32>, result: &mut Vec<u32>) -> Option<u32> {
    if !visited.insert(i) {
        return Some(i);
    }

    for j in g[i as usize].iter() {
        if let Some(parent) = parent {
            if parent == *j {
                continue
            }
        }
        if let Some(k) = dfs(*j, g, visited, Some(i), result) {
            result.push(i);
            if k == i {

                println!("YES");
                println!("{}", result.len());
                result.iter().for_each(|a| print!("{} ", a+1));
                std::process::exit(0);
            } else {
                return Some(k);
            }
        }
    }

    None
}