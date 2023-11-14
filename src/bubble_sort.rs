pub fn execute() {

    let mut src = vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0];

    let n = src.len();

    for i in 0..n-1 {
        for j in 0..n-1-i {
            if src[j] > src[j+1] {
                src.swap(j, j+1);
            }
        }
    }

    println!("{:?}", src);
}