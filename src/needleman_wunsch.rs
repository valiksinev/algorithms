use std::io;
use std::str::FromStr;

pub fn execute(){
    let read_stdin = || -> Vec<u32> {
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        s.as_str().split_whitespace().map(u32::from_str).collect::<Result<Vec<u32>, _>>().unwrap()
    };

    let x = read_stdin();
    let y = read_stdin();

    let mut a = vec![vec![0_u32; y.len()+1]; x.len()+1];

    for i in 0..x.len() {
        for j in 0..y.len() {
            a[i+1][j+1] = if x[i] == y[j] {
                a[i][j]
            } else {
                a[i+1][j].max(a[i][j+1])
            };
        }
    }

    let mut i = x.len();
    let mut j = y.len();
    let mut lcs = vec![];
    while i >0 && j >0 {
        if x[i-1] == y[j-1] {
            lcs.push(x[i-1]);
            i -=1;
            j -=1;
        } else if a[i][j-1] > a[i-1][j] {
            j -=1;
        } else {
            j -=1;
        }
    };

    for i in lcs.iter().rev() {
        print!("{i} ");
    }
}