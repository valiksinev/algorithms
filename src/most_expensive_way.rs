use std::io;
use std::str::FromStr;

pub fn execute() {
    let mut matrix = vec![vec![0_u32; 0]; 0];

    let mut s = String::new();

    io::stdin().read_line(&mut s).unwrap();
    let dimensions = s.as_str().split_whitespace().map(usize::from_str).collect::<Result<Vec<usize>, _>>().unwrap();
    assert_eq!(dimensions.len(), 2);
    let (n, m) = (dimensions[0], dimensions[1]);

    for _i in 0..n {
        s.clear();
        io::stdin().read_line(&mut s).unwrap();
        let row = s.as_str().split_whitespace().map(u32::from_str).collect::<Result<Vec<u32>, _>>().unwrap();
        if row.len() != m {
            println!("row length must be {m}");
            return;
        }
        matrix.push(row);
    }

    let mut cost = vec![vec![0; m+1]; n+1];
    for i in 1..n+1 {
        for j in 1..m+1 {
            cost[i][j] = cost[i-1][j].max(cost[i][j-1] )+ matrix[i-1][j-1];
        }
    }

    let mut steps = vec![];
    let (mut i,  mut j) = (n, m);
    while i > 1 || j > 1 {
        if i == 1  {
            steps.push('R');
            j -= 1;
        }
        else if j == 1 {
            steps.push('D');
            i -= 1;
        }
        else if cost[i-1][j] > cost[i][j-1] {
            steps.push('D');
            i -= 1;
        }
        else { steps.push('R');
            j -= 1;
        }
    }

    println!("{}", cost[n][m]);
    for i in steps.iter().rev() {
        print!("{} ", i);
    }
}