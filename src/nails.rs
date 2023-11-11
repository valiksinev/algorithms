use std::io::stdin;

pub fn execute() {
    let mut s = String::new();
    stdin().read_line(&mut s).unwrap();
    let _count = s.as_str().trim().parse::<u32>().unwrap();

    s.clear();
    stdin().read_line(&mut s).unwrap();
    let items : Vec<&str> = s.split(' ').collect();
    let mut items : Vec<u32> = items.iter().map(|a| a.trim().parse().unwrap()).collect();
    items.sort();

    assert!(items.len() > 1);

    let mut result : Vec<u32> = vec![0; items.len()];

    let cost = |i: usize| -> u32 {
        items.get(i).unwrap() - items.get(i-1).unwrap()
    };
    result[1] = cost(1);
    result[2] = cost(1) + cost(2);

    for i in 3..items.len()  {
        result[i] =  u32::min(result[i-1], result[i-2]) + cost(i);
    }
    println!("{:?}", result.last().unwrap());
}
