use std::{thread, time};

pub mod messaging;
pub mod channels;
use failure::{Fallible};

use channels::channel_writer;
use channels::channel_reader;

#[tokio::main]
async fn main() -> Fallible<()> {

    let seed = "SOME9AUTHOR9SEED9HERE9NOW";
    let node = "https://nodes.devnet.iota.org:443";

    //Create Channel Instance
    let mut channel_writer = channel_writer::Channel::new(seed, node);
    
    //Open Channel
    let (channel_address, announcement_tag) = channel_writer.open().await.unwrap();
    println!("channel_address: {}",channel_address);
    println!("announcement_tag: {}",announcement_tag);

    //Send Messages
    channel_writer.write_signed("PAYLOAD9SIGNED9PUBLIC", "PAYLOAD9SIGNED9MASKED").await.unwrap();
    channel_writer.write_tagged("PAYLOAD9TAGGED9PUBLIC", "PAYLOAD9TAGGED9MASKED").await.unwrap();
 
    //Give messages some time to propagate
    thread::sleep(time::Duration::from_millis(20000));


    let seed = "SUB9SEED";
    
    //Create Channel Instance fro reader
    let mut channel_reader = channel_reader::Channel::new(seed, node, channel_address, announcement_tag);

    //Connect to channel
    channel_reader.connect().await.unwrap();

    //Read all signed messages
    for msg in channel_reader.read_signed().await.unwrap(){
        let (public, masked) = msg;
        println!("Public: {} -- Masked: {}", public, masked)
    }

    
    
    /* Gives error Link not found

    for msg in channel_reader.read_tagged().await.unwrap(){
        let (public, masked) = msg;
        println!("Public: {} -- Masked: {}", public, masked)
    }
    //Disconnect from channel
    let unsubscribe_tag = channel_reader.disconnect().await.unwrap();

    //Give messages some time to propagate
    thread::sleep(time::Duration::from_millis(20000));

    channel_writer.read_disconnect(unsubscribe_tag).await.unwrap();

    */
    
    Ok(())

}
