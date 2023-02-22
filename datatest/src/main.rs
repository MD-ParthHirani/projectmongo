use mongodb::{Client, options::{ClientOptions, FindOptions, UpdateOptions}};
use bson::{doc, oid::ObjectId};
use serde::{Serialize, Deserialize};
use std::io;


#[derive(Debug, Serialize, Deserialize)]
struct User {
    #[serde(rename = "_id")]
    id: ObjectId,
    company: String,
    city: String,
    projectno: i32,
    address:String
}


#[tokio::main]
async fn main() {
    let client_options = ClientOptions::parse("mongodb://localhost:27017").await.unwrap();
    let client = Client::with_options(client_options).unwrap();
    let db = client.database("companydatabase");
    let collection = db.collection("users");

   

    println!("\n- company name :");
    let mut company = String::new();
    io::stdin().read_line(&mut company).unwrap();
    let company = company.trim().to_string();

    println!("\n- project no:");
    let mut projectno = String::new();
    io::stdin().read_line(&mut projectno).unwrap();
    let projectno = projectno.trim().to_string();

    
    println!("\n- address:");
    let mut address = String::new();
    io::stdin().read_line(&mut address).unwrap();
    let address = address.trim().to_string();

    println!("\n- city:");
    let mut city = String::new();
    io::stdin().read_line(&mut city).unwrap();
    let city = city.trim().to_string();

    let data = User {
        id: ObjectId::new(),
        company: company.to_string(),
        city: city.to_string(),
        projectno: projectno.parse().unwrap(),
        address: address.to_string(),

    };

    collection.insert_one(data, None).await.unwrap();

}
