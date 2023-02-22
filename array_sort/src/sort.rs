use std::str::FromStr;
pub fn sort(seq: &str) -> String {
    let mut result = "".to_string();
    let split = seq.split(";");
    let mut arr: Vec<i32> = Vec::new();

    for num in split {
        let insert: i32 = FromStr::from_str(num).unwrap();
        arr.push(insert);
    }

    heap_sort(&mut arr);
    for sorted_item in arr {
        result = result + &sorted_item.to_string() + ";";
    }

    return result;
}

fn heap_sort(arr: &mut [i32]) {
    if arr.len() <= 1 {
        return;
    }

    let last_parent = (arr.len() - 2) / 2;
    for i in (0..=last_parent).rev() {
        min_heapify(arr, i);
    }

    for end in (1..arr.len()).rev() {
        //tato část kódu se provede jakmile bude  pole splňovat podmínky pro haldu
        arr.swap(0, end);
        println!("{:?}", arr);
        min_heapify(&mut arr[..end], 0);
    }
}

fn min_heapify(arr: &mut [i32], mut root: usize) {
    /*Zhalduje pole, neboli převede největší položku do kořenu */
    let last: usize = arr.len() - 1;
    loop {
        let left = 2 * root + 1;
        if left > last {
            break;
        }
        let right = left + 1;

        //Podmínka pro zjištění, zda je potomek levý nebo pravý
        let child = if right <= last && arr[right] > arr[left] {
            right
        } else {
            left
        };

        //pokud má potomek větší hodnotu než kořen, tak si vymění místo
        if arr[child] > arr[root] {
            arr.swap(root, child);
        }
        root = child;
    }
}