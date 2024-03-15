use anchor_lang::prelude::*;

#[error_code]
pub enum ContractError {
    #[msg("A required constraint signer is missing.")]
    MissingConstraintSigner,

    #[msg("Signers are not in the correct order.")]
    IncorrectOrderOfSigners,

    #[msg("A required constraint signer did not sign the transaction.")]
    MissingSignatureConstraintSigner,

    #[msg("The price per token changed.")]
    PriceChanged,

    #[msg("You don't own enough Token.")]
    InsufficientTokenBalance,
}
