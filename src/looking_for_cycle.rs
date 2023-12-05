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
    for i in 0..n {
        if !visited.contains(&(i as u32) ) {
            if let Err(v) = dfs(i as u32, &g, &mut visited, None) {
                println!("YES");
                println!("{}", v.len());
                v.iter().for_each(|a| print!("{} ", a+1));
                return;
            }
        }
    }
    println!("NO");
}


fn dfs (i: u32, g: &Vec<Vec<u32>>, visited: &mut BTreeSet<u32>, parent: Option<u32>) -> Result<Vec<u32>, Vec<u32>> {
    if !visited.insert(i) {
        return Ok(vec![i]);
    }

    for j in g[i as usize].iter() {
        //skip parent
        if let Some(parent) = parent {
            if parent == *j {
                continue
            }
        }
        match dfs(*j, g, visited, Some(i)) {
            Ok(mut v) => {
                if !v.is_empty(){
                    if *v.first().unwrap() == i {
                        return Err(v)
                    } else {
                        v.push(i);
                        return Ok(v);
                    }
                }
            },
            Err(v) => { return Err(v); }
        }
    }

    Ok(vec![])
}