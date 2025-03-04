#![allow(unused_imports)]

/*
	Rotate Matrix 90 Degrees
	Given a 2D matrix, rotate it 90 degrees in place. 
	You need to perform the rotation without using any additional matrix storage.

	You need to implement the function `rotate_matrix_90_degrees(matrix: &mut Vec<Vec<i32>>)`.
	The function should rotate the input matrix in place.

	Hint: Consider rotating the matrix layer by layer, starting from the outermost layer and working your way inward.
*/

use std::fmt::{self, Display, Formatter};
use std::mem;
use std::cmp::max;

pub fn rotate_matrix_90_degrees(matrix: &mut Vec<Vec<i32>>) {
	// TODO: Implement the logic to rotate the matrix 90 degrees in place
	let m=matrix.len();
	if m==0{
		return ;
	}
	let n=matrix[0].len();
	let x=max(m,n);
	matrix.resize(x, vec![0; matrix[0].len()]);
	for row in& mut  *matrix{
		row.resize(x, 0);  // 将每一行扩展到长度为x，填充默认值 0
	}
	for e in 0..(x/2){
		for i in e..(e+1){
			for j in e..(x-e-1){
				let temp=matrix[j][x-i-1];
				matrix[j][x-i-1]=matrix[i][j];
				matrix[i][j]=matrix[x-j-1][i];
				matrix[x-j-1][i]= matrix[x-i-1][x-j-1] ;
				matrix[x-i-1][x-j-1]=temp;
			}
		}
	}
	if n>m{
		for row in& mut  *matrix{
			row.drain(0..(n-m));
		}
	}
	if m>n{
		matrix.resize(n, vec![0; matrix[0].len()]);
	}

}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_rotate_matrix_1() {
		let mut matrix = vec![
			vec![1, 2, 3],
			vec![4, 5, 6],
			vec![7, 8, 9],
		];
		rotate_matrix_90_degrees(&mut matrix);
		println!("Rotated matrix: {:?}", matrix);
		assert_eq!(matrix, vec![
			vec![7, 4, 1],
			vec![8, 5, 2],
			vec![9, 6, 3],
		]);
	}

	#[test]
	fn test_rotate_matrix_2() {
		let mut matrix = vec![
			vec![1, 2],
			vec![3, 4],
		];
		rotate_matrix_90_degrees(&mut matrix);
		println!("Rotated matrix: {:?}", matrix);
		assert_eq!(matrix, vec![
			vec![3, 1],
			vec![4, 2],
		]);
	}

	#[test]
	fn test_rotate_matrix_3() {
		let mut matrix = vec![
			vec![1],
		];
		rotate_matrix_90_degrees(&mut matrix);
		println!("Rotated matrix: {:?}", matrix);
		assert_eq!(matrix, vec![
			vec![1],
		]);
	}

	#[test]
	fn test_rotate_matrix_4() {
		let mut matrix = vec![
			vec![1, 2],
			vec![3, 4],
			vec![5, 6],
		];
		rotate_matrix_90_degrees(&mut matrix);
		println!("Rotated matrix: {:?}", matrix);
		assert_eq!(matrix, vec![
			vec![5, 3, 1],
			vec![6, 4, 2],
		]);
	}
}
