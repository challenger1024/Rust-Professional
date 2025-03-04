extern crate chrono;
use chrono::{NaiveDate, Datelike, Duration};


// 计算退休时间
pub fn retire_time(birth_date: &str, worker_type: &str) -> String {
    // 手动补充 `-01` 以符合 `"%Y-%m-%d"` 解析格式
    let birth_date_str = format!("{}-01", birth_date);
    let birth_date = NaiveDate::parse_from_str(&birth_date_str, "%Y-%m-%d")
        .expect("Invalid date format");

    // 定义基础退休年龄和延迟规则
    let (base_retirement_age, delay_factor) = match worker_type {
        "男职工" => (60, 4),
        "原法定退休年龄55周岁女职工" => (55, 4),
        "原法定退休年龄50周岁女职工" => (50, 2),
        _ => panic!("Unknown worker type"),
    };

    // 计算原本的退休日期
    let retirement_year = birth_date.year() + base_retirement_age;
    let retirement_date = birth_date.with_year(retirement_year).unwrap();

    // 计算从2025年1月到原本退休日期的月数差 (a)
    let reference_date = NaiveDate::from_ymd(2025, 1, 1);
    let months_diff = (retirement_year - reference_date.year()) * 12 + (retirement_date.month() as i32 - reference_date.month() as i32);
    let a = months_diff + 1; // 根据公式，a = 原定退休时间 - 2025年1月 + 1

    // 计算推迟的月数
    let delay_months = if worker_type == "原法定退休年龄50周岁女职工" {
        a / 2 // 50周岁女职工的推迟规则，每2个月推迟1个月
        if months_to_delay > 60 {
            60 // 最大延迟60个月
        } else {
            months_to_delay
        }
    } else {
        let months_to_delay = a / 4;
        if months_to_delay > 36 {
            36 // 最大延迟36个月
        } else {
            months_to_delay
        }
    };

    // 计算新的退休日期
    let retirement_date = retirement_date
        .checked_add_signed(Duration::days((delay_months * 30) as i64)) // 假设每月30天
        .unwrap();

    // 计算退休年龄
    let retirement_age = (retirement_date.year() - birth_date.year()) as f64 +
        (retirement_date.ordinal() as f64 - birth_date.ordinal() as f64) / 365.25;

    // 格式化输出
    let retirement_date_str = retirement_date.format("%Y-%m").to_string();
    format!("{}, {:.2}, {}", retirement_date_str, retirement_age, delay_months)
}
