fn heapify(arr: &mut Vec<i32>, n: usize, i: usize) {
    let mut smallest = i;
    let left = 2 * i + 1;
    let right = 2 * i + 2;

    if left < n && arr[left] < arr[smallest] {
        smallest = left;
    }
    if right < n && arr[right] < arr[smallest] {
        smallest = right;
    }
    if smallest != i {
        arr.swap(i, smallest);
        heapify(arr, n, smallest);
    }
}

fn heap_sort(arr: &mut Vec<i32>) {
    let n = arr.len();

    for i in (0..n / 2).rev() {
        heapify(arr, n, i);
    }

    for i in (1..n).rev() {
        arr.swap(0, i);
        heapify(arr, i, 0);
    }
}

fn main() {
    let mut arr = vec![1, 3, 5, 4, 18, 13, 10, 9, 8, 15, 17];
    println!("Sebelum: {:?}", arr);
    heap_sort(&mut arr);
    arr.reverse(); 
    println!("Setelah: {:?}", arr);
}
