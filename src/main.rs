//This simple project is for me to understand how the Spotify API works and how to use it.
//Check the cargo.toml file for dependencies like reqwest and all.
use reqwest;
use serde::{Serialize, Deserialize};
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
fn print_tracks(tracks: vec<&track>){
    for track in tracks{
        println!("Track: {}",track.name);
        println!("Album: {}",track.albamu.name);
        println!("Artist: {}",track.albamu.wasaniis[0].name);
        println!("Spotify URL: {}",track.track_url.spotify);
        println!("\n");
    }
}
async fn main(){
    
}