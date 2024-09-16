/// A Savings Account data structure
pub struct SavingsAccount {
    balance: i32,
}

impl SavingsAccount {
    /// Creates instance of Savings Account
    /// with 0 balance
    /// ```
    /// use bank::SavingsAccount;
    /// let account = SavingsAccount::new();
    /// assert_eq!(account.get_balance(),0);
    /// ```
    pub fn new() -> SavingsAccount {
        SavingsAccount{
            balance: 0,
        }
    }

    /// Fetching Account Balance(Getter function)
    pub fn get_balance(&self) -> i32 { self.balance }

    /// Depositing into an account
    /// Will panic if the deposit amount is negative
    pub fn deposit(&mut self, amount: i32) { 
        if amount < 0 {
            panic!("Cannot deposit a negative amount");
        }
        self.balance += amount
        
    }

    /// will transfer from One account to other
    pub fn transfer(&self, acc_number: u32, amount: i32) -> Result<String,String> {
        Ok(format!("Transferred ${amount} to {acc_number}"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_have_a_starting_balance_of_0(){
        let account = SavingsAccount::new();
        assert_eq!(account.get_balance(), 0);
    }

    #[test]
    fn should_be_able_to_deposit(){
        let mut account = SavingsAccount::new();
        account.deposit(100);
        assert_eq!(account.get_balance(), 100);
        assert_ne!(account.get_balance(), 0);
        assert!(account.get_balance() == 100);
    }

    #[test]
    #[should_panic]
    fn should_panic_if_deposit_is_negative(){
        let mut account = SavingsAccount::new();
        account.deposit(-1);
    }

    #[test]
    fn should_be_able_to_transfer() -> Result<(),String> {
        let mut account = SavingsAccount::new();
        account.deposit(100);
        account.transfer(123456,100)?;
        Ok(())
    }
}