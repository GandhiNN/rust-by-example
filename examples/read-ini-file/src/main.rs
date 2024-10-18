use configparser::ini::Ini;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut config = Ini::new();

    // Load file from configuration path
    let configmap = config.load("config.ini")?;
    for k in configmap.keys() {
        println!("{}", k);
    }

    // Test: Load AWS configuration file
    let awsconfigmap = config.load("/home/ngandhi/.aws/config")?;
    for k in awsconfigmap.keys() {
        println!("{}", k);
    }

    Ok(())
}
