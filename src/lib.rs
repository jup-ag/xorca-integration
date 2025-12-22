use anyhow::{Result, anyhow};
use jupiter_amm_interface::{
    AccountMap, Amm, AmmContext, KeyedAccount, Quote, QuoteParams, Swap, SwapAndAccountMetas,
    SwapMode, SwapParams, try_get_account_data,
};
use rust_decimal::Decimal;
use solana_sdk::pubkey::Pubkey;
use xorca::{convert_orca_to_xorca, find_orca_vault_address};

pub mod accounts;
pub mod constants;
pub mod pdas;
use accounts::XorcaStakeAccounts;
use constants::*;
use pdas::{State, Vault, XorcaMint};

#[derive(Clone, Debug)]
pub struct XorcaStakingProgram {
    pub label: String,
    pub program_id: Pubkey,
    pub state_key: Pubkey,
    pub vault_key: Pubkey,
    pub escrowed_orca_amount: u64,
    pub vault_orca_amount: Option<u64>,
    pub xorca_supply: Option<u64>,
}

impl XorcaStakingProgram {
    pub fn new(state: State, state_key: Pubkey) -> Self {
        let (vault_key, _) = find_orca_vault_address(&state_key, &TOKEN_PROGRAM, &ORCA_MINT)
            .expect("derive ORCA vault ATA");
        Self {
            label: XORCA_STAKING_PROGRAM_LABEL.to_owned(),
            program_id: XORCA_STAKING_PROGRAM_ID,
            state_key,
            vault_key,
            escrowed_orca_amount: state.escrowed_orca_amount,
            vault_orca_amount: None,
            xorca_supply: None,
        }
    }
}

impl Amm for XorcaStakingProgram {
    fn from_keyed_account(keyed_account: &KeyedAccount, _amm_context: &AmmContext) -> Result<Self> {
        let state = State::load(&keyed_account.account.data)?;
        Ok(Self::new(state, keyed_account.key))
    }

    fn label(&self) -> String {
        self.label.clone()
    }

    fn program_id(&self) -> Pubkey {
        self.program_id
    }

    fn key(&self) -> Pubkey {
        self.state_key
    }

    fn get_reserve_mints(&self) -> Vec<Pubkey> {
        vec![ORCA_MINT, XORCA_MINT]
    }

    fn get_accounts_to_update(&self) -> Vec<Pubkey> {
        vec![self.state_key, self.vault_key, XORCA_MINT]
    }

    fn update(&mut self, account_map: &AccountMap) -> Result<()> {
        let state_data = try_get_account_data(account_map, &self.state_key)?;
        let state = State::load(&state_data)?;
        self.escrowed_orca_amount = state.escrowed_orca_amount;

        let vault_data = try_get_account_data(account_map, &self.vault_key)?;
        let vault = Vault::load(&vault_data)?;
        self.vault_orca_amount = Some(vault.vault_orca_amount);

        let xorca_mint_data = try_get_account_data(account_map, &XORCA_MINT)?;
        let xorca_mint = XorcaMint::load(&xorca_mint_data)?;
        self.xorca_supply = Some(xorca_mint.xorca_supply);

        Ok(())
    }

    fn quote(&self, quote_params: &QuoteParams) -> Result<Quote> {
        let is_stake = quote_params.input_mint.eq(&ORCA_MINT);
        let is_exact_in = quote_params.swap_mode.eq(&SwapMode::ExactIn);
        if !is_stake || !is_exact_in {
            return Err(anyhow!("only exact in stake operations are supported"));
        }

        let vault_orca_amount = self
            .vault_orca_amount
            .ok_or_else(|| anyhow!("vault account not initialized or not loaded"))?;
        let escrowed_orca_amount = self.escrowed_orca_amount;
        let non_escrowed_orca_amount = vault_orca_amount - escrowed_orca_amount;
        let xorca_supply = self
            .xorca_supply
            .ok_or_else(|| anyhow!("xORCA mint supply not initialized or not loaded"))?;
        let xorca_amount_out =
            convert_orca_to_xorca(quote_params.amount, non_escrowed_orca_amount, xorca_supply)?;

        Ok(Quote {
            in_amount: quote_params.amount,
            out_amount: xorca_amount_out,
            // No fees on stake
            fee_amount: 0,
            fee_mint: quote_params.input_mint,
            fee_pct: Decimal::from(0),
        })
    }

    fn get_swap_and_account_metas(&self, swap_params: &SwapParams) -> Result<SwapAndAccountMetas> {
        let accounts =
            XorcaStakeAccounts::from_swap_params(swap_params, self.state_key, self.vault_key)?;
        Ok(SwapAndAccountMetas {
            // TODO: Add a new Swap variant for xORCA staking.
            // Instruction builder can be found here:
            // https://github.com/orca-so/xorca/blob/main/rust-client/src/generated/instructions/stake.rs
            swap: Swap::TokenSwap, // placeholder value
            account_metas: accounts.into_metas(),
        })
    }

    fn clone_amm(&self) -> Box<dyn Amm + Send + Sync> {
        Box::new(self.clone())
    }

    fn unidirectional(&self) -> bool {
        true
    }
}
