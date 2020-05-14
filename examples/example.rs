use std::{thread, time};

extern crate channels_lite;
use failure::{Fallible};

use channels_lite::channels::channel_author;
use channels_lite::channels::channel_subscriber;

#[tokio::main]
async fn main()-> Fallible<()>{

    let seed_author = "SOME9AUTHOR9SEED9SECRTE";
    let seed_subscriber = "SOME9SUBSCRIBER9SEED";

    let node:  &'static str = "https://nodes.devnet.iota.org:443";

    let delay_time: u64 = 30;



    //Create Channel Instance for author
    let mut channel_author = channel_author::Channel::new(seed_author, node);

    //Open Channel
    let (channel_address, announcement_tag) = channel_author.open().unwrap();
    println!("Author: Announced channel");
    println!("channel_address: {}", channel_address);
    println!("announcement_tag: {}", announcement_tag);



    //Give messages some time to propagate
    println!("Waiting for propagation... ({}s)", delay_time);
    thread::sleep(time::Duration::from_secs(delay_time));



    //Create Channel Instance for subscriber
    let mut channel_subscriber = channel_subscriber::Channel::new(seed_subscriber, node, channel_address, announcement_tag);
    

    //Connect to channel
    let subscription_tag = channel_subscriber.connect().unwrap();
    println!("Subscriber: Connected to channel");



    //Give messages some time to propagate
    println!("Waiting for propagation... ({}s)",delay_time);
    thread::sleep(time::Duration::from_secs(delay_time));



    let keyload_tag = channel_author.add_subscriber(subscription_tag).unwrap();

    //Send Messages
    let signed_packed_tag:String = channel_author.write_signed("PAYLOAD9SIGNED9PUBLIC", "PAYLOAD9SIGNED9MASKED").unwrap();
    println!("Author: Sent signed message");
    let tagged_packed_tag:String = channel_author.write_tagged("PAYLOAD9TAGGED9PUBLIC", "PAYLOAD9TAGGED9MASKED").unwrap();
    println!("Author: Sent tagged message");

 
    //Give messages some time to propagate
    println!("Waiting for propagation... ({}s)", delay_time);
    thread::sleep(time::Duration::from_secs(delay_time));



    channel_subscriber.update_keyload(keyload_tag).unwrap();
    println!("Subscriber: Updated keyload");

    //Read all signed messages
    let list_signed = channel_subscriber.read_signed(signed_packed_tag).unwrap();
    println!("Subscriber: Reading signed messgaes");
    for msg in list_signed.iter(){
        let (public, masked) = msg;
        println!("Subscriber: Found Signed Message -> Public: {} -- Masked: {}", public, masked)
    }

    //Read all tagged messages
    let list_tagged = channel_subscriber.read_tagged(tagged_packed_tag).unwrap();
    println!("Subscriber: Reading tagged messgaes");
    for msg in list_tagged.iter(){
        let (public, masked) = msg;
        println!("Subscriber: Found Tagged Message -> Public: {} -- Masked: {}", public, masked)
    }



    //Disconnect from channel
    let unsubscribe_tag = channel_subscriber.disconnect().unwrap();
    println!("Subscriber: Disconnected from channel");



    //Give messages some time to propagate
    println!("Waiting for propagation... ({}s)", delay_time);
    thread::sleep(time::Duration::from_secs(delay_time));



    channel_author.remove_subscriber(unsubscribe_tag).unwrap();
    println!("Author: Removed subscriber");



    Ok(())
}
