use std::fmt;

use serde::{de, Deserialize, Deserializer, Serialize, Serializer};

/// A decimal value intended to be precise up to four decimal places. The underlying storage of
/// this floating point number is a u64, meaning the maximum value of a bank account would be
/// 1844674407370955.1615 (represented as 2 ^ 64).
///
/// This type currently supports basic add and subtraction, and will need an extension on it's api
/// if we want to handle overflows in the future. We can also change the underlying storage to
/// allocate more bits for larger account maximums or to handle negative values if we want to allow
/// accounts to be negative.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Amount(u64);

impl Amount {
    pub fn new() -> Self {
        Amount(0)
    }
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

        Ok(Self(amount))
    }
}

impl std::ops::Add for Amount {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0)
    }
}

impl std::ops::AddAssign for Amount {
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0
    }
}

impl std::ops::SubAssign for Amount {
    fn sub_assign(&mut self, other: Self) {
        self.0 -= other.0
    }
}

impl Serialize for Amount {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_f64(self.0 as f64 / 10000.0)
    }
}

#[cfg(test)]
impl From<u64> for Amount {
    fn from(inner: u64) -> Self {
        Self(inner)
    }
}

#[cfg(test)]
impl From<f64> for Amount {
    fn from(inner: f64) -> Self {
        Self((inner * 10000.0).round() as u64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize() {
        let data = "1234.5678";

        let amount: Amount = serde_json::from_str(data).unwrap();
        assert_eq!(amount.0, 12345678)
    }
}
