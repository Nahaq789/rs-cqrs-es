# rs-cqrs-es

build command
cargo lambda build --release --target x86_64-unknown-linux-gnu

terraForm deploy command
terraform plan
terraform apply

docker build --platform=linux/amd64 -f Dockerfile.write -t write-api-lambda-repo .