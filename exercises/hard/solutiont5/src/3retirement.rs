pub fn retire_time(birth_date: &str, worker_type: &str) -> String {
    // 解析出生年月
    let parts: Vec<&str> = birth_date.split('-').collect();
    let birth_year: i32 = parts[0].parse().expect("Invalid year format");
    let birth_month: i32 = parts[1].parse().expect("Invalid month format");
    
    // 计算原定退休年龄
    let base_retirement_age = match worker_type {
        "男职工" => 60,
        "原法定退休年龄55周岁女职工" => 55,
        "原法定退休年龄50周岁女职工" => 50,
        _ => panic!("Unknown worker type"),
    };
    
    // 计算原定退休时间
    let retirement_year = birth_year + base_retirement_age;
    let retirement_month = birth_month;
    let retirement_total_months = retirement_year * 12 + retirement_month;
    
    // 计算 2025 年 1 月的总月份值
    let reference_months = 2025 * 12 + 1;
    let month_diff = retirement_total_months - reference_months;
    
    // 计算推迟月数
    let delay_months = if worker_type == "原法定退休年龄50周岁女职工" {
        (month_diff / 2).min(60).max(0)
    } else {
        (month_diff / 4).min(36).max(0)
    };
    
    // 计算最终退休时间
    let final_total_months = retirement_total_months + delay_months;
    let final_year = final_total_months / 12;
    let final_month = final_total_months % 12;
    let final_retirement_date = format!("{}-{:02}", final_year, final_month);
    
    // 计算最终退休年龄
    let final_age = base_retirement_age as f64 + delay_months as f64 / 12.0;
    
    format!("{}, {:.2}, {}", final_retirement_date, final_age, delay_months)
}
