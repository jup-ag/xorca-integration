use thiserror::Error;

// ----------------------------------
// STATE
// ----------------------------------

#[derive(Debug)]
pub struct State {
    pub escrowed_orca_amount: u64,
    pub cool_down_period_s: i64,
}

impl State {
    pub fn load(account_data: &[u8]) -> PdaDataResult<Self> {
        let mut offset = 8; // skip discriminator, padding1, bump and vault_bump
        let escrowed_orca_amount = read_u64(account_data, &mut offset)?;
        let cool_down_period_s = read_i64(account_data, &mut offset)?;
        Ok(Self {
            escrowed_orca_amount,
            cool_down_period_s,
        })
    }
}

// ----------------------------------
// VAULT
// ----------------------------------

#[derive(Debug)]
pub struct Vault {
    pub vault_orca_amount: u64,
}

impl Vault {
    pub fn load(account_data: &[u8]) -> PdaDataResult<Self> {
        let mut offset = 64; // skip token mint and owner
        let vault_orca_amount = read_u64(account_data, &mut offset)?;
        Ok(Self { vault_orca_amount })
    }
}

// ----------------------------------
// XORCA MINT
// ----------------------------------

#[derive(Debug)]
pub struct XorcaMint {
    pub xorca_supply: u64,
}

impl XorcaMint {
    pub fn load(account_data: &[u8]) -> PdaDataResult<Self> {
        let mut offset = 36; // skip mint authority
        let xorca_supply = read_u64(account_data, &mut offset)?;
        Ok(Self { xorca_supply })
    }
}

// ----------------------------------
// TYPES
// ----------------------------------

#[derive(Debug, Error)]
pub enum PdaDataError {
    #[error("account data too short: need at least {needed} bytes, found {found}")]
    TooShort { needed: usize, found: usize },
}

pub type PdaDataResult<T> = std::result::Result<T, PdaDataError>;

// ----------------------------------
// UTILS
// ----------------------------------

const U64_SIZE: usize = 8;
const I64_SIZE: usize = 8;

fn read_u64(data: &[u8], offset: &mut usize) -> PdaDataResult<u64> {
    let end = *offset + U64_SIZE;
    let bytes = data.get(*offset..end).ok_or(PdaDataError::TooShort {
        needed: end,
        found: data.len(),
    })?;
    *offset = end;
    Ok(u64::from_le_bytes(
        bytes
            .try_into()
            .expect("slice length checked to be 8 bytes"),
    ))
}

fn read_i64(data: &[u8], offset: &mut usize) -> PdaDataResult<i64> {
    let end = *offset + I64_SIZE;
    let bytes = data.get(*offset..end).ok_or(PdaDataError::TooShort {
        needed: end,
        found: data.len(),
    })?;
    *offset = end;
    Ok(i64::from_le_bytes(
        bytes
            .try_into()
            .expect("slice length checked to be 8 bytes"),
    ))
}
