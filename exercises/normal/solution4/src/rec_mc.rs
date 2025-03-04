use std::cmp::min;
//use std::cmp::max;
pub fn dp_rec_mc(amount: u32) -> u32 {
    // TODO: 这里写逻辑
	let money: Vec<u32> =vec![1,2,5,10,20,30,50,100];
//	let n=money.len();
	let mut dp: Vec<u32> =vec![i32::MAX as u32 ;(amount+1) as usize];
	dp[0]=0;
	for num in money{
		for i in num..(amount+1){//num比amount小
			dp[i as usize]=min(dp[i as usize] as usize,(dp[(i-num) as usize]+1) as usize) as u32;
		}
	}
	dp[amount as usize]
}
