#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use iota_lib_rs::prelude::iota_client;
use iota_streams::app_channels::{
    api::tangle::{ Address, Author, DefaultTW, Message}
    , message
};
use iota_streams::app::transport::Transport;
use iota_streams::app::transport::tangle::client::SendTrytesOptions;
use iota_streams::protobuf3::types::Trytes;
use iota_streams::core::tbits::Tbits;
use std::string::ToString;
use std::str::FromStr;
use failure::Fallible;


pub struct Channel{
    author: Author,
    client: iota_client::Client<'static>,
    send_opt: SendTrytesOptions,
    channel_address:String,
    announcement_link: Address,
    keyload_link: Address,
}

impl Channel {

    pub fn new(seed: &str, node_ulr: &'static str) -> Channel{

        let author = Author::new(seed, 3, true);

        let channel_address = author.channel_address().to_string();

        let mut send_opt = SendTrytesOptions::default();
        send_opt.min_weight_magnitude = 9;
        send_opt.local_pow = false;

        Self {
            author:author,
            client: iota_client::Client::new(node_ulr),
            send_opt: send_opt,
            channel_address:channel_address,
            announcement_link: Address::default(),
            keyload_link: Address::default(),
        }
    }

    pub fn open(&mut self)-> Result<(String, String), &str>{

        let announcement_message = self.author.announce().unwrap();
        self.client.send_message_with_options(&announcement_message, self.send_opt).unwrap();
        let announcement_address: String = announcement_message.link.appinst.to_string();
        let announcement_tag: String = announcement_message.link.msgid.to_string();

        self.announcement_link = Address::from_str(&announcement_address, &announcement_tag).unwrap();

        Ok((self.channel_address.clone(), announcement_tag))
    }

    pub fn add_subscriber(&mut self, subscribe_tag: String) -> Result<String,&str>{

        let subscribe_link = Address::from_str(&self.channel_address, &subscribe_tag).unwrap();

        let message_list = self.client.recv_messages_with_options(&subscribe_link,()).unwrap();
    
        for tx in message_list.iter() {
            match tx.parse_header(){
                Ok(header) => {
                    if header.check_content_type(message::subscribe::TYPE) {
                        match self.author.unwrap_subscribe(header.clone()) {
                            Ok(_) => {
                                break;
                            },
                            Err(e) => println!("Subscribe Packet Error: {}", e),
                        }
                    }else{
                        println!("Expected a subscription message, found {}", header.content_type());
                    }
                }
                Err(e) => {
                    println!("Parsing Error Header: {}", e)
                }
            }
        }

        self.keyload_link = {
            let msg = self.author.share_keyload_for_everyone(&self.announcement_link).unwrap();
            self.client.send_message_with_options(&msg, self.send_opt).unwrap();
            msg.link
        };

        Ok(self.keyload_link.msgid.to_string())

    }

    pub fn write_signed(&mut self, masked:bool, public_payload: &str, private_payload: &str)-> Result<String, &str>{

        let public_payload = Trytes(Tbits::from_str(&public_payload).unwrap());
        let private_payload = Trytes(Tbits::from_str(&private_payload).unwrap());

        let signed_packet_link  = {

            if masked{
                let msg = self.author.sign_packet(&self.keyload_link, &public_payload, &private_payload).unwrap();
                self.client.send_message_with_options(&msg, self.send_opt).unwrap();
                msg.link.clone()
            }else{
                let msg = self.author.sign_packet(&self.announcement_link, &public_payload, &private_payload).unwrap();
                self.client.send_message_with_options(&msg, self.send_opt).unwrap();
                msg.link.clone()
            }
        };

        Ok(signed_packet_link.msgid.to_string())

    }

    pub fn write_tagged(&mut self, public_payload: &str, private_payload: &str)-> Result<String, &str>{

        let public_payload = Trytes(Tbits::from_str(&public_payload).unwrap());
        let private_payload = Trytes(Tbits::from_str(&private_payload).unwrap());

        let tagged_packet_link = {
            let msg = self.author.tag_packet(&self.keyload_link, &public_payload, &private_payload).unwrap();
            self.client.send_message_with_options(&msg, self.send_opt).unwrap();
            msg.link.clone()
        };

        Ok(tagged_packet_link.msgid.to_string())

    }

    pub fn remove_subscriber(&mut self, unsubscribe_tag: String) -> Fallible<()>{

        let unsubscribe_link = Address::from_str(&self.channel_address, &unsubscribe_tag).unwrap();

        let message_list = self.client.recv_messages_with_options( &unsubscribe_link,()).unwrap();
    
        for tx in message_list.iter() {
            match tx.parse_header(){
                Ok(header) => {
                    if header.check_content_type(message::unsubscribe::TYPE) {
                        match self.author.unwrap_unsubscribe(header.clone()) {
                            Ok(_) => {
                                break;
                            },
                            Err(e) => println!("Unsubscribe Packet Error: {}", e),
                        }
                    }else{
                        println!("Expected a unsubscription message, found {}", header.content_type());
                    }
                }
                Err(e) => {
                    println!("Parsing Error Header: {}", e);
                }
            };
        }
        Ok(())
    }

}