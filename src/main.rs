// Author <Krooxk>

use std::io::{self, Write};
use html2text::from_read;
use colored::*;
use regex::Regex;
use reqwest;

fn html_to_text(html: &str) -> String {
    from_read(html.as_bytes(), 80).trim().to_string()
}

fn remove_uuids(text: &str) -> String {
    let re_uuid = Regex::new(r"\[\w{8}-\w{4}-\w{4}-\w{4}-\w{12}\]\[\d+\]\*").unwrap();
    let re_by_id = Regex::new(r"\[\d+\]:\s*/documents/by_id/\w{8}-\w{4}-\w{4}-\w{4}-\w{12}").unwrap();
    let re_empty_lines = Regex::new(r"(?m)^\s*$\n?").unwrap();

    re_empty_lines.replace_all(&re_by_id.replace_all(&re_uuid.replace_all(text, ""), "").replace("*", ""), "").to_string()
}

fn req(url: &str) -> Result<(), reqwest::Error> {
    let body = reqwest::blocking::get(url)?.text()?;
    let out = html_to_text(&body);
    println!("{}", remove_uuids(&out));
    Ok(())
}

fn request(choice: &str) {
    let mut data = String::new();
    
    print!(" {} {} ", "[*]".yellow().bold(), "Data -> ".bold());
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut data).expect("Failed to read line");

    let url = format!("https://search.illicit.services/records?{}={}", choice, data.trim());

    req(&url);
}

fn main() {
    println!("\n {}{} FirstName.", "1".bold(), ")".yellow().bold());
    println!(" {}{} LastName.", "2".bold(), ")".yellow().bold());
    println!(" {}{} Email.", "3".bold(), ")".yellow().bold());
    println!(" {}{} Username.", "4".bold(), ")".yellow().bold());
    println!(" {}{} Phone.", "5".bold(), ")".yellow().bold());
    println!(" {}{} Address.", "6".bold(), ")".yellow().bold());
    println!(" {}{} License Plate Number.", "7".bold(), ")".yellow().bold());
    println!(" {}{} VIN.", "8".bold(), ")".yellow().bold());
    println!(" {}{} City.", "9".bold(), ")".yellow().bold());
    println!(" {}{} State.", "10".bold(), ")".yellow().bold());
    println!(" {}{} Zip.\n", "11".bold(), ")".yellow().bold());

    print!(" {} {} ", "[*]".yellow().bold(), "~$".bold());
    io::stdout().flush().unwrap();

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read line");
    let choice = choice.trim();

    match choice {
        "1" => request("firstName"),
        "2" => request("lastName"),
        "3" => request("emails"),
        "4" => request("usernames"),
        "5" => request("phoneNumbers"),
        "6" => request("address"),
        "7" => request("VRN"),
        "8" => request("vin"),
        "9" => request("city"),
        "10" => request("state"),
        "11" => request("zipCode"),
        _ => println!("\n {}", "[!] Not Found".red().bold()),
    }
}


