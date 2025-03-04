/*
	heap
	This question requires you to implement a binary heap function
*/


use std::cmp::Ord;
use std::default::Default;

pub struct Heap<T>
where
	T: Default,
{
	count: usize,
	items: Vec<T>,
	comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T>
where
	T: Default,
{
	pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
	Self {	
			count: 0,
			items: vec![T::default()],
			comparator,
		}
	}

	pub fn len(&self) -> usize {
		self.count
	}

	pub fn is_empty(&self) -> bool {
		self.len() == 0
	}

	pub fn add(&mut self, value: T) {
		//TODO
		self.items.push(value);
		self.count+=1;
		let mut idx=self.len();
		while idx/2 > 0 {
			let fa=self.parent_idx(idx);
			if !(self.comparator)(&self.items[fa],&self.items[idx]){
				self.items.swap(fa,idx);
			}
			idx=fa;
		}
	}

	fn parent_idx(&self, idx: usize) -> usize {
		idx / 2
	}

	fn children_present(&self, idx: usize) -> bool {
		self.left_child_idx(idx) <= self.count
	}

	fn left_child_idx(&self, idx: usize) -> usize {
		idx * 2
	}

	fn right_child_idx(&self, idx: usize) -> usize {
		self.left_child_idx(idx) + 1
	}

	fn smallest_child_idx(&self, idx: usize) -> usize {
		//TODO
		let l=self.left_child_idx(idx);
		let r=self.right_child_idx(idx);
		if r>self.len(){
			return l;
		}
		if (self.comparator)(&self.items[l],&self.items[r]){
			l
		}else{
			r
		}
	}
}

impl<T> Heap<T>
where
	T: Default + Ord,
{
	/// Create a new MinHeap
	pub fn new_min() -> Self {
		Self::new(|a, b| a < b)
	}

	/// Create a new MaxHeap
	pub fn new_max() -> Self {
		Self::new(|a, b| a > b)
	}
}

impl<T> Iterator for Heap<T>
where
	T: Default,
{
	type Item = T;

	fn next(&mut self) -> Option<T> {
		//TODO
		if self.len()==0{
			return None;
		}
		let l=self.len();
		self.items.swap(1,l);
		let ans=Some(self.items.pop());
		self.count-=1;
		let mut fa=1;
		while self.children_present(fa){
			let child=self.smallest_child_idx(fa);
			if (self.comparator)(&self.items[fa],&self.items[child]){
				break;
			}else{
				self.items.swap(child,fa);
				fa=child;
			}
		}
		ans?
	}
}

pub struct MinHeap;

impl MinHeap {
	#[allow(clippy::new_ret_no_self)]
	pub fn new<T>() -> Heap<T>
	where
		T: Default + Ord,
	{
		Heap::new(|a, b| a < b)
	}
}

pub struct MaxHeap;

impl MaxHeap {
	#[allow(clippy::new_ret_no_self)]
	pub fn new<T>() -> Heap<T>
	where
		T: Default + Ord,
	{
		Heap::new(|a, b| a > b)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn test_empty_heap() {
		let mut heap = MaxHeap::new::<i32>();
		assert_eq!(heap.next(), None);
	}

	#[test]
	fn test_min_heap() {
		let mut heap = MinHeap::new();
		heap.add(4);
		heap.add(2);
		heap.add(9);
		heap.add(11);
		assert_eq!(heap.len(), 4);
		assert_eq!(heap.next(), Some(2));
		assert_eq!(heap.next(), Some(4));
		assert_eq!(heap.next(), Some(9));
		heap.add(1);
		assert_eq!(heap.next(), Some(1));
	}

	#[test]
	fn test_max_heap() {
		let mut heap = MaxHeap::new();
		heap.add(4);
		heap.add(2);
		heap.add(9);
		heap.add(11);
		assert_eq!(heap.len(), 4);
		assert_eq!(heap.next(), Some(11));
		assert_eq!(heap.next(), Some(9));
		assert_eq!(heap.next(), Some(4));
		heap.add(1);
		assert_eq!(heap.next(), Some(2));
	}
}