use bank::SavingsAccount;

//because utils is a child module
mod utils;

#[test]
fn it_should_have_a_starting_balance_of_0() {
    
    //because utils is a child module, now we can use its function here.
    utils::common_setup();

    let account = SavingsAccount::new();
    assert_eq!(account.get_balance(), 0);
}