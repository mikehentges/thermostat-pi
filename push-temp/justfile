build: 
	cross build --release --target x86_64-unknown-linux-musl
	cp target/x86_64-unknown-linux-musl/release/push_temp bootstrap

test:
	curl -X POST -F "record_date=2022:02:03T15:50:00" -F "thermostat_on=true" -F "temperature=55" -F "thermostat_value=60" https://5zvz7wehuh.execute-api.us-east-2.amazonaws.com/test_lambda