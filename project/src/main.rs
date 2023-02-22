use mongodb::{bson::{doc, oid::ObjectId},Client,  options::{ClientOptions,FindOptions, UpdateOptions}, Collection};
use tokio;
use serde::{Deserialize,Serialize};
use async_std::stream::StreamExt;

use std::io;


#[derive(Debug,Serialize,Deserialize)]
pub struct User {
        #[serde(rename = "_id")]
        
    id: ObjectId,
    name:String,
    fname:String,
    lname:String,
    address: String,
    city: String,
    code: i32,
}
   
 
 
#[tokio::main]
async fn main(){
    
    
    let client = mongo_connect().await;
   let col= get_collection(client);
   let mut read_choice = true;
   while read_choice{
   let mut choice = String::new();
   println!("1: employee data insert \n2: employee data find \n3: employee data update \n4: employee data delete \n5: Exit ");
   println!("enter your choice:");
   std::io::stdin().read_line(&mut choice).unwrap();
   choice = choice.trim().to_string();
   match choice.as_str(){

    "1"=> {
        add_employee(col.clone()).await;
    },
    "2"=> {
        find_employee(col.clone()).await;
    },
    "3"=> {
        update_employee(col.clone()).await;
    },
    "4"=>{
        delete_employee(col.clone()).await;
    },
    "5"=>{
        println!("exited");
        read_choice=false;
    },
    _ => {println!("enter valid choice")}
   }}
   
}
async fn add_employee(collection:Collection<User>){
    println!("\n enter company name");
    let mut input_name = String::new();
    io::stdin().read_line(&mut input_name).unwrap();
    let name = input_name.trim().to_string();

    println!("\nEnter first  name:");
    let mut fname = String::new();
    io::stdin().read_line(&mut fname).unwrap();
    let fname = fname.trim().to_string();

    println!("\nEnter last  name:");
    let mut lname = String::new();
    io::stdin().read_line(&mut lname).unwrap();
    let lname = lname.trim().to_string();

    println!("\nEnter address:");
    let mut address = String::new();
    io::stdin().read_line(&mut address).unwrap();
    let address = address.trim().to_string();

    println!("\nEnter city  name:");
    let mut city = String::new();
    io::stdin().read_line(&mut city).unwrap();
    let city = city.trim().to_string();

    println!("\nEnter your code:");
    let mut code = String::new();
    io::stdin().read_line(&mut code).unwrap();
    let code = code.trim().to_string();

    let user = User {
        id: ObjectId::new(),
        name: name.to_string(),
        fname:fname.to_string(),
        lname:lname.to_string(),
        address:address.to_string(),
        city:city.to_string(),
        code: code.parse().unwrap(),
    };

    collection.insert_one(user, None).await.unwrap();
    println!("\nHello, {}!!", name);
                    
}


async fn update_employee(collection:Collection<User>){

   
    println!("\n Enter company name which you want to update:");
    let mut input_name = String::new();
    io::stdin().read_line(&mut input_name).unwrap();
    let name = input_name.trim().to_string();
    let filter = doc! { "name": &name };

    println!("\nEnter new company name:");
    let mut new_input_name = String::new();
    io::stdin().read_line(&mut new_input_name).unwrap();
    let new_name = new_input_name.trim().to_string();

    println!("\nEnter new first name:");
    let mut input_fname = String::new();
    io::stdin().read_line(&mut input_fname).unwrap();
    let fname = input_fname.trim().to_string();

    println!("\nEnter new last name:");
    let mut input_lname = String::new();
    io::stdin().read_line(&mut input_lname).unwrap();
    let lname = input_lname.trim().to_string();

    println!("\nEnter new address:");
    let mut input_address = String::new();
    io::stdin().read_line(&mut input_address).unwrap();
    let address = input_address.trim().to_string();

    println!("\nEnter new city name:");
    let mut input_city = String::new();
    io::stdin().read_line(&mut input_city).unwrap();
    let city = input_city.trim().to_string();

    println!("\nEnter new code:");
    let mut code = String::new();
    io::stdin().read_line(&mut code).unwrap();
    let code:i32 = code.trim().parse().unwrap();

    let update = doc! { "$set": { "company": new_name, "first name":fname,"last name":lname , "address":address ,"city":city,"code":code} };
    let update_options = UpdateOptions::builder().build();

    collection.update_one(filter, update, update_options).await.unwrap();


    let filter = doc! { "name": &name };
    let find_options = FindOptions::builder().build();
    let mut cursor = collection.find(filter, find_options).await.unwrap();


    cursor.next().await.into_iter().for_each(|result| {
    match result {
    Ok(document) => {
        println!("3 {:?}",document);
    },
        Err(error) => println!("Error: {:?}", error),
        }
    });
    
}
async fn find_employee(collection:Collection<User>){
    println!("\n enter company name which you want to fine:");
    
    let mut input_name = String::new();
    io::stdin().read_line(&mut input_name).unwrap();
    let name = input_name.trim().to_string();

    let filter = doc! { "name": name };
    let find_options = FindOptions::builder().build();
    let mut cursor = collection.find(filter, find_options).await.unwrap();

    cursor.next().await.into_iter().for_each(|result| {
        match result {
            Ok(document) => {
                    println!("\ncompany: {:?}\nfirst name: {:?}\nlast name: {:?}\naddress: {:?}\ncity: {:?}\ncode: {:?}",document.name,document.fname,document.lname,document.address,document.city,document.code);
                },
                    Err(e) => println!("Error: {:?}", e),
                }
    });
   
}
async fn delete_employee(collection:Collection<User>){

    println!("Enter company name:");
    let mut input_name = String::new();
    io::stdin().read_line(&mut input_name).unwrap();
    let name = input_name.trim().to_string();

    let filter = doc! { "name": name };
    collection.delete_one(filter, None).await.unwrap();
    println!("\n data is deleted");
    
   
}

pub async fn mongo_connect()->Client{

    let client_options = ClientOptions::parse("mongodb://localhost:27017").await.unwrap();
    let client = Client::with_options(client_options).unwrap();
    client
}

pub fn get_collection(client:Client)->Collection<User>{

    let db = client.database("companydatabase");
    let collection_list = db.collection::<User>("company");
    collection_list
}