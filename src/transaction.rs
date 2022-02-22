use std::fmt;

use serde::{de, Deserialize, Deserializer};

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

    pub fn amount(&self) -> Option<u64> {
        self.amount.map(|amount| amount.inner)
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

/// A decimal value intended to be precise up to four decimal places. The underlying storage of
/// this floating point number is a u64, meaning the maximum value of a bank account would be
/// 1844674407370955.1615 (represented as 2 ^ 64).
#[derive(Clone, Copy, Debug)]
struct Amount {
    inner: u64,
}

impl<'de> Deserialize<'de> for Amount {
    fn deserialize<D>(deserializer: D) -> Result<Amount, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AmountVisitor;

        impl<'de> de::Visitor<'de> for AmountVisitor {
            type Value = u64;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a floating point value precise up to four decimal places")
            }

            fn visit_f64<E>(self, value: f64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok((value * 10000.0).round() as u64)
            }
        }

        let amount = deserializer.deserialize_f64(AmountVisitor)?;

        Ok(Amount { inner: amount })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize() {
        let data = "1234.5678";

        // Parse the string of data into serde_json::Value.
        let amount: Amount = serde_json::from_str(data).unwrap();
        assert_eq!(amount.inner, 12345678)
    }
}
