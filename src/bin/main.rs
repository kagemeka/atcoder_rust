// use reqwest;

use std::collections::HashMap;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // async fn main() -> anyhow::Result<()> {
    println!("main: Hello, world!");
    let response =
        reqwest::get("https://atcoder.jp/contests/abc003/submissions")
            .await?
            .text()
            .await?;
    let html = scraper::Html::parse_document(&response);
    let a_selector = scraper::Selector::parse("a").unwrap();
    let tbody_selector = scraper::Selector::parse("tbody").unwrap();
    let tr_selector = scraper::Selector::parse("tr").unwrap();
    let td_selector = scraper::Selector::parse("td").unwrap();

    let v: Result<Vec<u64>, Box<dyn std::error::Error>> = html
        .select(&tbody_selector)
        .next()
        .ok_or("cannot find submissions table")? // .unwrap()
        .select(&tr_selector)
        .map(|tr| {
            let mut tds = tr.select(&td_selector);
            let time = tds
                .next()
                .ok_or("Failed to parse html.")?
                .text()
                .next()
                .ok_or("Failed to parse html.")?;
            let time =
                chrono::DateTime::parse_from_str(time, "%Y-%m-%d %H:%M:%S%z")?;
            let epoch_second = time.timestamp() as u64;
            Ok(epoch_second)
        })
        .collect();

    println!("{:#?}", v);

    Ok(())
}
