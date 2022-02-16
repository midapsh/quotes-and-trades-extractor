use std::time::SystemTime;

use chrono;
use futures::future;

use crate::services::get_debank_balance_service::get_debank_balance_service;

pub async fn debank_process() {
    println!("\n[{:?}]Getting balances...", chrono::offset::Local::now());
    let wallets = ["0x84D34f4f83a87596Cd3FB6887cFf8F17Bf5A7B83"];

    let start = SystemTime::now();
    let wallet_balances = future::try_join_all(
        wallets
            .into_iter()
            .map(|wallet| get_debank_balance_service(wallet.into())),
    )
    .await;
    let wallet_balances = match wallet_balances {
        Ok(wbs) => wbs,
        Err(_error) => return,
    };

    for wallet_balance in wallet_balances {
        if let Some(wb) = wallet_balance {
            for balance in wb.chain_list {
                println!("{:?}", balance);
                break;
            }
        }
    }
    let time_elapsed = start.elapsed();
    if let Ok(te) = time_elapsed {
        println!(" /// Time elapsed: {:?}", te);
    }
}
