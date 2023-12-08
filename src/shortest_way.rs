use std::io;

pub fn execute() {
    let read = || -> Vec<u32> {
        let mut s= String::new();
        io::stdin().read_line(&mut s).unwrap();
        s.trim().split_whitespace().map(|a| a.parse().unwrap()).collect()
    };

    let n= read()[0] as usize;

    let mut g = vec![vec![]; n];
    for i in 0..n {
        let v = read();
        for j in 0..n {
            if v[j] == 1 {
                g[i].push(j as u32);
            }
        }
    }
    let from_to = read();
    let (from, to) = (from_to[0]-1, from_to[1]-1);
    let mut visited = vec![];

    println!("{}", bfs(&[from], to, &g, 0, &mut visited));
}

fn bfs(from: &[u32], to: u32, g: &[Vec<u32>], depth:  i32, visited: &mut Vec<u32>) -> i32 {

    let mut child = vec![];

    if from.is_empty() {
        return -1;
    }

    for i in from {
        if *i == to {
            return depth;
        } else {
            if !visited.contains(i) {
                child.append(&mut g[*i as usize].clone());
                visited.push(*i);
            }
        }
    }

    bfs(&child, to, g, depth +1, visited)
}