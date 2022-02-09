use crate::shared_data::AccessSharedData;
use glob::glob;
use log::{debug, error, info};
use std::error::Error;
use std::time::Duration;
use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

use crate::send_temp::store_temp_data;

const BASE_DIR: &str = "/sys/bus/w1/devices/";

pub async fn read_the_temperature(
    sd: &AccessSharedData,
    aws_url: String,
) -> Result<(), Box<dyn Error>> {
    debug!("starting to read the temp");
    let mut device_file: String = "".to_string();
    for entry in glob(&format!("{}/28*", BASE_DIR)).unwrap() {
        match entry {
            Ok(path) => device_file = format!("{}/w1_slave", path.display()),
            Err(e) => return Err(Box::try_from(e).unwrap()),
        }
    }
    debug!("device file is: {}", device_file);

    loop {
        read_temp(&device_file, &sd).await?;
        store_temp_data(&sd, &aws_url).await?;

        debug!("Trying to get a lock");
        if !(sd.get_continue_read_temp()) {
            debug!("breaking loop");
            break;
        }

        debug!("going to sleep");
        //thread::sleep(Duration::from_millis(500));
        //thread::sleep(Duration::from_secs(15));
        tokio::time::sleep(Duration::from_secs(15)).await;
    }
    Ok(())
}

async fn read_temp(device_file: &str, sd: &AccessSharedData) -> Result<(), std::io::Error> {
    let mut data = lines_from_file(device_file).await;
    debug!("Data read: {:?}", data);
    while &(data[0][data[0].len() - 3..]) != "YES" {
        tokio::time::sleep(Duration::from_millis(500)).await;
        data = lines_from_file(device_file).await;
    }
    let equals_pos = data[1].find("t=").unwrap();
    let temp_string = &data[1][equals_pos + 2..];
    let temp_c = temp_string.parse::<f32>().unwrap() / 1000.0;
    let temp_f = temp_c * 9.0 / 5.0 + 32.0;

    debug!("trying to set global current temp");
    sd.set_current_temp(temp_f);

    println!("temp is: {}", temp_f);

    Ok(())
}

async fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}
