use std::io;
use std::str::FromStr;

pub fn execute(){
    let mut matrix: Vec<Vec<u32>> = vec![];

    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();

    let dimension = s.as_str().split_whitespace().map(usize::from_str).collect::<Result<Vec<usize>, _>>().unwrap();
    let n: usize = dimension.get(0).unwrap().clone();
    let m = dimension.get(1).unwrap().clone();

    assert!(n>=1 && m>=1);

    for _i in 0..n {
        s.clear();
        io::stdin().read_line(&mut s).unwrap();
        let row = s.as_str().split_whitespace().map(u32::from_str).collect::<Result<Vec<u32>, _>>().unwrap();
        if row.len() != m as usize {
            println!("matrix size should be {n}x{m}");
            return;
        }
        matrix.push(row);
    }

    let mut cost = vec![vec![0; m]; n];

    let mut value = 0_u32;
    for j in 0..m {
        value += matrix[0][j];
        cost[0][j] = value;
    }

    for i in 1..n {
        for j in 0..m {
            if j == 0 {
                cost[i][0] = matrix[i][0] + cost[i-1][0];
            } else {
                cost[i][j] = matrix[i][j] + cost[i-1][j].min(cost[i][j-1]);
            }
        }
    }
    println!("{}", cost[n-1][m-1]);
}