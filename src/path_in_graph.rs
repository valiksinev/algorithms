use std::{io, fmt};

#[derive(Clone)]
struct Node {
    pub value: u32,
    pub parent: Option<Box<Node>>,
}

impl Node {
    fn unroll(&self) -> Vec<u32> {
        if self.parent.is_none() {
            return vec![self.value];
        } else {
            let mut v: Vec<u32> = vec![self.value];
            v.append( &mut self.parent.clone().unwrap().unroll());
            v
        }
    }

}

impl fmt::Display for Node {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let v = self.unroll();
        v.iter().rev().for_each(|a| print!("{} ", a+1));
        Ok(())
    }

}

pub fn execute() {
    let read = || -> Vec<u32> {
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        s.trim().split_whitespace().map(|a| {a.parse().unwrap()}).collect()
    };

    let n = read()[0] as usize;
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

    let from = Box::new(
        Node {
            value: from,
            parent: None
        });

    if let Ok(node) = bfs(&[from], to, &g, &mut visited){
        let way_len = node.unroll().len()-1;
        println!("{}", way_len);
        if way_len > 0 {
            println!("{}", node);
        }
    } else {
        println!("-1");
    }
}

fn bfs(from: &[Box<Node>], to: u32, g: &[Vec<u32>], visited: &mut Vec<u32>) -> Result<Box<Node>, ()> {
    let mut child = vec![];

    if from.is_empty() {
        return Err(());
    }

    for node in from {
        if node.value == to {
            return Ok(node.clone());
        } else {
            if !visited.contains(&node.value){
                visited.push(node.value);

                let mut nodes: Vec<Box<Node>> = g[node.value as usize].iter().map(|a| {
                    Box::new(
                        Node{
                            value: *a,
                            parent: Some(node.clone()),
                            }
                    )
                }).collect();

                child.append(&mut nodes);
            }
        }
    }

    bfs(&child, to, g, visited)
}