use std::{fs::File, error::Error, io::{Read, Write}};

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn get_input(url: &str) -> Result<String, reqwest::Error>{
    use reqwest::{cookie::Jar, Url};

    let url = url.parse::<Url>().expect("Error a la url");
    let cookie = include_str!("../../.env");

    let jar = Jar::default();
    jar.add_cookie_str(&cookie, &url);
        
    let client = reqwest::blocking::Client::builder()
        .cookie_provider(std::sync::Arc::new(jar))
        .build()?;

    let resposta = client
        .get(url)
        .send()?
        .text()?;

    Ok(resposta)
}

pub fn load_input(part_num: u8, day: u8) -> Result<String, Box<dyn Error>> {
    let filename = format!("input{part_num}.txt");
    let file = File::open(&filename);
    if let Ok(mut text) = file {
        println!("Using cached input...");
        let mut contents = String::new();
        text.read_to_string(&mut contents)?;
        Ok(contents)
    } else {
        println!("Input not downloading, getting...");
        let text = get_input(&format!("https://adventofcode.com/2023/day/{day}/input"))?;
        let mut file = File::create(&filename)?;
        file.write_all(text.as_bytes())?;
        Ok(text)
    }
}
