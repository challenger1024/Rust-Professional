/*
	single linked list merge
	This problem requires you to merge two ordered singly linked lists into one ordered singly linked list
*/

use std::cmp::PartialOrd;
use std::fmt::Display;
use std::fmt::{self, /*Display,*/ Formatter};
use std::ptr::NonNull;
//use std::vec::*;

#[derive(Debug,PartialEq)]
struct Node<T: Display + PartialOrd > {
	val: T,
	next: Option<NonNull<Node<T>>>,
}
/* 
impl<T: Display + PartialOrd +Clone> PartialOrd for Node<T> {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.val.partial_cmp(&other.val)?)
	}
}
*/
impl<T: Display + PartialOrd > Node<T> {
	fn new(t: T) -> Node<T> {
		Node {
			val: t,
			next: None,
		}
	}
}
#[derive(Debug)]
struct LinkedList<T: Display+ PartialOrd +Clone> {
	length: u32,
	start: Option<NonNull<Node<T>>>,
	end: Option<NonNull<Node<T>>>,
}

impl<T: Display + PartialOrd + Clone> Default for LinkedList<T> {
	fn default() -> Self {
		Self::new()
	}
}

impl<T:Display  +  PartialOrd +Clone> LinkedList<T> {
	pub fn new() -> Self {
		Self {
			length: 0,
			start: None,
			end: None,
		}
	}

	pub fn add(&mut self, obj: T) {
		let mut node = Box::new(Node::new(obj));
		node.next = None;
		let node_ptr = Some(unsafe { NonNull::new_unchecked(Box::into_raw(node)) });
		match self.end {
			None => self.start = node_ptr,
			Some(end_ptr) => unsafe { (*end_ptr.as_ptr()).next = node_ptr },
		}
		self.end = node_ptr;
		self.length += 1;
	}

	pub fn get(&mut self, index: i32) -> Option<&T> {
		self.get_ith_node(self.start, index)
	}

	fn get_ith_node(&mut self, node: Option<NonNull<Node<T>>>, index: i32) -> Option<&T> {
		match node {
			None => None,
			Some(next_ptr) => match index {
				0 => Some(unsafe { &(*next_ptr.as_ptr()).val }),
				_ => self.get_ith_node(unsafe { (*next_ptr.as_ptr()).next }, index - 1),
			},
		}
	}
	
/* 
	pub fn merge( list_a: LinkedList<T>,  list_b: LinkedList<T>) -> Self
	{
		//TOdo
		let mut dummy = Node::new(unsafe { std::mem::zeroed() }); // 用 zeroed() 初始化一个“空”值
		let  ptr_a= &list_a.start;
		let  ptr_b=& list_b.start;
		let mut cur=&mut dummy;
		while let (Some(node1),Some(node2))=(ptr_a,ptr_b){
			let n1=unsafe{node1.as_ref()};
			let n2=unsafe{node2.as_ref()};
			if n1.val < n2.val{
				cur.next=ptr_a.take();
				cur=cur.next.as_mut().map(|nn| unsafe{nn.as_mut()}).expect("REASON");
				ptr_a=&cur.next.take();
			}else{
				cur.next=ptr_b.take();
				cur=cur.next.as_mut().map(|nn| unsafe{nn.as_mut()}).expect("REASON");
				ptr_b=&cur.next.take();
			}
		}
		cur.next=(*ptr_a).or(*ptr_b);
		while let Some(node)=cur.next.as_mut(){
			cur=unsafe{node.as_mut()};
		}

		Self {
			length:  list_a.length+list_b.length,
			start: dummy.next,
			end: cur.next,
		}
	}
*/
	pub fn merge( list_a: LinkedList<T>,  list_b: LinkedList<T>) -> Self{
		let mut ans=LinkedList::<T>::new();
		let  mut ptr_a= list_a.start;
		let  mut ptr_b= list_b.start;
		while let (Some(mut node1),Some(mut node2))=(ptr_a,ptr_b) {
			let n1=unsafe{node1.as_mut()};
			let n2=unsafe{node2.as_mut()};
			if n1.val<n2.val{
				ans.add(n1.val.clone());
				ptr_a=n1.next.take();
			}else{
				ans.add(n2.val.clone());
				ptr_b=n2.next.take();
			}
		}
		while let Some(mut node1)=ptr_a{
			let n1=unsafe{node1.as_mut()};
			ans.add(n1.val.clone());
			ptr_a=n1.next.take();
		}
		while let Some(mut node2)=ptr_b{
			let n2=unsafe{node2.as_mut()};
			ans.add(n2.val.clone());
			ptr_b=n2.next.take();
		}
		ans
	}

}

impl<T> Display for LinkedList<T>
where
	T: Display + PartialOrd + Clone,
{
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		match self.start {
			Some(node) => write!(f, "{}", unsafe { node.as_ref() }),
			None => Ok(()),
		}
	}
}

impl<T> Display for Node<T>
where
	T: Display + PartialOrd,
{
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		match self.next {
			Some(node) => write!(f, "{}, {}", self.val, unsafe { node.as_ref() }),
			None => write!(f, "{}", self.val),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::LinkedList;

	#[test]
	fn create_numeric_list() {
		let mut list = LinkedList::<i32>::new();
		list.add(1);
		list.add(2);
		list.add(3);
		println!("Linked List is {}", list);
		assert_eq!(3, list.length);
	}

	#[test]
	fn create_string_list() {
		let mut list_str = LinkedList::<String>::new();
		list_str.add("A".to_string());
		list_str.add("B".to_string());
		list_str.add("C".to_string());
		println!("Linked List is {}", list_str);
		assert_eq!(3, list_str.length);
	}

	#[test]
	fn test_merge_linked_list_1() {
		let mut list_a = LinkedList::<i32>::new();
		let mut list_b = LinkedList::<i32>::new();
		let vec_a = vec![1,3,5,7];
		let vec_b = vec![2,4,6,8];
		let target_vec = vec![1,2,3,4,5,6,7,8];
		
		for i in 0..vec_a.len(){
			list_a.add(vec_a[i]);
		}
		for i in 0..vec_b.len(){
			list_b.add(vec_b[i]);
		}
		println!("list a {} list b {}", list_a,list_b);
		let mut list_c = LinkedList::<i32>::merge(list_a,list_b);
		println!("merged List is {}", list_c);
		for i in 0..target_vec.len(){
			assert_eq!(target_vec[i],*list_c.get(i as i32).unwrap());
		}
	}

	#[test]
	fn test_merge_linked_list_2() {
		let mut list_a = LinkedList::<i32>::new();
		let mut list_b = LinkedList::<i32>::new();
		let vec_b = vec![11,33,44,88,89,90,100];
		let vec_a = vec![1,22,30,45];
		let target_vec = vec![1,11,22,30,33,44,45,88,89,90,100];

		for i in 0..vec_a.len(){
			list_a.add(vec_a[i]);
		}
		for i in 0..vec_b.len(){
			list_b.add(vec_b[i]);
		}
		println!("list a {} list b {}", list_a,list_b);
		let mut list_c = LinkedList::<i32>::merge(list_a,list_b);
		println!("merged List is {}", list_c);
		for i in 0..target_vec.len(){
			assert_eq!(target_vec[i],*list_c.get(i as i32).unwrap());
		}
	}
}