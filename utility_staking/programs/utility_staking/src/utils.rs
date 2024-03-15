use anchor_lang::prelude::*;

use crate::errors::ContractError;

// pub fn assert_unique_owners(owners: &[Pubkey]) -> Result<()> {
//     for (i, owner) in owners.iter().enumerate() {
//         require!(
//             !owners.iter().skip(i + 1).any(|item| item == owner),
//             ContractError::UniqueSigners
//         )
//     }
//     Ok(())
// }