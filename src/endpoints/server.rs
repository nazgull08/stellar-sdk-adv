use serde_json;

use crate::endpoints::{
    fee_stats::FeeStats, ledger_call_builder::LedgerCallBuilder, Account, AccountCallBuilder,
    AssetCallBuilder, Ledger, Offer, Transaction, TransactionCallBuilder,
};
use crate::utils::{req, Endpoint};

use super::OfferCallBuilder;

#[derive(Debug)]
pub struct Server(pub String);

impl Server {
    pub fn new(network_id: String) -> Self {
        Server(network_id)
    }

    pub fn accounts(&self) -> AccountCallBuilder {
        AccountCallBuilder {
            server: self,
            cursor: None,
            order: None,
            sponsor: None,
            limit: None,
            signer: None,
            liquidity_pool: None,
            asset: None,
            endpoint: Endpoint::None,
        }
    }

    pub fn assets(&self) -> AssetCallBuilder {
        AssetCallBuilder {
            server: self,
            cursor: None,
            order: None,
            limit: None,
            asset_code: None,
            asset_issuer: None,
            endpoint: Endpoint::None,
        }
    }

    pub fn load_account(&self, account_id: &str) -> Result<Account, &str> {
        let url = format!("{}/accounts/{}", self.0, account_id);
        let resp = req(&url).unwrap();

        let parsed: Account = serde_json::from_str(&resp).unwrap();

        Ok(parsed)
    }

    pub fn load_transaction(&self, hash: &str) -> Result<Transaction, &str> {
        let url = format!("{}/transactions/{}", self.0, hash);
        let resp = req(&url).unwrap();

        let parsed: Transaction = serde_json::from_str(&resp).unwrap();

        Ok(parsed)
    }

    pub fn transactions(&self) -> TransactionCallBuilder {
        TransactionCallBuilder {
            server: self,
            cursor: None,
            order: None,
            limit: None,
            include_failed: false,
            endpoint: Endpoint::None,
        }
    }

    pub fn load_ledger(&self, sequence: u64) -> Result<Ledger, &str> {
        let url = format!("{}/ledgers/{}", self.0, sequence);
        let resp = req(&url).unwrap();

        let parsed: Ledger = serde_json::from_str(&resp).unwrap();

        Ok(parsed)
    }

    pub fn ledgers(&self) -> LedgerCallBuilder {
        LedgerCallBuilder {
            server: self,
            cursor: None,
            order: None,
            limit: None,
            endpoint: Endpoint::None,
        }
    }

    pub fn load_offer(&self, offer_id: &str) -> Result<Offer, &str> {
        let url = format!("{}/offers/{}", self.0, offer_id);
        let resp = req(&url).unwrap();

        let parsed: Offer = serde_json::from_str(&resp).unwrap();

        Ok(parsed)
    }

    pub fn offers(&self) -> OfferCallBuilder {
        OfferCallBuilder {
            server: self,
            cursor: None,
            order: None,
            limit: None,
            buying: None,
            seller: None,
            selling: None,
            sponsor: None,
            endpoint: Endpoint::None,
        }
    }

    pub fn fee_stats(&self) -> Result<FeeStats, &str> {
        let url = format!("{}/fee_stats", self.0);
        let resp = req(&url).unwrap();

        let parsed: FeeStats = serde_json::from_str(&resp).unwrap();

        Ok(parsed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_account() {
        let s = Server::new(String::from("https://horizon.stellar.org"));

        let tx = s
            .load_account("GAUZUPTHOMSZEV65VNSRMUDAAE4VBMSRYYAX3UOWYU3BQUZ6OK65NOWM")
            .unwrap();

        assert_eq!(tx.id, tx.account_id);
    }

    #[test]
    fn test_load_transaction() {
        let s = Server::new(String::from("https://horizon.stellar.org"));

        let tx = s
            .load_transaction("3389e9f0f1a65f19736cacf544c2e825313e8447f569233bb8db39aa607c8889")
            .unwrap();

        assert_eq!(tx.id, tx.hash);
    }

    #[test]
    fn test_load_ledger() {
        let s = Server::new(String::from("https://horizon.stellar.org"));

        let ledger3 = s.load_ledger(3).unwrap();
        let ledger4 = s.load_ledger(4).unwrap();

        assert_eq!(ledger3.hash, ledger4.prev_hash);
    }

    #[test]
    fn test_load_fee_stats() {
        let s = Server::new(String::from("https://horizon.stellar.org"));

        let _fee_stats = s.fee_stats().unwrap();
    }
}
