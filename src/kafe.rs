use std::io;

pub fn execute(){
    let mut s= String::new();

    io::stdin().read_line(&mut s).unwrap();
    let n: usize = s.trim().parse().unwrap();

    if n == 0 {
        println!("0");
        println!("0 0");
        return;
    }

    let mut cost = vec![0;n];
    for i in 0..n {
        s.clear();
        io::stdin().read_line(&mut s).unwrap();
        let price : u32 = s.trim().parse().unwrap();
        cost[i] = price;
    }

    let mut g: Vec<Vec<Option<(u32, Vec<u32>)>>> = vec![vec![None; n+1];n];

    let (result, k1, days) = {
        let mut min = u32::MAX;
        let mut k1 = 0;
        let mut v = vec![];
        for i in 0..=n {
            let (val, vec) = dp(n-1, i, &mut cost, &mut g);
            if min >= val {
                min = val;
                k1 = i;
                v = vec;
            }
        }
        (min, k1, v)
    };

    println!("{:?}", result);
    println!("{} {:?}", k1, days.len());
    days.iter().for_each(|&a| print!("{} ", &a+1));
}

fn dp (day: usize, j: usize, cost: &[u32], g: &mut Vec<Vec<Option<(u32, Vec<u32>)>>>) -> (u32, Vec<u32>) {
    if j.saturating_sub(day)  > 1 {
        (u32::MAX, vec![])
    } else if day == 0 {
        let val = if j == 0 {
            if cost[day] > 100 {
                u32::MAX
            } else {
                cost[day]
            }
        } else {
            if cost[day] > 100 {
                cost[day]
            } else {
                u32::MAX
            }
        };
        (val, vec![])
    } else {
        if let Some((val,vec)) = g[day][j].clone() {
            (val, vec)
        } else {
            if j == 0 {
                if cost[day] > 100 {
                    let (val, mut vec) =  dp(day-1, j+1, cost, g);
                    vec.push(day as u32);
                    (val, vec)
                } else {
                    let (mut a, vec_a) = dp(day-1, j, cost, g);
                    a = a.saturating_add(cost[day]);
                    let (b, mut vec_b) = dp(day-1, j+1, cost, g);
                    if a < b {
                        (a, vec_a)
                    } else {
                        vec_b.push(day as u32);
                        (b, vec_b)
                    }
                }
            } else {
                let (mut a, vec_a) = if cost[day] > 100 {
                    dp(day-1, j-1, cost, g)
                } else {
                    dp(day-1, j, cost, g)
                };
                a = a.saturating_add(cost[day]);
                let (b, mut vec_b) = dp(day-1, j+1, cost, g);
                let res = if a < b {
                    (a, vec_a)
                } else {
                    vec_b.push(day as u32);
                    (b, vec_b)
                };
                g[day][j] = Some(res);
                g[day][j].clone().unwrap()
            }
        }
    }
}