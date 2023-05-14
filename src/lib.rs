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

