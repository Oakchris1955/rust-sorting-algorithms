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
    fn insertion_test() {
        general_test(&mut UNSORTED_LIST.to_vec(), insertion_sort);
    }

	#[test]
    fn selection_test() {
        general_test(&mut UNSORTED_LIST.to_vec(), selection_sort);
    }
}
