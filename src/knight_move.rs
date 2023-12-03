use std::fs;

pub fn execute(){

    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let (n, m): (usize, usize) = s.trim().split_once(' ')
        .map(|(a, b)| (a.parse::<usize>().unwrap(), b.parse::<usize>()
            .unwrap())).expect("whilespace split error");

    let mut table = vec![vec![0; m+1]; n+1];

    table[1][1] = 1;
    for i in 2..n+1 {
        for j in 2..m+1 {
            table[i][j] = table[i-2][j-1] + table[i-1][j-2]
        }
    }

    let res = format!("{}", table[n][m]);
    println!("{}", res);
}

