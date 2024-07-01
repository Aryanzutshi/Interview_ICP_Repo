mod test;

const GAS_LIMIT: f32 = 256.0;
const PRICE_PER_UNIT: f32 = 0.0043;
const TOTAL_SUPPLY: u64 = 100000;
const GAS_FEES: f32 = GAS_LIMIT * PRICE_PER_UNIT;

//////////////////////////////////////////////////////////////////////////////////////////////////////////////
////////////////////////////// OWNER STRUCTS AND ENUMS /////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////////////////////////////////////

struct OwnerDetails {
    owner_address: String,
    owner_amount: u64,
    owner_is_active: bool,
}

enum OwnerErrors {
    OwnerNotInitialized,
    NotEnoughBalance,
    ExpectedLowFee, 
}

const MAX_OWNER_ACCOUNTS: usize = 5;
const ARRAY_REPEAT_VALUE: Option<OwnerDetails> = None;
static mut OWNER_ACCOUNTS: [Option<OwnerDetails>; MAX_OWNER_ACCOUNTS] = [ARRAY_REPEAT_VALUE; MAX_OWNER_ACCOUNTS];

//////////////////////////////////////////////////////////////////////////////////////////////////////////////
////////////////////////////// SPENDER STRUCTS AND ENUMS /////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////////////////////////////////////

struct SpenderDetails {
    spender_address: String,
    spender_amount: u64,
    spender_allowance: u64,
    spender_is_active: bool,
}

enum SpenderErrors {
    SpenderNotInitialized,
    NotEnoughBalance,
}

const MAX_SPENDER_ACCOUNTS: usize = 5;
const ARRAY_REPEAT_VALUE_SPENDER: Option<SpenderDetails> = None;
static mut SPENDER_ACCOUNTS: [Option<SpenderDetails>; MAX_SPENDER_ACCOUNTS] = [ARRAY_REPEAT_VALUE_SPENDER; MAX_SPENDER_ACCOUNTS];

//////////////////////////////////////////////////////////////////////////////////////////////////////////////
////////////////////////////// ACCOUNTS INITIALIZATION ///////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn owner_init(_address: String, _amount: u64) {
    let owner = OwnerDetails {
        owner_address: _address,
        owner_amount: _amount,
        owner_is_active: true,
    };

    unsafe {
        for account in OWNER_ACCOUNTS.iter_mut() {
            if account.is_none() {
                *account = Some(owner);
                return;
            }
        }
    }
}

pub fn spender_init(_address: String, _amount: u64) {
    let spender = SpenderDetails {
        spender_address: _address,
        spender_amount: _amount,
        spender_allowance: 0,
        spender_is_active: true,
    };

    unsafe {
        for account in SPENDER_ACCOUNTS.iter_mut() {
            if account.is_none() {
                *account = Some(spender);
                return;
            }
        }
    }
}

//////////////////////////////////////////////////////////////////////////////////////////////////////////////
////////////////////////////// SET ALLOWANCE /////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn set_allowance(owner_address: String, spender_address: String, allowance: u64) {
    unsafe {
        let owner = OWNER_ACCOUNTS.iter_mut().find(|acc| acc.as_ref().map_or(false, |a| a.owner_address == owner_address));
        let spender = SPENDER_ACCOUNTS.iter_mut().find(|acc| acc.as_ref().map_or(false, |a| a.spender_address == spender_address));

        if let (Some(Some(owner)), Some(Some(spender))) = (owner, spender) {
            if owner.owner_amount >= allowance {
                owner.owner_amount -= allowance;
                spender.spender_allowance += allowance;
                println!("Allowance set successfully. Current Allowance: {}", spender.spender_allowance);
            } else {
                println!("Error: Not enough balance in owner's account");
            }
        } else {
            println!("Error: Owner or spender account not found");
        }
    }
}

//////////////////////////////////////////////////////////////////////////////////////////////////////////////
////////////////////////////// CHECK BALANCE AND METADATA ////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn check_balance(owner_address: String, spender_address: String) {
    unsafe {
        let owner = OWNER_ACCOUNTS.iter().find(|acc| acc.as_ref().map_or(false, |a| a.owner_address == owner_address));
        let spender = SPENDER_ACCOUNTS.iter().find(|acc| acc.as_ref().map_or(false, |a| a.spender_address == spender_address));

        if let Some(Some(owner)) = owner {
            println!("The current balance of owner is: {}", owner.owner_amount);
        } else {
            println!("Owner account not found");
        }

        if let Some(Some(spender)) = spender {
            println!("The current balance of spender is: {}", spender.spender_amount);
            println!("The current allowance limit of spender is: {}", spender.spender_allowance);
        } else {
            println!("Spender account not found");
        }
    }
}

pub fn check_metadata() {
    println!("Gas Limit: {}", GAS_LIMIT);
    println!("Price Per Unit: {}", PRICE_PER_UNIT);
    println!("Current Gas fees: {}", GAS_FEES);
    println!("Total Supply of tokens: {}", TOTAL_SUPPLY);
}

//////////////////////////////////////////////////////////////////////////////////////////////////////////////
////////////////////////////// TRANSACTIONS //////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn transfer_from(spender_address: String, to: String, amount: u64, fee: f32) {
    unsafe {
        let spender = SPENDER_ACCOUNTS.iter_mut().find(|acc| acc.as_ref().map_or(false, |a| a.spender_address == spender_address));

        if let Some(Some(spender)) = spender {
            if spender.spender_allowance >= amount && fee <= GAS_FEES {
                spender.spender_allowance -= amount;
                println!("Transfer successful. Amount: {}, To: {}", amount, to);
            } else {
                println!("Error: Not enough allowance or gas fee too high");
            }
        } else {
            println!("Spender account not found");
        }
    }
}
