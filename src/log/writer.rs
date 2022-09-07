use std::fs::{File};
use std::io::prelude::Write;

pub fn write(file_name: &str, message: &str, tag: Option<&str>) -> std::io::Result<()> {
    let mut file = File::options()
        .write(true)
        .append(true)
        .create(true)
        .open(format!("{}.txt", file_name))?;
        

    let msg = format!("TIME: {}, TAG: {}, MSG: {} \n", chrono::Utc::now()
        .format("%d-%m-%Y %H:%M:%S")
        .to_string(), tag.unwrap_or("default"), message);
    
    file.write(msg.as_bytes())?;
    
    Ok(())
}