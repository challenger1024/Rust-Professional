#![allow(unused_imports)]

/*
	
Nth Fibonacci Number
	
Implement a function to calculate the `n`th Fibonacci number. 
	
The Fibonacci sequence is defined as follows:
	
F(0) = 0, F(1) = 1, F(n) = F(n-1) + F(n-2) for n > 1.

	
You need to implement the function `fib(n: i32) -> i32` to return the `n`th Fibonacci number.
	

	
Hint: Consider using matrix exponentiation to solve the problem in O(log n) time complexity.
*/

use std::fmt::{self, Display, Formatter};

fn mat_mult(a: &Vec<Vec<u64>>, b: &Vec<Vec<u64>>) -> Vec<Vec<u64>> {
	let mut res=vec![vec![0;2];2];
	for i in 0..2{
		for j in 0..2{
			for k in 0..2{
				res[i][j]+=a[i][k]*b[k][j];
			}
		}
	}
	res
}
fn mat_pow(mut base: Vec<Vec<u64>>, mut exp: u64) -> Vec<Vec<u64>> {
	let mut result = vec![vec![1, 0], vec![0, 1]]; // 单位矩阵
	while exp>0 {
		if exp%2==1{
			result=mat_mult(&result, &base);
		}
		base=mat_mult(&base, &base);
		exp/=2;
	}
	result
}

pub fn fib(n: i32) -> i32 {
// TODO: Implement the logic to calculate the nth Fibonacci number using matrix exponentiation
	if n==0{
		return 0;
	}
	let base=vec![vec![1,1],vec![1,0]];
	let result=mat_pow(base, (n-1) as u64);
	result[0][0] as i32 // Placeholder return value
}

#[cfg(test)]
mod tests {
	
use super::*;

	
#[test]
	
fn test_fib_1() {
	
	
let result = fib(0);
	
	
println!("Fibonacci of 0: {}", result);
	
	
assert_eq!(result, 0);
	
}

	
#[test]
	
fn test_fib_2() {
	
	
let result = fib(1);
	
	
println!("Fibonacci of 1: {}", result);
	
	
assert_eq!(result, 1);
	
}

	
#[test]
	
fn test_fib_3() {
	
	
let result = fib(2);
	
	
println!("Fibonacci of 2: {}", result);
	
	
assert_eq!(result, 1);
	
}

	
#[test]
	
fn test_fib_4() {
	
	
let result = fib(3);
	
	
println!("Fibonacci of 3: {}", result);
	
	
assert_eq!(result, 2);
	
}

	
#[test]
	
fn test_fib_5() {
	
	
let result = fib(10);
	
	
println!("Fibonacci of 10: {}", result);
	
	
assert_eq!(result, 55);
	
}

	
#[test]
	
fn test_fib_6() {
	
	
let result = fib(20);
	
	
println!("Fibonacci of 20: {}", result);
	
	
assert_eq!(result, 6765);
	
}
}
