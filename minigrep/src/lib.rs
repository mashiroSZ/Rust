// 用于文件路径处理
use std::{env, fs, error::Error};

pub struct Config {
    query: String,
    filename: String,
    case_sensitive: bool,
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // 读取文件内容
    let contents = fs::read_to_string(config.filename)?;

    // 搜索内容并打印结果
    for line in search(&config.query, &contents, config.case_sensitive) {
        println!("{}", line);
    }

    Ok(())
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();
        
        // 读取命令行参数来设置 case_sensitive
        let case_sensitive = match args.get(3).map(|s| s.as_str()) {
            Some("1") => true,   // 大小写敏感
            Some("0") => false,  // 不区分大小写
            _ => true,           // 默认设置为大小写敏感
        };

        Ok(Config { query, filename, case_sensitive })
    }
}

pub fn search<'a>(query: &str, contents: &'a str, case_sensitive: bool) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if (case_sensitive && line.contains(query)) ||
           (!case_sensitive && line.to_lowercase().contains(&query.to_lowercase())) {
            results.push(line);
        }
    }

    results
}
