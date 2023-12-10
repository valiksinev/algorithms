use std::io;

pub fn execute() {
    let read = || -> Vec<usize> {
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        s.trim().split_whitespace().map(|a| a.parse().unwrap()).collect()
    };

    let v = read();
    let n = v[0];
    let m = v[1];
    let finish = (v[2], v[3]);
    let q = v[4];

    let mut fleas = vec![];
    for _i in 0..q {
        let point = read();
        fleas.push((point[0], point[1]));
    }

    let mut cost =vec![vec![None; m+1]; n+1];
    bfs(&[finish], &mut cost, 0 );

    let mut sum = 0;
    for i  in 0..q {
        let (x, y) = fleas[i];
        if let Some(cost) = cost[x][y] {
            sum += cost;
        } else {
            println!("-1");
            return;
        }
    }

    println!("{sum}");
}

fn bfs(from: &[(usize, usize)], cost: &mut Vec<Vec<Option<u32>>>, depth: u32) {

    if from.is_empty() {
        return
    };

    let mut to = vec![];

    for (x, y) in from {
        cost[*x][*y] = Some(depth);

        for dx in [-1_i32, -2, 1, 2] {
            for dy in [-1_i32, -2, 1, 2] {
                if dx.abs() == dy.abs() {
                    continue
                }
                let u: i32 = *x as i32 + dx;
                let v: i32 = *y as i32 + dy;
                if u >= 1 && u < cost.len() as i32 && v >= 1 && v < cost[0].len() as i32 {
                    let u = u as usize;
                    let v = v as usize;
                    if cost[u][v].is_none() {
                        to.push((u, v));
                    }
                }
            }
        }
    }

    bfs(&to, cost, depth + 1);
}
