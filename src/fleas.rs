use std::{io, collections::HashSet};

pub fn execute() {
    let read = || -> Vec<u32> {
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        s.trim().split_whitespace().map(|a| a.parse().unwrap()).collect()
    };

    let v = read();
    let n = v[0];
    let m = v[1];
    let finish = (v[2], v[3]);
    let q = v[4] as usize;

    let mut start = vec![];
    for _i in 0..q {
        let point = read();
        start.push((point[0], point[1]));
    }

    let mut sum = 0;
    for flea in 0..q {
        let mut visited = HashSet::from([start[flea]]);
        let (x, y) = start[flea];
        if let Ok(depth) =  bfs(&[(x, y)], n, m, finish, &mut visited, 0) {
            sum += depth;
        } else {
            println!("-1");
            return;
        }
    }

    println!("{sum}");
}

fn bfs(from: &[(u32, u32)], n: u32, m: u32, finish: (u32, u32), visited: &mut HashSet<(u32, u32)>, depth: u32) -> Result<u32, ()> {
    let mut to = vec![];

    for (x,y) in from {
        if *x == finish.0 && *y == finish.1 {
            return Ok(depth);
        }
        for dx in [-1_i32, -2, 1, 2] {
            for dy in  [-1_i32, -2, 1, 2] {
                if dx.abs() == dy.abs() {
                    continue
                }
                let u: i32 = *x as i32 + dx;
                let v: i32 = *y as i32 + dy;
                if u >= 1  && u <= n as i32  && v >= 1 && v <= m as i32 {
                    let u = u as u32;
                    let v = v as u32;
                    if visited.insert((u, v)){
                        to.push((u, v));
                    }
                }
            }
        }

    }

    if to.is_empty() {
        return Err(())
    };

    bfs(&to, n, m, finish, visited, depth + 1)
}
