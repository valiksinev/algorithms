use std::fs;

pub fn execute(){

    let s= fs::read_to_string("/home/user/CLionProjects/alg/algorithms/file/knight_move.in")
        .expect("Should have been able to read the file");

    let (n, m): (usize, usize) = s.split_once(' ')
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
    fs::write("/home/user/CLionProjects/alg/algorithms/file/knight_move.out", res)
        .expect("Should have been able to write the file");
}

