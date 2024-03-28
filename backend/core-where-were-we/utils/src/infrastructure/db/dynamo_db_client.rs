use aws_sdk_dynamodb as dynamodb;

pub async fn dynamodb_client() -> dynamodb::Client {
    let config = aws_config::load_from_env().await;
    aws_sdk_dynamodb::Client::new(&config)
}