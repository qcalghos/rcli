use csv::Reader;
use serde::{Serialize,Deserialize};
use anyhow::Result;
use std::fs;

#[derive(Debug,Serialize,Deserialize)]
#[serde(rename_all="PascalCase")]
struct Player{
    
    name:String,
    position:String,
    #[serde(rename="DOB")]
    dob:String,
    nationality:String,
    #[serde(rename="Kit Number")]
    kit:u8,

}
//convert csv to json
pub fn process_csv(input:&str,output:&str)->Result<()>{
    let mut reader=Reader::from_path(input)?;
            // let mut ret=Vec::with_capacity(128);
            let ret=reader
                .deserialize()
                .map(|record|record.unwrap())
                .collect::<Vec<Player>>();
            let json=serde_json::to_string_pretty(&ret)?;
            fs::write(output, json)?;
            Ok(())
}