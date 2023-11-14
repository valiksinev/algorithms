
fn bs (a: &[i32], val: i32) {
    let mid: usize = a.len()/2;

    if mid == 0 {
        if a[0] == val {
            println!("{val}")
        } else {
            println!("{val} not found");
        }
        return;
    }

    if val < a[mid] {
        bs(&a[..mid], val);
    } else {
        bs(&a[mid..], val);
    }
}

pub fn execute() {
    let src = [10, 20, 30, 40, 50, 60, 70, 80, 90];
    let val  = 70;
    bs(&src, val);
}