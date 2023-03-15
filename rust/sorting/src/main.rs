fn bubble_sort(arr: &Vec<i32>) -> Vec<i32> {
    let mut arr = arr.clone();

    for i in 0..arr.len() {
        for j in i..arr.len() {
            // this is 5head
            if arr[i] > arr[j] {
                // instead of doing:
                // ---
                // let tmp = arr[j];
                // arr[j] = arr[j + 1];
                // arr[j + 1] = tmp;
                // ---
                // you can do:
                arr.swap(i, j);
            }
        }
    }

    arr
}

fn main() {
    let numbers = vec![1, 4, 7, 3, 2];
    println!("initial: {:?}", numbers);

    println!("bubble: {:?}", bubble_sort(&numbers));
}
