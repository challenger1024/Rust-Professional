

pub fn goldbach_conjecture() -> String {
	let limit=10000;
	let mut is_prime =vec![true;limit+1];//筛选数组
	let mut primes=Vec::new();//存储质数
	for i in 2..=(limit){
		if is_prime[i]{
			//如果经过筛选后的i仍然是质数，加入到质数数组中
			primes.push(i);
		}
		//便利已有的每个质数，作为筛选因子
		for &p in &primes{
			if i*p>limit{
				//防止溢出
				break;
			}
			is_prime[i*p]=false;
			if i%p==0{
				break;
			}
		}
	}
	let mut i=9;
	let mut ans=Vec::new();
//	dbg!(&primes);
	while i<limit && ans.len()<2{
		if is_prime[i]{
			i+=2;
			continue;
		}
		let mut flag=false;
		for &p in &primes{
			if p>=i /*|| (i-p)%2!=0 */{
				break;
			}
		if (i-p)%2!=0{
			continue;
		}
//			let mut flag=false;
			let k=((i as f64 -p as f64)/2 as f64).sqrt();
			dbg!(k);
			if k.fract()==0.0 /*&& (2 as f64) *(k*k)+p as f64 ==i as f64*/ {
				flag=true;
			}
			if flag{
//				ans.push(i);
			break;
			}
		}
		if !flag{
			ans.push(i);
		}
		i+=2;
	}
	let result=ans.iter()
		.map(|n| n.to_string())
		.collect::<Vec<String>>()
		.join(",");
	result
}
