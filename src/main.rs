//This simple project is for me to understand how the Spotify API works and how to use it.
//Check the cargo.toml file for dependencies like reqwest and all.
use reqwest::{self};
use serde::{Serialize, Deserialize};
use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use std::env;

//Here we will define the structs for artist,Album and Track
#[derive(Serialize, Deserialize,Debug)]
struct Url{
    spotify: String,
}
#[derive(Serialize, Deserialize,Debug)]
struct Msanii{
    name: String,
    msanii_url: Url,
}
#[derive(Serialize, Deserialize,Debug)]
struct Album{
    name: String,
    //Using Vecrots to store multiple artists
    wasaniis: Vec<Msanii>,
    albamu_url: Url,
}
#[derive(Serialize, Deserialize,Debug)]
struct Track{
    name: String,
    href: String,
    albamu: Album,
    track_url: Url,
}
#[derive(Deserialize,Debug)]
struct APIresponse{
    tracks: Items<Track>,
}
#[derive(Debug)]
struct Items<T>{
    items: Vec<T>,
}
// Implement Deserialize for Items<T> where T is also Deserialize
impl<'de, T> Deserialize<'de> for Items<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(Items {
            items: Vec::deserialize(_deserializer)?,
        })
    }
}

fn print_tracks(tracks: Vec<&Track>){
    for Track in tracks{
        println!("Track: {}",Track.name);
        println!("Album: {}",Track.albamu.name);
        println!("Artist: {}",Track.albamu
        .wasaniis
        .iter()
        .map(|msanii| msanii.name.to_string())
        .collect::<String>()
        );
        println!("Spotify URL: {}",Track.track_url.spotify);
        println!("\n");
    }
}
#[tokio::main]
async fn main(){
    let args: Vec<String> = env::args().collect();
    let tafuta = &args[1];
    let auth_token = &args[2];
    let link = format!(
        "https://api.spotify.com/v1/search?q={query}&type=Track,artist",
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
match jibu.status(){
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
