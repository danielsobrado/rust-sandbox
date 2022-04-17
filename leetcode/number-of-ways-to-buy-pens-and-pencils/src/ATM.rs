struct ATM {

}

// There is an ATM machine that stores banknotes of 5 denominations: 20, 50, 100, 200, and 500 dollars. 
// Initially the ATM is empty. The user can use the machine to deposit or withdraw any amount of money.
// When withdrawing, the machine prioritizes using banknotes of larger values.
// For example, if you want to withdraw $300 and there are 2 $50 banknotes, 1 $100 banknote, and 1 $200 banknote, 
// then the machine will use the $100 and $200 banknotes.
// However, if you try to withdraw $600 and there are 3 $200 banknotes and 1 $500 banknote, 
// then the withdraw request will be rejected because the machine will first try to use the $500 banknote and 
// then be unable to use banknotes to complete the remaining $100. 
// Note that the machine is not allowed to use the $200 banknotes instead of the $500 banknote.
// Implement the ATM class:
// - ATM() Initializes the ATM object.
// - void deposit(int[] banknotesCount) Deposits new banknotes in the order $20, $50, $100, $200, and $500.
// - int[] withdraw(int amount) Returns an array of length 5 of the number of banknotes 
//   that will be handed to the user in the order $20, $50, $100, $200, and $500, and update the number of banknotes in the ATM after withdrawing. Returns [-1] if it is not possible (do not withdraw any banknotes in this case).


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl ATM {

    fn new() -> Self {
        vec![]
    }
    
    fn deposit(&self, banknotes_count: Vec<i32>) {
        vec![]
    }
    
    fn withdraw(&self, amount: i32) -> Vec<i32> {
        vec![]
    }
}

/**
 * Your ATM object will be instantiated and called as such:
 * let obj = ATM::new();
 * obj.deposit(banknotesCount);
 * let ret_2: Vec<i32> = obj.withdraw(amount);
 */

#[test]
fn test1() {
    let obj = ATM::new();
    obj.deposit(vec![0,0,1,2,1]);
    let ret_1: Vec<i32> = obj.withdraw(600);
    obj.deposit(vec![0,1,0,1,1]);
    let ret_2: Vec<i32> = obj.withdraw(600);
    let ret_3: Vec<i32> = obj.withdraw(600);
    assert_eq!(ret_1, vec![-1]);
}