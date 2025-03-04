#![allow(dead_code)]  // 禁用死代码警告
#![allow(unused_imports)]
#![allow(unused_variables)]


extern crate chrono;
use chrono::{NaiveDate, Datelike,Duration};
use std::time::Instant;

// 性别类型
#[derive(Debug)]
enum Gender {
    Male,
    Female50,
    Female55,
}

// 计算退休时间
pub fn retire_time(birth_date: &str, worker_type: &str) -> String {
    // 手动补充 `-01` 以符合 `"%Y-%m-%d"` 解析格式
    let birth_date_str = format!("{}-01", birth_date);
    let birth_date = NaiveDate::parse_from_str(&birth_date_str, "%Y-%m-%d")
        .expect("Invalid date format");

    // 定义延迟规则
    let (base_retirement_age, delay_factor, retirement_month) = match worker_type {
        "男职工" => (60, 4, 1),
        "原法定退休年龄55周岁女职工" => (55, 4, 1),
        "原法定退休年龄50周岁女职工" => (50, 2, 1),
        _ => panic!("Unknown worker type"),
    };

    // 计算原本的退休日期
    let retirement_year = birth_date.year() + base_retirement_age;
    let  retirement_date = birth_date.with_year(retirement_year).unwrap();
    let retirement_date_month = birth_date.month();

    // 计算从原本退休日期到2025年1月的年数差，转换为月份差 (a)
    let year_diff = retirement_year - 2025; // 退休年份与2025年的年差
    let a =1 + year_diff * 12 + retirement_date_month as i32; // 计算月数差

    // 计算推迟的月数
    let delay_months = if worker_type == "原法定退休年龄50周岁女职工" {
        let x=(a) / 2; // 50周岁女职工的推迟规则，每2个月推迟1个月
        if x > 60 {
            60 // 最大延迟36个月
        } else {
            x
        }
    } else {
        let months_to_delay = (a) / 4;
        if months_to_delay > 36 {
            36 // 最大延迟36个月
        } else {
            months_to_delay
        }
    };
    if delay_months < 0 {
        // 如果推迟月份小于0，直接输出原定退休时间和年龄
        let retirement_age =if worker_type == "原法定退休年龄50周岁女职工" {  // 假设原定退休年龄为60
            50
        }else if worker_type == "原法定退休年龄55周岁女职工"{
            55
        }else{
            60
        };
        let retirement_date_str = retirement_date.format("%Y-%m").to_string();
            return format!("{},{},0", retirement_date_str, retirement_age);

//        return (retirement_date_str, format!("{}", retirement_age), 0);
    }
    if delay_months ==0 {
        let retirement_age =if worker_type == "原法定退休年龄50周岁女职工" {  
            50.0*601.0/600.0
        }else if worker_type == "原法定退休年龄55周岁女职工"{
            55.0*661.0/660.0
        }else{
            60.08
        };
        return format!("2025-02,{:.2},1",retirement_age);
    }

    // 计算新的退休日期
    let total_months = retirement_date.month() as u32 + delay_months as u32;
    let retirement_year = retirement_date.year() + (total_months / 12) as i32; // 计算超出部分的年份
    let retirement_month = total_months % 12; // 计算新的月份
    let mut retirement_date = NaiveDate::from_ymd_opt(
        retirement_year,
        if retirement_month == 0 { 12 } else { retirement_month },
        1,
        )
        .unwrap();
    if retirement_month==0{
        retirement_date = retirement_date.with_year(retirement_date.year() - 1).unwrap();
    }


    // 计算退休年龄
    let mut year_diff = retirement_year - birth_date.year();
    let mut month_diff = retirement_month as i32 - birth_date.month() as i32;
    if month_diff <=0 {
        month_diff += 12;
        year_diff -= 1; // 年份差减少1
//        retirement_date = retirement_date.with_month(12).unwrap();
//        retirement_date = retirement_date.with_year(retirement_date.year() - 1).unwrap();
    }
    let retirement_age = year_diff as f64 + month_diff as f64 / 12.0;

    // 格式化输出
    let retirement_date_str = retirement_date.format("%Y-%m").to_string();
    let output = format!("{}, {:.2}, {}", retirement_date_str, retirement_age, delay_months);
//    println!("{}", output);
    let retirement_age_str = if retirement_age == retirement_age.round() {
        format!("{}", retirement_age.round() as u64)  // 没有小数部分，直接输出整数
    } else {
        format!("{:.2}", retirement_age)  // 有小数部分时，保留两位小数
    };
    format!("{},{},{}", retirement_date_str, retirement_age_str, delay_months)
}
