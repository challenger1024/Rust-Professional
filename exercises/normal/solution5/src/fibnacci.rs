pub fn odd_fibnacci_sum(threshold: u32) -> u32 {
    // TODO: 这里写逻辑
	if threshold==0{
		return 0;
	}
	if threshold==1{
		return 1;
	}
	let mut    ans=1;
	let mut a=0;
	let mut b=1;
	while a+b<threshold{
		if (a+b)%2==1{
			ans+=a+b;
		}
		let c=a+b;
		a=b;
		b=c;
	}
	ans
}
