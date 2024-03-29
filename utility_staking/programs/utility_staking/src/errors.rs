use anchor_lang::prelude::*;

#[error_code]
pub enum ContractError {
    #[msg("Buy request provided wrong constraint signer.")]
    InvalidConstraintSigner,

    #[msg("Constraint signer did not sign the buy request.")]
    ConstraintSignerNotSigned,

    #[msg("Request provided wrong admin signer.")]
    InvalidAdminSigner,

    #[msg("Admin signer did not sign the transaction.")]
    AdminSignerNotSigned,

    #[msg("The price per token changed.")]
    PriceChanged,

    #[msg("You don't own enough Token.")]
    InsufficientTokenBalance,

    #[msg("Insufficient Collateral in Contract.")]
    InsufficientCollateralInContract,
    
    #[msg("There is still time left before the withdrawal deadline.")]
    StillTimeLeft,

    #[msg("You have to withdraw within 10 days after withdrawal deadline.")]
    TooLate,

    #[msg("You have to withdraw within 10 days after withdrawal deadline.")]
    InvalidInputAmount,
}
