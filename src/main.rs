use std::{thread, time};

pub mod messaging;
pub mod channels;
use failure::{Fallible};

use channels::channel_author;
use channels::channel_subscriber;

#[tokio::main]
async fn main() -> Fallible<()> {

    let seed_author = "SOME9AUTHOR9SEED9HERE9NOW9K";
    let seed_subscriber = "SOME9SUBSCRIBER9SEED";

    let node = "https://nodes.devnet.iota.org:443";



    //Create Channel Instance for writer
    let mut channel_author = channel_author::Channel::new(seed_author, node);

    //Open Channel
    let (channel_address, announcement_tag) = channel_author.open().await.unwrap();
    println!("channel_address: {}", channel_address);
    println!("announcement_tag: {}", announcement_tag);



    //Give messages some time to propagate
    thread::sleep(time::Duration::from_millis(20000));



    //Create Channel Instance for reader
    let mut channel_subscriber = channel_subscriber::Channel::new(seed_subscriber, node, channel_address, announcement_tag);

    //Connect to channel
    let subscription_tag = channel_subscriber.connect().await.unwrap();



    //Give messages some time to propagate
    thread::sleep(time::Duration::from_millis(20000));



    let keyload_tag = channel_author.add_subscriber(subscription_tag).await.unwrap();

    //Send Messages
   channel_author.write_signed("PAYLOAD9SIGNED9PUBLIC", "PAYLOAD9SIGNED9MASKED").await.unwrap();
   channel_author.write_tagged("PAYLOAD9TAGGED9PUBLIC", "PAYLOAD9TAGGED9MASKED").await.unwrap();


 
    //Give messages some time to propagate
    thread::sleep(time::Duration::from_millis(20000));



    channel_subscriber.update_keyload(keyload_tag).await.unwrap();

    //Read all signed messages
    for msg in channel_subscriber.read_signed().await.unwrap(){
        let (public, masked) = msg;
        println!("Signed -> Public: {} -- Masked: {}", public, masked)
    }

    //Read all tagged messages
    for msg in channel_subscriber.read_tagged().await.unwrap(){
        let (public, masked) = msg;
        println!("Tagged -> Public: {} -- Masked: {}", public, masked)
    }



    //Disconnect from channel
    let unsubscribe_tag = channel_subscriber.disconnect().await.unwrap();



    //Give messages some time to propagate
    thread::sleep(time::Duration::from_millis(20000));



    channel_author.remove_subscriber(unsubscribe_tag).await.unwrap();


    
    Ok(())
}
