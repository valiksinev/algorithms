use std::io;
use std::str::FromStr;
use std::cmp::Ordering;

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

    let mut cost = vec![vec![0; m]; n];

    let mut value = 0;
    for j in 0..m {
        value += matrix[0][j];
        cost[0][j] = value;
    }

    for i in 1..n {
        for j in 0..m {
            if j == 0 {
                cost[i][0] = cost[i-1][0] + matrix[i][0];
            } else {
                cost[i][j] = cost[i-1][j].max(cost[i][j-1] )+ matrix[i][j];
            }
        }
    }

    let mut steps = vec![];

    let (mut i,  mut j) = (n-1, m-1);

    loop{
        match cost[i-1][j].cmp(&cost[i][j-1]) {
            Ordering::Less => {
                steps.push('R');
                j -=1;
            },
            _ => {
                steps.push('D');
                i -=1;
            }
        };

        if j == 0 {
            steps.resize(n+m-1, 'D' );
            break;
        }
        if i == 0 {
            steps.resize(n+m-1, 'R');
        }
    }

    println!("{:?}", cost[n-1][m-1]);
    for i in steps.iter().rev() {
        print!("{} ", i);
    }
}