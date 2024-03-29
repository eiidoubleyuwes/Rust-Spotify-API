//This simple project is for me to understand how the Spotify API works and how to use it.
//Check the cargo.toml file for dependencies like reqwest and all.
use reqwest::{self, Request};
use serde::{Serialize, Deserialize, Debug};
use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use std::env;

//Here we will define the structs for artist,album and track
#[derive(Serialize, Deserialize,Debug)]
struct Url{
    spotify: String,
}
#[derive(Serialize, Deserialize,Debug)]
struct msanii{
    name: String,
    msanii_url: Url,
}
#[derive(Serialize, Deserialize,Debug)]
struct album{
    name: String,
    //Using Vecrots to store multiple artists
    wasaniis: Vec<msanii>,
    albamu_url: Url,
}
#[derive(Serialize, Deserialize,Debug)]
struct track{
    name: String,
    href: String,
    albamu: album,
    track_url: Url,
}
struct APIresponse{
    tracks: Items<track>,
}
struct Items<T>{
    items: Vec<T>,
}
fn print_tracks(tracks: Vec<&track>){
    for track in tracks{
        println!("Track: {}",track.name);
        println!("Album: {}",track.albamu.name);
        println!("Artist: {}",track.albamu
        .wasaniis
        .iter()
        .map(|artist| artist.name.to_string())
        .collect::<String>()
        );
        println!("Spotify URL: {}",track.track_url.spotify);
        println!("\n");
    }
}
#[tokio::main]
async fn main(){
    let args: Vec<String> = env::args().collect();
    let tafuta = &args[1];
    let auth_token = &args[2];
    let link = format!(
        "https://api.spotify.com/v1/search?q={query}&type=track,artist",
        query = tafuta
    );
    let client = reqwest::Client::new();
    let jibu = client
    .get(&link)
    .header(AUTHORIZATION, format!("Bearer {}", auth_token))
    .header(CONTENT_TYPE, "application/json")
    .header(ACCEPT, "application/json")
    .send()
    .await
    .unwrap();
match jibu.Status(){
    reqwest::StatusCode::OK => {
        match jibu.json::<APIresponse>().await{
            Ok(parsed) => print_tracks(parsed.tracks.items.iter().collect()),
            Err(e) => {
                println!("Error: {}",e);
            }
        };
    }
    reqwest::StatusCode::UNAUTHORIZED => {
        println!("Bad token");
    }
    other => {
        panic!("Unexpected status code: {:?}", other);
    }
}
}