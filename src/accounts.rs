use anyhow::{ensure, Result};
use jupiter_amm_interface::SwapParams;
use solana_sdk::{instruction::AccountMeta, pubkey::Pubkey};

use crate::constants::{ORCA_MINT, TOKEN_PROGRAM, XORCA_MINT};

pub struct XorcaStakeAccounts {
    staker_account: Pubkey,
    vault_account: Pubkey,
    staker_orca_ata: Pubkey,
    staker_xorca_ata: Pubkey,
    xorca_mint_account: Pubkey,
    state_account: Pubkey,
    orca_mint_account: Pubkey,
    token_program_account: Pubkey,
}

impl XorcaStakeAccounts {
    pub fn from_swap_params(
        swap_params: &SwapParams,
        state_key: Pubkey,
        vault_key: Pubkey,
    ) -> Result<Self> {
        ensure!(
            swap_params.source_mint == ORCA_MINT && swap_params.destination_mint == XORCA_MINT,
            "invalid mint direction for xORCA stake"
        );

        Ok(Self {
            staker_account: swap_params.token_transfer_authority,
            vault_account: vault_key,
            staker_orca_ata: swap_params.source_token_account,
            staker_xorca_ata: swap_params.destination_token_account,
            xorca_mint_account: swap_params.destination_mint,
            state_account: state_key,
            orca_mint_account: swap_params.source_mint,
            token_program_account: TOKEN_PROGRAM,
        })
    }

    pub fn into_metas(self) -> Vec<AccountMeta> {
        vec![
            AccountMeta::new(self.staker_account, true),
            AccountMeta::new(self.vault_account, false),
            AccountMeta::new(self.staker_orca_ata, false),
            AccountMeta::new(self.staker_xorca_ata, false),
            AccountMeta::new(self.xorca_mint_account, false),
            AccountMeta::new_readonly(self.state_account, false),
            AccountMeta::new_readonly(self.orca_mint_account, false),
            AccountMeta::new_readonly(self.token_program_account, false),
        ]
    }
}
