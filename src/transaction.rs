use serde::Deserialize;

use crate::amount::Amount;

/// A basic Transaction containing a type, client id, transaction number, and amount.
#[derive(Clone, Debug, Deserialize)]
pub struct Transaction {
    r#type: Kind,
    client: u16,
    tx: u32,
    amount: Option<Amount>,
}

impl Transaction {
    pub fn kind(&self) -> Kind {
        self.r#type
    }

    pub fn client(&self) -> u16 {
        self.client
    }

    pub fn id(&self) -> u32 {
        self.tx
    }

    pub fn amount(&self) -> Option<Amount> {
        self.amount
    }
}

/// Enum variant for the different types of transactions.
#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Kind {
    Deposit,
    Withdrawal,
    Dispute,
    Resolve,
    Chargeback,
}
