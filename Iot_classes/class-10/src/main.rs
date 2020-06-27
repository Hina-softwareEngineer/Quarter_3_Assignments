use bson::doc;
use mongodb::{options::ClientOptions, Client};
use serde_json::{json, value};


fn main(){
    let mut client_options = ClientOptions::parse("mongodb://localhost:27017").await?;

    let db = client.database("mydb");

    let collection = db.collection("books");

    let docs = vec![
    doc! { "title": "1984", "author": "George Orwell" },
    doc! { "title": "Animal Farm", "author": "George Orwell" },
    doc! { "title": "The Great Gatsby", "author": "F. Scott Fitzgerald" },
];

// Insert some documents into the "mydb.books" collection.
collection.insert_many(docs, None).await?;

// let data=db.insert_one(docs, None).unwrap();
// match data{
    /*
    Some(data)=>
    {
        let data: Value=json!(data);
        println!("data {}", data);
    }
    None=>println!("no record")
    */
}



// updating a doc
    /*
    let filter=doc!{"username":"JOins"};
    let replacement=doc!{"username":"umer123"};
    db.find_one_and_replace(filter, replacement, None);
    */


    let docs1=doc!{"username":"user123"};
    let data=db.find_one(docs1, None).unwrap();


    // deleting the document

    let query=doc!{"username":"jondlf"};
    db.delete_one(query, None);

    


}