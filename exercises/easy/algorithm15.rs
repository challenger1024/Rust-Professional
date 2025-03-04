#![allow(unused_imports)]

/*
	Longest Substring Without Repeating Characters
	Given a string, find the length of the longest substring without repeating characters. 
	The substring must not contain any duplicate characters, and its length should be maximized.

	You need to implement the function `longest_substring_without_repeating_chars(s: String) -> i32`.
	The function should return the length of the longest substring without repeating characters.
	
	Hint: Consider using the sliding window technique to efficiently solve this problem in O(n) time complexity.
*/

use std::fmt::{self, Display, Formatter};
use std::cmp;
pub fn longest_substring_without_repeating_chars(s: String) -> i32 {
	// TODO: Implement the logic to find the longest substring without repeating characters
	let mut cnt=vec![0;26];
	let mut l: i32=0;
	let mut r: i32 =0;
	let  v: Vec<char> =s.chars().collect();
	let mut ans:i32 =0;
	while (r as usize) <v.len(){
		cnt[(v[r as usize] as u8 -'a' as u8) as usize]+=1;
		while l<r && cnt[(v[r as usize] as u8 -'a' as u8) as usize]>1 {
			cnt[(v[l as usize] as u8 -'a' as u8) as usize]-=1;
			l+=1;
		}
		ans=cmp::max(ans,r-l+1) as i32;
		r+=1;
	}
	ans // Placeholder return value
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_longest_substring_1() {
		let s = "abcabcbb".to_string();
		let result = longest_substring_without_repeating_chars(s);
		println!("Length of longest substring: {}", result);
		assert_eq!(result, 3);  // "abc"
	}

	#[test]
	fn test_longest_substring_2() {
		let s = "bbbbb".to_string();
		let result = longest_substring_without_repeating_chars(s);
		println!("Length of longest substring: {}", result);
		assert_eq!(result, 1);  // "b"
	}

	#[test]
	fn test_longest_substring_3() {
		let s = "pwwkew".to_string();
		let result = longest_substring_without_repeating_chars(s);
		println!("Length of longest substring: {}", result);
		assert_eq!(result, 3);  // "wke"
	}

	#[test]
	fn test_longest_substring_4() {
		let s = "".to_string();
		let result = longest_substring_without_repeating_chars(s);
		println!("Length of longest substring: {}", result);
		assert_eq!(result, 0);  // Empty string
	}

	#[test]
	fn test_longest_substring_5() {
		let s = "abcde".to_string();
		let result = longest_substring_without_repeating_chars(s);
		println!("Length of longest substring: {}", result);
		assert_eq!(result, 5);  // "abcde"
	}
}
