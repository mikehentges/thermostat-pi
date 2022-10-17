TARGET_HOST := "mhentges@shop-therm.local"
TARGET_PATH := "/home/mhentges/thermostat-pi"
TARGET_ARCH := "arm-unknown-linux-gnueabihf"
SOURCE_PATH := "./target/" + TARGET_ARCH + "/release/thermostat-pi"

check:
    cargo check

build:
    cross build --release --target={{TARGET_ARCH}} --features vendored-openssl
    rsync ./configuration.yaml {{TARGET_HOST}}:/home/mhentges/configuration.yaml
    rsync {{SOURCE_PATH}} {{TARGET_HOST}}:{{TARGET_PATH}}
    #ssh -t {{TARGET_HOST}} {{TARGET_PATH}}

clippy:
    cargo clippy