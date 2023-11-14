
fn partition(a: &mut [i32], lo: usize, hi: usize) -> usize {
    let pivot = a[lo];
    let mut partition_index = lo;

    for i in lo+1..hi {
        if a[i] < pivot {
            a.swap(partition_index, i);
            partition_index += 1;
        }
    }
    partition_index
}

fn quick_sort(a: &mut [i32], lo: usize, hi: usize) {
    if lo+1 < hi {  //  lo+1  -  exclude the cases when array contains only one element
        let mut partition_index = partition(a, lo, hi);
        quick_sort(a, lo, partition_index);
        if partition_index == lo {
            partition_index +=1;
        }
        quick_sort(a, partition_index, hi);
    }
}

pub fn execute() {
    let a: &mut[i32] = &mut [4, 8, 1, 6, 5, 0, 9, 2, 12, 0];
    quick_sort(a, 0, a.len());
    println!("{:?}", a);
}