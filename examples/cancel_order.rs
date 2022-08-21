use binance::{api::Binance, config::Config};

#[tokio::main]
async fn main() {
    let config = Config::testnet();
    let account: binance::account::Account = Binance::new_with_config(
        Some("jCOMqJG7crShBe1X48WE6eava1nnyhr1tubqMCFw1KWQYW33psqOkkdG8gM7fVUP".to_string()),
        Some("GtBJ4TP4vmOTgHJJZW8SaWtyKQue7MaNG7SFhKpHbtTOC0ANkeef3xHx4U5QwDas".to_string()),
        &config,
    );

    println!("{:?}", account.get_all_open_orders().await.unwrap());
    let resp = account.cancel_all_open_orders("LTCUSDT").await.unwrap();
    println!("Cancelling all open orders: {:?}", resp);

}