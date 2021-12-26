use std::{
    env::args,
    error::Error,
    fs::{read, write},
    io::BufRead,
};

fn main() -> Result<(), Box<dyn Error>> {
    let argv = &args().into_iter().collect::<Vec<String>>();
    if argv.len() < 2 {
        return Err(Box::from("[On read] No file provided"));
    }
    let target = match read(&argv[1]) {
                        Ok(v) => v,
                        Err(_) => return Err(Box::from(
                            "[On read] Insufficient permissions or file does not exist. Check your spelling",
                        )),
                    };
    let use_lib = if argv.len() > 2 && &argv[2] == "--no-std" {
        false
    } else {
        true
    };
    let mut lib = if use_lib { stdlib() } else { (vec![], vec![]) };
    let mut new = String::new();
    let matches: &mut Vec<String> = &mut lib.0;
    let mut replaces: Vec<String> = lib.1;
    for line in target.lines() {
        if let Ok(line) = line {
            let mut chars = line.chars();
            match chars.nth(0) {
                Some('=') => {
                    let matchnreplace = parse_def(&line);
                    matches.push(matchnreplace.0);
                    replaces.push(matchnreplace.1);
                }
                Some('#') => {
                    let mut line = line;
                    let len = matches.len();
                    for (v, i) in matches.into_iter().zip(0..len) {
                        if line.contains(v.as_str()) {
                            line = line.replace(v.as_str(), replaces[i].as_str());
                        }
                    }
                    line = line.replace("/", "\n");
                    let matchnreplace = parse_def(&line.to_string());
                    matches.push(matchnreplace.0);
                    replaces.push(matchnreplace.1);
                }
                Some('-') => {
                    let target = match read(&line[1..]) {
                        Ok(v) => v,
                        Err(_) => return Err(Box::from(
                            "[On import] Insufficient permissions or file does not exist. Check your spelling",
                        )),
                    };
                    for line in target.lines() {
                        if let Ok(line) = line {
                            let mut chars = line.chars();
                            match chars.nth(0) {
                                Some('=') => {
                                    let matchnreplace = parse_def(&line);
                                    matches.push(matchnreplace.0);
                                    replaces.push(matchnreplace.1);
                                }
                                Some('#') => {
                                    let mut line = line;
                                    let len = matches.len();
                                    for (v, i) in matches.into_iter().zip(0..len) {
                                        if line.contains(v.as_str()) {
                                            line = line.replace(v.as_str(), replaces[i].as_str());
                                        }
                                    }
                                    line = line.replace("/", "\n");
                                    let matchnreplace = parse_def(&line.to_string());
                                    matches.push(matchnreplace.0);
                                    replaces.push(matchnreplace.1);
                                }
                                _ => continue,
                            }
                        }
                    }
                }
                Some('!') => {
                    let mut idx = 2;
                    let mut num = String::new();
                    while let Some(ch) = chars.next() {
                        if !ch.is_numeric() {
                            break;
                        }
                        num += format!("{}", ch).as_str();
                        idx += 1;
                    }
                    let mut line = line;
                    let len = matches.len();
                    for (v, i) in matches.into_iter().zip(0..len) {
                        if line.contains(v.as_str()) {
                            line = line.replace(v.as_str(), replaces[i].as_str());
                        }
                    }
                    line = line.replace("/", "\n");
                    for _ in 0..(num.parse()?) {
                        new += &format!("{}\n", line).as_str()[idx..];
                    }
                }
                _ => {
                    let mut line = line;
                    let len = matches.len();
                    for (v, i) in matches.into_iter().zip(0..len) {
                        if line.contains(v.as_str()) {
                            line = line.replace(v.as_str(), replaces[i].as_str());
                        }
                    }
                    line = line.replace("/", "\n");
                    new += format!("{}\n", line).as_str();
                }
            }
        }
    }
    write("./out.mcfunction", new.trim())?;
    Ok(())
}

fn parse_def(src: &String) -> (String, String) {
    let tmp = src.split_whitespace().collect::<Vec<&str>>();
    let mut pat: Vec<String> = vec![];
    pat.push(tmp[0].to_string());
    pat.push(tmp[1..].join(" "));

    (pat[0][1..].to_string(), pat[1].to_string())
}

fn stdlib() -> (Vec<String>, Vec<String>) {
    (
        vec![
            " self ".to_string(),
            " here ".to_string(),
            " do ".to_string(),
            "$".to_string(),
        ],
        vec![
            " as @s ".to_string(),
            " at @s ".to_string(),
            " execute ".to_string(),
            " run ".to_string(),
        ],
    )
}
