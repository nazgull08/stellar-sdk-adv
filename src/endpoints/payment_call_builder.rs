use std::collections::HashMap;

use crate::api_call::api_call;
use crate::endpoints::{horizon::Record, CallBuilder, Server};
use crate::types::Operation;
use crate::utils::{Direction, Endpoint};

#[derive(Debug)]
pub struct PaymentCallBuilder<'a> {
    server_url: &'a str,
    endpoint: Endpoint,
    query_params: HashMap<String, String>,
    token: &'a Option<String>,
}

impl<'a> PaymentCallBuilder<'a> {
    pub fn new(s: &'a Server) -> Self {
        Self {
            server_url: &s.0,
            endpoint: Endpoint::None,
            query_params: HashMap::new(),
            token: &s.1,
        }
    }
}

impl<'a> CallBuilder<Operation> for PaymentCallBuilder<'a> {
    fn cursor(&mut self, cursor: &str) -> &mut Self {
        self.query_params
            .insert(String::from("cursor"), String::from(cursor));

        self
    }

    fn order(&mut self, dir: Direction) -> &mut Self {
        self.query_params
            .insert(String::from("order"), dir.to_string());

        self
    }

    fn limit(&mut self, limit: u8) -> &mut Self {
        self.query_params
            .insert(String::from("limit"), limit.to_string());

        self
    }

    fn for_endpoint(&mut self, endpoint: Endpoint) -> &mut Self {
        self.endpoint = endpoint;

        self
    }

    fn call(&self) -> Result<Record<Operation>, anyhow::Error> {
        let url = format!(
            "{}{}{}",
            &self.server_url,
            self.endpoint.as_str(),
            "/payments",
        );

        api_call::<Record<Operation>>(
            url,
            crate::types::HttpMethod::GET,
            &self.query_params,
            self.token,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn limit_payment_call_builder() {
        let s = Server::new(String::from("https://horizon.stellar.org"));

        let mut pcb = PaymentCallBuilder::new(&s);

        let payment_records = pcb
            .for_endpoint(Endpoint::Accounts(String::from(
                "GAUZUPTHOMSZEV65VNSRMUDAAE4VBMSRYYAX3UOWYU3BQUZ6OK65NOWM",
            )))
            .limit(200)
            .call()
            .unwrap();

        assert_eq!(payment_records._embedded.records.len(), 200);
    }
}
