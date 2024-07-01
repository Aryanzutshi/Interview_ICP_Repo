#[cfg(test)]
mod tests {
    #[test]
    fn test_spender_init() {
        crate::spender_init("spender1".to_string(), 100);
        unsafe {
            let spender = crate::SPENDER_ACCOUNTS.iter().find(|acc| acc.as_ref().map_or(false, |a| a.spender_address == "spender1"));
            assert!(spender.is_some());
            assert_eq!(spender.unwrap().as_ref().unwrap().spender_amount, 100);
        }
    }

    #[test]
    fn test_check_balance() {
        crate::owner_init("owner1".to_string(), 1000);
        crate::spender_init("spender1".to_string(), 100);
        crate::set_allowance("owner1".to_string(), "spender1".to_string(), 50);
        crate::check_balance("owner1".to_string(), "spender1".to_string());
        unsafe {
            let owner = crate::OWNER_ACCOUNTS.iter().find(|acc| acc.as_ref().map_or(false, |a| a.owner_address == "owner1"));
            let spender = crate::SPENDER_ACCOUNTS.iter().find(|acc| acc.as_ref().map_or(false, |a| a.spender_address == "spender1"));
            assert_eq!(owner.unwrap().as_ref().unwrap().owner_amount, 950);
            assert_eq!(spender.unwrap().as_ref().unwrap().spender_amount, 100);
            assert_eq!(spender.unwrap().as_ref().unwrap().spender_allowance, 50);
        }
    }
    }

