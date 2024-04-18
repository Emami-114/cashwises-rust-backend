install:
	cargo add actix-web
	cargo add actix-cors
	cargo add serde_json
	cargo add async-trait
	cargo add serde --features derive
	cargo add chrono --features serde
	cargo add futures-util
	cargo add env_logger
	cargo add dotenv
	cargo add uuid --features "serde v4"
	cargo add sqlx --features "tls-native-tls runtime-async-std postgres chrono uuid"
	cargo add jsonwebtoken
	cargo add argon2
	cargo add validator -F derive
	cargo install cargo-watch
	cargo install sqlx-cli
	cargo add actix-multipart
	cargo add image
	cargo add mime
	cargo add handlebars
	cargo add lettre -F "tokio1, tokio1-native-tls"
start-server:
	cargo watch -q -c -w src/ -x run

db-up:
	sqlx migrate run
db-prepare:
	cargo sqlx prepare

db-down:
	sqlx migrate revert