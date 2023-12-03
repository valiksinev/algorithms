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

    let mut price_list = vec![0;n];
    for i in 0..n {
        s.clear();
        io::stdin().read_line(&mut s).unwrap();
        let price : u32 = s.trim().parse().unwrap();
        price_list[i] = price;
    }

    let mut g = vec![vec![u32::MAX; n+1];n];

    let mut coupon_count = 0;

    let mut k2 = vec![0_usize; n+1];
    let mut k2_days = vec![vec![]; n+1];

    let mut k2_cur = k2.clone();
    let mut k2_days_cur = k2_days.clone();

    for day in 0..n {

        let price = price_list[day];
        if price > 100 {
            coupon_count += 1;
            if day == 0 {
                g[0][coupon_count] = price;
            } else {
                for coupon in 0..coupon_count+1 {
                    if coupon == 0 {
                        if g[day-1][coupon +1] != u32::MAX {
                            g[day][coupon] = g[day-1][coupon +1];

                            k2_cur[coupon] = k2[coupon + 1] + 1;
                            k2_days_cur[coupon] = k2_days[coupon + 1].clone();
                            k2_days_cur[coupon].push(day)
                        }
                    } else {
                        let cash = g[day-1][coupon-1].saturating_add(price);
                        let free = g[day-1][coupon+1];
                        if cash < free {
                            g[day][coupon] = cash;
                        } else {
                            g[day][coupon] = free;
                            if g[day-1][coupon +1] != u32::MAX {
                                k2_cur[coupon] = k2[coupon + 1] + 1;
                                k2_days_cur[coupon] = k2_days[coupon + 1].clone();
                                k2_days_cur[coupon].push(day)
                            }


                        }
                    };
                }
            }
        } else {
            if day == 0 {
                g[0][0] = price;
            } else {
                for coupon in 0..coupon_count+1 {
                    let cash = g[day-1][coupon].saturating_add(price);
                    let free = g[day-1][coupon+1];
                    if cash < free {
                        g[day][coupon] = cash;
                    } else {
                        g[day][coupon] = free;

                        if g[day-1][coupon +1] != u32::MAX {
                            k2_cur[coupon] = k2[coupon + 1] + 1;
                            k2_days_cur[coupon] = k2_days[coupon + 1].clone();
                            k2_days_cur[coupon].push(day)

                        }
                    }
                }
            }
        }
        k2 = k2_cur.clone();
        k2_days = k2_days_cur.clone();
    }

    let mut res = u32::MAX;
    let mut k1 = 0;
    for i in 0..coupon_count+1 {
        if g[n-1][i] <= res {
            res = g[n-1][i];
            k1 = i;
        }
    }

    let k2 = k2[k1];
    println!("{:?}", res);
    println!("{} {:?}", k1, k2);
    k2_days[k1].iter().for_each(|&a| print!("{} ", &a+1));
}
