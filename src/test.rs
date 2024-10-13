#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use dotenv::dotenv;
    use serde_json::Value;
    use soroban_sdk::{
        testutils::{Address as _, Ledger},
        Address,
        Env,
    };

    use crate::{BillChain, BillChainClient, WalletNavigator};

    #[tokio::test]
    async fn test_get_account_data() {
        let wallet_navigator = WalletNavigator::new();
        dotenv().ok();
        let account_id = env::var("ACCOUNT_ID").expect("ACCOUNT_ID not set");;
        let url = format!("{}/accounts/{}", wallet_navigator.base_url, account_id);

        let result = wallet_navigator.client.get(&url).send().await;
        println!("{:?}", result);
        match result {
            Ok(response) => {
                let json_data: Value = response.json().await.unwrap();
                println!("JSON data: {:?}", json_data);
                if let Some(balances) = json_data["balances"].as_array() {
                    for balance in balances {
                        if balance["asset_type"] == "native" {
                            println!("Wallet Balance (XLM): {:?}", balance["balance"]);
                        }
                    }
                } else {
                    println!("Not balance data");
                }
            }
            Err(e) => {
                eprintln!("Hata: {:?}", e);
            }
        }

        println!("{:?} ", account_id);
    }

    #[test]
    fn test_bill_payment() {
        let env = Env::default();
        let contract_id = env.register_contract(None, BillChain);
        let client = BillChainClient::new(&env, &contract_id);

        let user1 = Address::generate(&env);
        let user2 = Address::generate(&env);
        println!("user1: {:?}", user1);
        println!("user2: {:?}", user2);
    }
}
