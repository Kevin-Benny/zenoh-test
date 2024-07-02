use clap::Parser;
use sabaton_mw::Topic;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::time::Duration;
use zenoh::config::Config;
use zenoh::prelude::r#async::*;
use zenoh_ext_examples::CommonArgs;
use tokio::time::sleep;
use dc_meter_service;
#[tokio::main]
async fn main() {
    let builder = sabaton_mw::NodeBuilder::default();
    let node = builder.build("test".to_owned(), sabaton_mw::config::MWConfig::default()).unwrap();

    let reader = node.subscribe::<dc_meter_service::meter_types::InstantaneousMeterReading>("01",  |s|{
        println!("Received: {:?}", s);
    });
    
    sleep(std::time::Duration::from_secs(10)).await;
}
