use lapin::{Connection, ConnectionProperties, options::{BasicConsumeOptions, BasicAckOptions}, types::FieldTable, message::DeliveryResult};
use reqwest::StatusCode;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub enum Visibility {
    Public,
    Private,
}

#[derive(Serialize, Deserialize)]
struct Post {
    author: String,
    content: String,
    visibility: Visibility
}
 
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + 'static>>{
    tracing_subscriber::fmt::init();
    dotenvy::dotenv().ok();

    let amqp_addr = std::env::var("AMQP_ADDR").expect("AMQP_ADDR must be set");
    let amqp_queue = std::env::var("AMQP_QUEUE").expect("AMQP_QUEUE must be set");

    let options = ConnectionProperties::default()
        .with_executor(tokio_executor_trait::Tokio::current())
        .with_reactor(tokio_reactor_trait::Tokio);
    let conn = Connection::connect(&amqp_addr, options)
        .await?;

    let channel = conn.create_channel().await?;
    let consumer = channel
        .basic_consume(
            &amqp_queue,
            "sm_consumer",
            BasicConsumeOptions::default(),
            FieldTable::default()
        )
        .await?;

    consumer.set_delegate(move |delivery: DeliveryResult| async move {
        let api_addr = std::env::var("API_ADDR")
            .expect("API_ADDR must be set");
        tracing::info!("API_ADDR: {}", api_addr);
        tracing::info!("Message being processed");

        let delivery = match delivery {
            Ok(Some(delivery)) => delivery,
            Ok(None) => return,
            Err(error) => {
                tracing::error!("Error caught while consuming message: {}", error);
                return;
            }
        };

        let content = delivery.data
            .iter()
            .map(|&byte| byte as char)
            .collect::<String>();

        tracing::info!("Got message: {}", content);

        let post: Result<Post, serde_json::Error> = serde_json::from_str(&content);
        let post = match post {
            Ok(x) => x,
            Err(error) => {
                tracing::error!("Failed to deserialize message: {}", error);
                return;
            }
        };

        let client = reqwest::Client::new();        
        let res = client.post(api_addr)
            .json(&post)
            .send()
            .await;

        match res {
            Ok(x) => {
                match x.status() {
                    StatusCode::OK => {
                        tracing::info!("API response: {}", x.status());
                    }, 
                    _ => {
                        tracing::error!("API response: {}", x.status());
                        return;
                    }
                }
            }
            Err(error) => {
                tracing::error!("Failed to send message to API: {}", error);
                return;
            }
        }
        
        delivery
            .ack(BasicAckOptions::default())
            .await
            .expect("Failed to ack message");
    });

    // Wait for closing signal
    tokio::signal::ctrl_c().await?;

    return Ok(());
}
