//!send_temp.rs
// Reads the temperature sensor attached to the Pi
//
// The sensor on the Pi is a one-wire sensor. We have configured the Pi to continuously read
// this sensor, and place the output of that read in a text file on the system. This is all
// automatic, handled by standard Pi config done outside of our program.
//
// This application just reads the text file that contains the temperature sensor data, does a
// little computation to get it into the right format, pushes the read value to AWS, and saves it
// into our shared data space for later use.

use crate::shared_data::AccessSharedData;
use glob::glob;
use std::error::Error;
use std::path::Path;
use std::time::Duration;
use tokio::fs::File;
use tokio::io::{AsyncBufReadExt, BufReader};
use uuid::Uuid;

use crate::send_temp::store_temp_data;

// This is where the Pi stores temperature sensor data files
const BASE_DIR: &str = "/sys/bus/w1/devices/";

// This function is spawned on it's own thread by main. It finds the data file for our sensor,
// then sits in a loop reading that file, sleeping, reading again ==> until "get_continue_read_loop"
// gets set to false outside of our loop.
#[tracing::instrument(name = "running the loop to read temp setting",
    skip(sd),
    fields(temp_span = %Uuid::new_v4())
)]
pub async fn read_the_temperature(
    sd: &AccessSharedData,
    aws_url: String,
    poll_interval: usize,
) -> Result<(), Box<dyn Error>> {
    // Find our data file. Each sensor has an ID that is mapped to it's own data file.
    // This code only works for a single sensor attached to our pi.
    let mut device_file: String = "".to_string();
    for entry in glob(&format!("{}/28*", BASE_DIR)).unwrap() {
        match entry {
            Ok(path) => device_file = format!("{}/w1_slave", path.display()),
            Err(e) => return Err(Box::try_from(e).unwrap()),
        }
    }
    tracing::debug!("device file is: {}", device_file);

    // Now we loop "forever", reading the temperature, storing it, and sleeping
    loop {
        // if either of these fail, we return the error - which shuts down the program
        read_temp(&device_file, sd).await?;
        store_temp_data(sd, &aws_url).await?;

        // Check to see if someone externally has told us to shut down - typically a
        // signal handler allowing us to cleanly exit.
        if !(sd.get_continue_background_tasks()) {
            tracing::debug!("breaking loop");
            break;
        }

        tracing::debug!("going to sleep");
        tokio::time::sleep(Duration::from_secs(poll_interval as u64)).await;
    }
    Ok(())
}

// Here we implement the necessary protocol for interpreting the temperature probe's
// text file.
#[tracing::instrument(name = "reading the temp", skip(sd))]
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

// Utility function that just reads all of the lines in our file and returns them as a vector of
// strings for easier processing. We do all of this asynchronously, as reading the file might
// block while the system updates it in the background.
#[tracing::instrument(name = "reading the lines from the file", skip(filename))]
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
