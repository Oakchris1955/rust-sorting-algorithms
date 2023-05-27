pub fn insertion_sort(list: &mut Vec<usize>) {
	let mut index = 1;
	while index < list.len() {
		let item = list[index];

		let mut local_index = 0;
		for x in (0..index-1).rev() {
			if list[x] < item {
				local_index = x+1;
				break;
			}
		}

		list.remove(index);
		list.insert(local_index, item);

		index += 1;
	}
}

pub fn selection_sort(list: &mut Vec<usize>) {
	for begin_index in 0..list.len() {
		let mut min_index = begin_index;
		for check_index in begin_index..list.len() {
			if list[check_index] < list[min_index] {
				min_index = check_index;
			}
		}

		let min_item = list[min_index];
		list.remove(min_index);
		list.insert(begin_index, min_item);
	}
}

pub fn bubble_sort(list: &mut Vec<usize>) {
	loop {
		let mut is_sorted = true;
		let mut last_index = 1;
		while last_index < list.len() {
			if list[last_index-1] > list[last_index] {
				list.swap(last_index-1, last_index);
				is_sorted = false;
			}
			last_index += 1;
		}
		if is_sorted {
			break;
		}
	}
}


#[allow(dead_code)]
/// The algorithm below is used in order to determine at which index to insert an item into a vector, assuming that the given vector is sorted. It is supposed to be used by module functions and not by outside sources (that is why it isn't a public function)
fn binary_search(list: &Vec<usize>, item: &usize) -> usize {
	let length = list.len();

	if length == 1 {
		let list_element = list[0];
		if list_element > *item {
			return 0;
		} else {
			return 1;
		}
	} else {
		let split_index = length/2;
		
		let split_item = list[split_index];

		if split_item > *item {
			return binary_search(&list[..split_index].to_vec(), item);
		} else if split_item < *item {
			return binary_search(&list[split_index..].to_vec(), item) + split_index;
		} else {
			return split_index;
		}
	}
}

#[cfg(test)]
mod tests {
	const UNSORTED_LIST: &[usize] = &[6, 3, 4, 2, 5, 1];
	const SORTED_LIST: &[usize]= &[1, 2, 3, 4, 5, 6];

    use super::*;
	
	fn general_test(list: &mut Vec<usize>, func: fn(&mut Vec<usize>)) {
		func(list);

		assert_eq!(list, SORTED_LIST, "Expected lists to be equal");
	}

	#[test]
	fn binary_search_test() {
		assert_eq!(binary_search(&vec![1, 3, 5, 7, 8, 19, 67], &9), 5);
	}

    #[test]
    fn insertion_test() {
        general_test(&mut UNSORTED_LIST.to_vec(), insertion_sort);
    }

	#[test]
    fn selection_test() {
        general_test(&mut UNSORTED_LIST.to_vec(), selection_sort);
    }

	#[test]
    fn bubble_test() {
        general_test(&mut UNSORTED_LIST.to_vec(), bubble_sort);
    }
}
