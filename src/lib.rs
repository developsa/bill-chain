mod test;

use reqwest::Client;
use serde::{Deserialize, Serialize};
use soroban_sdk::{contract, contractimpl, Address, Env, Error};

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountData {
    pub balance: String,
    pub other_info: String,
}

pub struct WalletNavigator {
    client: Client,
    base_url: String,
}

impl WalletNavigator {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            base_url: "https://horizon-testnet.stellar.org".to_string(),
        }
    }

    pub async fn get_account_data(&self, account_id: &str) -> Result<AccountData, reqwest::Error> {
        let url = format!("{}/accounts/{}", self.base_url, account_id);
        let response = self.client.get(&url).send().await?.json::<AccountData>().await?;
        Ok(response)
    }
}


#[contract]
pub struct BillChain;

#[contractimpl]
impl BillChain {
    pub fn bill(
        env: Env,
        user_address: Address,
        institution_address: Address,
        amount: i128,
        message: Option<String>,
    ) -> Result<(), Error> {
        user_address.require_auth();
        let balance = env.storage().instance().get(&user_address).unwrap_or(0i128);
        if balance < amount {
            println!("Failed to balance less than amount");
        }

        let current_user_balance = env.storage().instance().set(&user_address, &(balance - amount));
        let institution_balance = env.storage().instance().get(&institution_address).unwrap_or(0i128);
        let current_institution_balance = env.storage().instance().set(&institution_address, &(institution_balance + amount));

        if let Some(msg) = message {
            env.events().publish(("bill", user_address, institution_address), msg);
        }

        Ok(())
    }

    pub fn check_balance(env: Env, address: Address) -> i128 {
        env.storage().instance().get(&address).unwrap_or(0i128)
    }
}


