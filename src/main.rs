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
    albamu_url: Url,
}
#[derive(Serialize, Deserialize,Debug)]
struct track{
    name: String,
    track_url: Url,
}