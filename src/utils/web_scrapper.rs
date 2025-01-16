use std::{error::Error, thread::sleep, time::Duration};

use rand::Rng;
use reqwest::blocking::Client;
use scraper::Selector;

pub trait LostArkCodexWebScrapper {
    fn get_zone_name(&self, id: u32) -> Result<Option<String>, Box<dyn Error>>;
}

pub struct DefaultLostArkCodexWebScrapper<'a> {
    url: &'a str
}

impl<'a> LostArkCodexWebScrapper for DefaultLostArkCodexWebScrapper<'a> {
    
    fn get_zone_name(&self, id: u32) -> Result<Option<String>, Box<dyn Error>> {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(10))
            .build()?;

        let response = client
            .get(format!("{}/us/zone/{}/", self.url, id))
            .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/114.0.0.0 Safari/537.36")
            .header("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8")
            .header("Accept-Language", "en-US,en;q=0.9")
            .header("Connection", "keep-alive")
            .send()?;

        let html_content = response.text()?;
        
        let document = scraper::Html::parse_document(&html_content);
        let selector = Selector::parse("nav > ol > li:nth-child(3)")?;

        if let Some(element) = document.select(&selector).next() {
            let zone_name = element.text().collect::<Vec<_>>().join(" ");

            if zone_name.is_empty() {
                return Ok(None);    
            }

            return Ok(Some(zone_name));
        }

        self.random_delay();

        Ok(None)
    }
}

impl<'a> DefaultLostArkCodexWebScrapper<'a> {
    pub fn new() -> Self {
        
        Self {
            url: "https://lostarkcodex.com"
        }
    }

    fn random_delay(&self) {
        let mut rng = rand::thread_rng();
        let delay = rng.gen_range(1000..2000);
    
        sleep(Duration::from_millis(delay));
    }
}