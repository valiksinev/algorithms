use std::io;

pub fn execute(){
    let mut s= String::new();

    io::stdin().read_line(&mut s).unwrap();
    let n: usize = s.trim().parse().unwrap();

    let mut price_list = vec![0;n];
    for i in 0..n {
        s.clear();
        io::stdin().read_line(&mut s).unwrap();
        let price : u32 = s.trim().parse().unwrap();
        price_list[i] = price;
    }

    let mut g = vec![vec![u32::MAX; n+1];n];

    let mut coupon_count = 0;
    for day in 0..n {
        let price = price_list[day];
        if price > 100 {
            coupon_count += 1;
            if day == 0 {
                g[0][coupon_count] = price;
            } else {
                for coupon in 0..coupon_count+1 {
                    g[day][coupon] = if coupon == 0 {
                        g[day-1][coupon +1]
                    } else {
                        (g[day-1][coupon-1].saturating_add(price)).min(g[day-1][coupon+1])
                    };
                }
            }
        } else {
            if day == 0 {
                g[0][0] = price;
            } else {
                for coupon in 0..coupon_count+1 {
                    g[day][coupon] = (g[day-1][coupon].saturating_add(price)).min(g[day-1][coupon+1])
                }
            }
        }
    }
    println!("{:?}", g[n-1][0]);

}
