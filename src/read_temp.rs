use crate::shared_data::AccessSharedData;
use glob::glob;
use std::error::Error;
use std::path::Path;
use std::time::Duration;
use tokio::fs::File;
use tokio::io::{AsyncBufReadExt, BufReader};
use uuid::Uuid;

use crate::send_temp::store_temp_data;

const BASE_DIR: &str = "/sys/bus/w1/devices/";
#[tracing::instrument(name = "running the loop to read temp setting",
    skip(sd),
    fields(temp_span = %Uuid::new_v4())
)]
pub async fn read_the_temperature(
    sd: &AccessSharedData,
    aws_url: String,
    poll_interval: usize,
) -> Result<(), Box<dyn Error>> {
    tracing::debug!("starting to read the temp");
    let mut device_file: String = "".to_string();
    for entry in glob(&format!("{}/28*", BASE_DIR)).unwrap() {
        match entry {
            Ok(path) => device_file = format!("{}/w1_slave", path.display()),
            Err(e) => return Err(Box::try_from(e).unwrap()),
        }
    }
    tracing::debug!("device file is: {}", device_file);

    loop {
        read_temp(&device_file, &sd).await?;
        store_temp_data(&sd, &aws_url).await?;

        tracing::debug!("Trying to get a lock");
        if !(sd.get_continue_read_temp()) {
            tracing::debug!("breaking loop");
            break;
        }

        tracing::debug!("going to sleep");
        //thread::sleep(Duration::from_millis(500));
        //thread::sleep(Duration::from_secs(15));
        tokio::time::sleep(Duration::from_secs(poll_interval as u64)).await;
    }
    Ok(())
}

async fn read_temp(device_file: &str, sd: &AccessSharedData) -> Result<(), std::io::Error> {
    let mut data = lines_from_file(device_file).await;
    tracing::debug!("Data read: {:?}", data);
    while &(data[0][data[0].len() - 3..]) != "YES" {
        tokio::time::sleep(Duration::from_millis(500)).await;
        data = lines_from_file(device_file).await;
    }
    let equals_pos = data[1].find("t=").unwrap();
    let temp_string = &data[1][equals_pos + 2..];
    let temp_c = temp_string.parse::<f32>().unwrap() / 1000.0;
    let temp_f = temp_c * 9.0 / 5.0 + 32.0;

    tracing::debug!("trying to set global current temp");
    sd.set_current_temp(temp_f);

    tracing::debug!("read temp is: {}", temp_f);

    Ok(())
}

async fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    tracing::debug!("reading temperature file: {:#?}", filename.as_ref());
    let file = File::open(filename).await.expect("no such file");
    let buf = BufReader::new(file);
    let mut lines = buf.lines();
    let mut rv: Vec<String> = Vec::new();
    while let Some(line) = lines.next_line().await.expect("Failed to read file") {
        rv.push(line);
    }
    rv
}
