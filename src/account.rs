#[derive(Debug)]
pub struct Account {
    available: u64,
    held: u64,
    locked: bool,
}

impl Account {
    pub fn new() -> Self {
        Self {
            available: 0,
            held: 0,
            locked: false,
        }
    }

    pub fn available(&self) -> u64 {
        self.available
    }

    pub fn held(&self) -> u64 {
        self.held
    }

    pub fn total(&self) -> u64 {
        // TODO: handle overflow
        self.held + self.available
    }

    pub fn is_locked(&self) -> bool {
        self.locked
    }

    pub fn credit(&mut self, val: u64) {
        self.available += val;
    }

    pub fn try_debit(&mut self, val: u64) -> Result<(), ()> {
        if self.available >= val {
            self.available -= val;
            return Ok(());
        }
        return Err(());
    }

    pub fn try_dispute(&mut self, val: u64) -> Result<(), ()> {
        if self.available >= val {
            self.available -= val;
            self.held += val;
            return Ok(());
        }
        return Err(());
    }

    pub fn try_resolve(&mut self, val: u64) -> Result<(), ()> {
        if self.held >= val {
            self.held -= val;
            self.available += val;
            return Ok(());
        }
        return Err(());
    }

    pub fn try_chargeback(&mut self, val: u64) -> Result<(), ()> {
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
        account.credit(1000);
        account.credit(3000);
        assert_eq!(account.available, 4000);
        assert!(!account.locked)
    }

    #[test]
    fn debit() {
        let mut account = Account::new();
        account.credit(1000);
        account.try_debit(500).unwrap();
        assert_eq!(account.available, 500);
        assert!(!account.locked)
    }

    #[test]
    fn debit_fail() {
        let mut account = Account::new();
        account.credit(1000);
        account.try_debit(1500).unwrap_err();
        assert!(!account.locked)
    }

    #[test]
    fn dispute() {
        let mut account = Account::new();
        account.credit(1000);
        account.try_dispute(1000).unwrap();
        assert_eq!(account.total(), 1000);
        assert_eq!(account.held, 1000);
        assert_eq!(account.available, 0);
        assert!(!account.locked)
    }

    #[test]
    fn dispute_fail() {
        let mut account = Account::new();
        account.credit(1000);
        account.try_dispute(2000).unwrap_err();
        assert!(!account.locked)
    }

    #[test]
    fn resolve() {
        let mut account = Account::new();
        account.credit(1000);
        account.credit(2000);
        account.try_dispute(1000).unwrap();
        account.try_resolve(1000).unwrap();
        assert_eq!(account.total(), 3000);
        assert_eq!(account.held, 0);
        assert_eq!(account.available, 3000);
        assert!(!account.locked)
    }

    #[test]
    fn resolve_fail() {
        let mut account = Account::new();
        account.credit(1000);
        account.credit(2000);
        account.try_dispute(1000).unwrap();
        account.try_resolve(2000).unwrap_err();
        assert!(!account.locked)
    }

    #[test]
    fn chargeback() {
        let mut account = Account::new();
        account.credit(1000);
        account.credit(2000);
        account.try_dispute(1000).unwrap();
        account.try_chargeback(1000).unwrap();
        assert_eq!(account.total(), 2000);
        assert_eq!(account.held, 0);
        assert_eq!(account.available, 2000);
        assert!(account.locked)
    }

    #[test]
    fn chargeback_fail() {
        let mut account = Account::new();
        account.credit(1000);
        account.credit(2000);
        account.try_dispute(1000).unwrap();
        account.try_chargeback(2000).unwrap_err();
        assert!(!account.locked)
    }
}
