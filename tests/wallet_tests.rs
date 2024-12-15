use icp_token_wallet::{create_wallet, get_balance, transfer, TransferArgs};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wallet_operations() {
        // Create wallet
        let result = create_wallet("user1".to_string(), 1000);
        assert!(result.is_ok());

        // Check balance
        let balance = get_balance("user1".to_string());
        assert_eq!(balance, 1000);
    }

    #[test]
    fn test_transfer() {
        // Create sender and receiver wallets
        create_wallet("sender".to_string(), 500).unwrap();
        create_wallet("receiver".to_string(), 0).unwrap();

        // Transfer tokens
        let transfer_result = transfer(TransferArgs {
            sender: "sender".to_string(),
            receiver: "receiver".to_string(),
            amount: 200,
        });
        assert!(transfer_result.is_ok());

        // Check balances after transfer
        assert_eq!(get_balance("sender".to_string()), 300);
        assert_eq!(get_balance("receiver".to_string()), 200);
    }

    #[test]
    fn test_insufficient_balance() {
        // Create a wallet with low balance
        create_wallet("poor_user".to_string(), 100).unwrap();

        // Attempt transfer that exceeds balance
        let transfer_result = transfer(TransferArgs {
            sender: "poor_user".to_string(),
            receiver: "receiver".to_string(),
            amount: 200,
        });
        assert!(transfer_result.is_err());
    }
}