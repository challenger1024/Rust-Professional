pub fn convert_base(num_str: &str, to_base: u32) -> String {
    // TODO: 这里写逻辑
	let start:usize;
	if let Some(index) =num_str.find('('){
		start =index;
	}else{
		return "".to_string();
	}
	let end:usize;
	if let Some(index)=num_str.find(')'){
		end=index;
	}else{
		return "".to_string();
	}
	let x_str = &num_str[start + 1..end]; // 提取括号内的字符串
	println!("{}",x_str);
	//let x: i32 = x_str.parse::<i32>().ok()?; // 解析成整数
	let Ok(x) = x_str.parse::<i32>().map_err(|e| e.to_string()) else {return "a".to_string() };  


	let mut middle: u32 =0;
	let v: Vec<char> =num_str[0..start].chars().collect();
	for c in v{
		if c>='0' &&c<='9'{
			let Ok(digit) = c.to_digit(10).ok_or_else(|| "无法将字符转换为数字".to_string()) else {return  "a".to_string(); };
			middle=middle*(x as u32)+(digit as u32);
		}else{
			middle=middle*(x as u32) +((c as u8 -'a' as u8) as u32 +10);
		}
	}
	let mut ans=String::new();
	let mut v_ans=Vec::new();
	while middle>0{
		v_ans.push(middle%to_base);
		middle/=to_base ;
	}
	let has=vec!['a','b','c','d','e','f'];
	for &item in v_ans.iter().rev() {
		if item<10{
			let Ok(c) = char::from_digit(item as u32, 10).ok_or_else(|| format!("无法转换数字 {} 为字符", item)) else { return "a".to_string(); };
			ans.push(c) ;
		}else{
			ans.push(has[(item-10) as usize]);
		}
	}
	ans
}
