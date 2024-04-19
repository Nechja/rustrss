use reqwest;
use rss::Channel;
use std::error::Error;
use colored::*;

fn main() -> Result<(), Box<dyn Error>> {
    let url = "http://feeds.arstechnica.com/arstechnica/index";
    let content = reqwest::blocking::get(url)?.bytes()?;
    let channel = Channel::read_from(&content[..])?;

    for item in channel.items() {
        let title = item.title().unwrap_or("No title").green();
        println!("Title: {}", title);
        if let Some(link) = item.link() {
            let link = link.blue().underline();
            println!("Link: {}", link);
        }
        if let Some(description) = item.description() {
            println!("Description: {}", description.bright_cyan());
        }
        println!(); 
    }

    Ok(())

}
