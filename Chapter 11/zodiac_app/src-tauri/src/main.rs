#![cfg_attr(all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod zodiac;
use zodiac::Zodiac;
#[tauri::command]
fn get_zodiac(zodiac: &str) -> Vec<String>{
    let zodiac = zodiac.to_lowercase();
    match zodiac.as_str(){
       "aries" => Zodiac::aries().to_string(),
       "taurus" => Zodiac::taurus().to_string(),
       "gemini" => Zodiac::gemini().to_string(),
       "cancer" => Zodiac::cancer().to_string(),
       "leo" => Zodiac::leo().to_string(),
       "virgo" => Zodiac::virgo().to_string(),
       "libra" => Zodiac::libra().to_string(),
       "scorpius" => Zodiac::scorpius().to_string(),
       "sagittarius" => Zodiac::sagittarius().to_string(),
       "capricorn" => Zodiac::capricorn().to_string(),
       "aquarius" => Zodiac::aquarius().to_string(),
       "pisces" => Zodiac::pisces().to_string(),
       _ => Zodiac::default().to_string()
   }
}
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_zodiac])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
