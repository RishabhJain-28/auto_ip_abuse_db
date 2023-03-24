use std::{fmt::format, fs};

#[tokio::main]
async fn main() {
    let f = fs::read_to_string("ips.txt").expect("File not found");

    let ip_vec: Vec<String> = f.split("\r").map(|v| String::from(v.trim())).collect();
    // println!("vec: {:?}", ip_vec);

    let l = ip_vec.len();
    let mut final_res: Vec<String> = vec![];
    let mut sleep_count = 0;
    for (i, ip) in ip_vec.into_iter().enumerate() {
        // tokio::spawn();

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
        println!("done: {}, total: {}, {}: {}", i, l, &ip, res);
        if res {
            final_res.push(ip);
        }
        sleep_count += 1;
    }

    let a = tokio::fs::write(
        format!(".\\out\\final",),
        final_res
            .iter()
            .fold(String::new(), |acc, v| acc + v + "\n"),
    )
    .await;
    if a.is_err() {
        println!("ERROR cant create output file");
    }
}

// async fn check(ip: String, i: usize, total: usize) {

// }

async fn check_ip(ip: &str) -> Result<bool, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let body = client
        .get(format!("https://www.abuseipdb.com/check/{}", ip))
        .send()
        .await?
        .text()
        .await?;

    if !body.contains("was not found in our database") {
        // println!("{}", ip);
        tokio::fs::write(format!(".\\out\\{}", ip), body).await?;
        return Ok(true);
    }
    Ok(false)
}
