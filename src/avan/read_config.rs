use std::fs::File;
use std::io::{self, BufRead};


// Вспомогательная функция для чтения файла и поиска нужной строки
fn read_config(keyword: &str) -> Option<String> {
    if let Ok(file) = File::open("config.txt") {
        for line in io::BufReader::new(file).lines() {
            if let Ok(line) = line {
                if line.starts_with(keyword) {
                    return Some(line[keyword.len()..].trim_matches('"').trim().to_string());
                }
            }
        }
    }
    None
}

pub fn get_api_key() -> String {
    read_config("API Key Steam: \"").unwrap_or_default()
}

pub fn get_trade_url() -> String {
    read_config("Ссылка на обмен: \"").unwrap_or_default()
}

pub fn get_percent_value() -> f64 {
    read_config("Желаемый % пополнения: \"")
        .and_then(|val| val.parse().ok())
        .unwrap_or(0.0)
}

pub fn get_smp_value() -> f64 {
    read_config("Кол-во продаж в месяц: \"")
        .and_then(|val| val.parse().ok())
        .unwrap_or(0.0)
}

pub fn get_login() -> String {
    read_config("login: \"").unwrap_or_default()
}


pub fn get_password() -> String {
    read_config("pass: \"").unwrap_or_default()
}
