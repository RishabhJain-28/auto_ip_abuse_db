use std::{fmt::format, fs};

#[tokio::main]
async fn main() {
    let f = fs::read_to_string("ips.txt").expect("File not found");

    let ip_vec: Vec<String> = f
        .split("\r")
        .map(|v| String::from(v.trim()))
        .filter(|v| !v.is_empty())
        .collect();

    let l = ip_vec.len();
    let mut final_res: Vec<(usize, String, u32, u32)> = vec![];
    let mut sleep_count = 0;
    for (i, ip) in ip_vec.into_iter().enumerate() {
        if sleep_count == 45 {
            sleep_count = 0;
            println!("sleeping");
            tokio::time::sleep(tokio::time::Duration::new(60, 0)).await
        }

        let res = check_ip(&ip).await;

        if res.is_err() {
            println!("ERROR {}: {}", i, res.as_ref().unwrap_err());
        }
        let res = res.unwrap();
        println!(
            "done: {}, total: {}, {}: b- {}, c- {} p- {} ",
            i, l, &ip, res.0, res.1, res.2
        );
        if res.0 {
            final_res.push((i, ip, res.1, res.2));
        }
        sleep_count += 1;
    }

    let a = tokio::fs::write(
        format!(".\\out\\final",),
        final_res.iter().fold(String::new(), |acc, v| {
            acc + &format!("{0: <5} - {1: <15} c- {2: <5} p- {3}", v.0, v.1, v.2, v.3) + "\n"
        }),
    )
    .await;
    if a.is_err() {
        println!("ERROR cant create output file");
    }
}

async fn check_ip(ip: &str) -> Result<(bool, u32, u32), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let body = client
        .get(format!("https://www.abuseipdb.com/check/{}", ip))
        .send()
        .await?
        .text()
        .await?;

    if !body.contains("was not found in our database") {
        // tokio::fs::write(format!(".\\out\\{}", ip), &body).await?;
        let times_pattern = r#"</b> times."#;
        let per_pattern = r#"%</b>:"#;

        return Ok((true, parse(&body, times_pattern), parse(&body, per_pattern)));
    }
    Ok((false, 0, 0))
}

fn parse(body: &str, pattern: &str) -> u32 {
    let times = body.find(pattern).unwrap();
    let mut res = String::new();
    let mut index = times - 1;

    while body[index..index + 1].ne(">") {
        res += &body[index..index + 1];
        index -= 1;
    }
    res = res.trim().to_string().chars().rev().collect();
    res.parse::<u32>().unwrap()
}
