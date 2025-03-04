#![allow(unused_imports)]

/*
	
Merge Intervals
	
Given an array of intervals where each interval is represented by a pair of integers [start, end], 
	
merge all overlapping intervals and return a list of non-overlapping intervals.
	

	
The intervals are inclusive, meaning the interval [start, end] includes both start and end points.
	

	
You need to implement the function `merge_intervals(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>>`.
	
The function should return a vector containing all the merged intervals.

	
Hint: You can start by sorting the intervals by their starting point and then merge them one by one.
*/

use std::fmt::{self, Display, Formatter};
use std::cmp::max;
pub fn merge_intervals(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
// TODO: Implement the logic to merge overlapping intervals
	intervals.sort();
	let mut ans: Vec<Vec<i32>>=Vec::new();
	for (i,interval) in intervals.iter().enumerate(){
		if i==0{
			ans.push(interval.clone());
		}else if interval[0] <=ans[ans.len()-1][1]{
			let last_row = ans.last_mut().unwrap();  // 获取最后一行的可变引用
			last_row[1] = last_row[1].max(interval[1]);  // 修改第二个元素
//			ans[ans.len()-1][1]=max(interval[1],ans[ans.len()-1][1]);
		}else if interval[0] > ans[ans.len()-1][1]{
			ans.push(interval.clone());
		}
	}
ans // Placeholder return value
}

#[cfg(test)]
mod tests {
	
use super::*;

	
#[test]
	
fn test_merge_intervals_1() {
	
	
let intervals = vec![
	
	
	
vec![1, 3],
	
	
	
vec![2, 6],
	
	
	
vec![8, 10],
	
	
	
vec![15, 18]
	
	
];
	
	
let result = merge_intervals(intervals);
	
	
println!("Merged intervals: {:?}", result);
	
	
assert_eq!(result, vec![
	
	
	
vec![1, 6],
	
	
	
vec![8, 10],
	
	
	
vec![15, 18]
	
	
]);
	
}

	
#[test]
	
fn test_merge_intervals_2() {
	
	
let intervals = vec![
	
	
	
vec![1, 4],
	
	
	
vec![4, 5]
	
	
];
	
	
let result = merge_intervals(intervals);
	
	
println!("Merged intervals: {:?}", result);
	
	
assert_eq!(result, vec![
	
	
	
vec![1, 5]
	
	
]);
	
}

	
#[test]
	
fn test_merge_intervals_3() {
	
	
let intervals = vec![
	
	
	
vec![1, 4],
	
	
	
vec![0, 4]
	
	
];
	
	
let result = merge_intervals(intervals);
	
	
println!("Merged intervals: {:?}", result);
	
	
assert_eq!(result, vec![
	
	
	
vec![0, 4]
	
	
]);
	
}

	
#[test]
	
fn test_merge_intervals_4() {
	
	
let intervals = vec![
	
	
	
vec![1, 10],
	
	
	
vec![2, 6],
	
	
	
vec![8, 10]
	
	
];
	
	
let result = merge_intervals(intervals);
	
	
println!("Merged intervals: {:?}", result);
	
	
assert_eq!(result, vec![
	
	
	
vec![1, 10]
	
	
]);
	
}

	
#[test]
	
fn test_merge_intervals_5() {
	
	
let intervals = vec![
	
	
	
vec![1, 2],
	
	
	
vec![3, 5],
	
	
	
vec![4, 7],
	
	
	
vec![8, 10]
	
	
];
	
	
let result = merge_intervals(intervals);
	
	
println!("Merged intervals: {:?}", result);
	
	
assert_eq!(result, vec![
	
	
	
vec![1, 2],
	
	
	
vec![3, 7],
	
	
	
vec![8, 10]
	
	
]);
	
}
}
