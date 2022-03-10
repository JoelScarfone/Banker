use crate::amount::Amount;

/// A basic Bank account. A single `Account` will store the amount available, held, and if the
/// account is locked or not.
#[derive(Debug)]
pub struct Account {
    available: Amount,
    held: Amount,
    locked: bool,
}

impl Account {
    pub fn new() -> Self {
        Self {
            available: Amount::new(),
            held: Amount::new(),
            locked: false,
        }
    }

    pub fn available(&self) -> Amount {
        self.available
    }

    pub fn held(&self) -> Amount {
        self.held
    }

    pub fn total(&self) -> Amount {
        self.held + self.available
    }

    pub fn is_locked(&self) -> bool {
        self.locked
    }

    pub fn credit(&mut self, val: Amount) {
        self.available += val;
    }

    pub fn try_debit(&mut self, val: Amount) -> Result<(), ()> {
        if self.available >= val {
            self.available -= val;
            return Ok(());
        }
        return Err(());
    }

    pub fn try_dispute(&mut self, val: Amount) -> Result<(), ()> {
        if self.available >= val {
            self.available -= val;
            self.held += val;
            return Ok(());
        }
        return Err(());
    }

    pub fn try_resolve(&mut self, val: Amount) -> Result<(), ()> {
        if self.held >= val {
            self.held -= val;
            self.available += val;
            return Ok(());
        }
        return Err(());
    }

    pub fn try_chargeback(&mut self, val: Amount) -> Result<(), ()> {
        if self.held >= val {
            self.held -= val;
            self.locked = true;
            return Ok(());
        }
        return Err(());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn credit() {
        let mut account = Account::new();
        account.credit(1000.into());
        account.credit(3000.into());
        assert_eq!(account.available, 4000.into());
        assert!(!account.locked)
    }

    #[test]
    fn debit() {
        let mut account = Account::new();
        account.credit(1000.into());
        account.try_debit(500.into()).unwrap();
        assert_eq!(account.available, 500.into());
        assert!(!account.locked)
    }

    #[test]
    fn debit_fail() {
        let mut account = Account::new();
        account.credit(1000.into());
        account.try_debit(1500.into()).unwrap_err();
        assert!(!account.locked)
    }

    #[test]
    fn dispute() {
        let mut account = Account::new();
        account.credit(1000.into());
        account.try_dispute(1000.into()).unwrap();
        assert_eq!(account.total(), 1000.into());
        assert_eq!(account.held, 1000.into());
        assert_eq!(account.available, 0.into());
        assert!(!account.locked)
    }

    #[test]
    fn dispute_fail() {
        let mut account = Account::new();
        account.credit(1000.into());
        account.try_dispute(2000.into()).unwrap_err();
        assert!(!account.locked)
    }

    #[test]
    fn resolve() {
        let mut account = Account::new();
        account.credit(1000.into());
        account.credit(2000.into());
        account.try_dispute(1000.into()).unwrap();
        account.try_resolve(1000.into()).unwrap();
        assert_eq!(account.total(), 3000.into());
        assert_eq!(account.held, 0.into());
        assert_eq!(account.available, 3000.into());
        assert!(!account.locked)
    }

    #[test]
    fn resolve_fail() {
        let mut account = Account::new();
        account.credit(1000.into());
        account.credit(2000.into());
        account.try_dispute(1000.into()).unwrap();
        account.try_resolve(2000.into()).unwrap_err();
        assert!(!account.locked)
    }

    #[test]
    fn chargeback() {
        let mut account = Account::new();
        account.credit(1000.into());
        account.credit(2000.into());
        account.try_dispute(1000.into()).unwrap();
        account.try_chargeback(1000.into()).unwrap();
        assert_eq!(account.total(), 2000.into());
        assert_eq!(account.held, 0.into());
        assert_eq!(account.available, 2000.into());
        assert!(account.locked)
    }

    #[test]
    fn chargeback_fail() {
        let mut account = Account::new();
        account.credit(1000.into());
        account.credit(2000.into());
        account.try_dispute(1000.into()).unwrap();
        account.try_chargeback(2000.into()).unwrap_err();
        assert!(!account.locked)
    }
}
