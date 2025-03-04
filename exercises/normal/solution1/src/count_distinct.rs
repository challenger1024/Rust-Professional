use std::collections::HashSet;


pub fn new_count_distinct(input_str: &str) -> usize {
	let v: Vec<String> = input_str.split(',').map(|s| s.to_string()).collect();
	let  m: HashSet<_>=v.into_iter().collect();
	m.len()
}
