#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use rss::Channel;
use std::error::Error;
const LINK: &str = "https://www.phoronix.com/phoronix-rss.php";

#[derive(Debug, Clone)]
struct Feed {
    titles: Vec<String>,
    dates: Vec<String>,
    links: Vec<String>,
}

impl Default for Feed {
    fn default() -> Self {
        Feed {
            titles: vec![],
            dates: vec![],
            links: vec![],
        }
    }
}
async fn get_feed() -> Result<Channel, Box<dyn Error>> {
    let content = reqwest::get(LINK).await?.bytes().await?;
    let channel = Channel::read_from(&content[..])?;
    Ok(channel)
}
impl Feed {
    pub async fn new() -> Result<Self, Box<dyn Error>> {
        let channel = get_feed().await?;
        let mut titles = Vec::new();
        let mut dates = Vec::new();
        let mut links = Vec::new();

        let items = channel.items();
        for i in 0..25 {
            titles.push(items[i].title.clone().unwrap());
            dates.push(items[i].pub_date.clone().unwrap());
            links.push(items[i].link.clone().unwrap())
        }

        Ok(Self {
            titles,
            dates,
            links,
        })
    }
    pub async fn load() -> Self {
        match Self::new().await {
            Ok(f) => f,
            Err(_) => Self::default(),
        }
    }
}

#[tauri::command]
async fn get_titles() -> Vec<String> {
    Feed::load().await.titles.clone()
}

#[tauri::command]
async fn get_dates() -> Vec<String> {
    Feed::load().await.dates.clone()
}
#[tauri::command]
async fn get_links() -> Vec<String> {
    Feed::load().await.links.clone()
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_titles, get_dates, get_links])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
