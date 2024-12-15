use std::collections::HashMap;
use std::sync::Mutex;
use lazy_static::lazy_static;

pub struct TransferArgs {
    pub sender: String,  // Changed from 'to'
    pub receiver: String,  // New field
    pub amount: u64,
}

lazy_static! {
    static ref WALLETS: Mutex<HashMap<String, u64>> = Mutex::new(HashMap::new());
}

pub fn create_wallet(user: String, initial_balance: u64) -> Result<(), String> {
    let mut wallets = WALLETS.lock().map_err(|_| "Failed to lock wallets")?;
    
    if wallets.contains_key(&user) {
        return Err("Wallet already exists".to_string());
    }
    
    wallets.insert(user, initial_balance);
    Ok(())
}

pub fn get_balance(user: String) -> u64 {
    let wallets = WALLETS.lock().unwrap_or_else(|e| e.into_inner());
    *wallets.get(&user).unwrap_or(&0)
}

pub fn transfer(args: TransferArgs) -> Result<(), String> {
    let mut wallets = WALLETS.lock().map_err(|_| "Failed to lock wallets")?;
    
    // Check if sender and receiver wallets exist
    if !wallets.contains_key(&args.sender) {
        return Err("Sender wallet does not exist".to_string());
    }
    
    if !wallets.contains_key(&args.receiver) {
        return Err("Receiver wallet does not exist".to_string());
    }
    
    // Get sender's balance
    let sender_balance = *wallets.get(&args.sender).unwrap();
    
    // Check if sender has sufficient balance
    if sender_balance < args.amount {
        return Err("Insufficient balance".to_string());
    }
    
    // Perform transfer
    wallets.entry(args.sender.clone()).and_modify(|balance| *balance -= args.amount);
    wallets.entry(args.receiver.clone()).and_modify(|balance| *balance += args.amount);
    
    Ok(())
}