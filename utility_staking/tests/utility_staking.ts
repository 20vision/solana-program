import { PROGRAM_ID as TOKEN_METADATA_PROGRAM_ID } from "@metaplex-foundation/mpl-token-metadata";
import * as anchor from "@coral-xyz/anchor";
import { UtilityStaking } from "../target/types/utility_staking";
import { Keypair,PublicKey, SYSVAR_RENT_PUBKEY, SystemProgram } from "@solana/web3.js";
import {
  getAssociatedTokenAddressSync,
  ASSOCIATED_TOKEN_PROGRAM_ID,
  TOKEN_PROGRAM_ID,
} from "@solana/spl-token";
import uniqid from 'uniqid';
import { publicKey } from "@coral-xyz/anchor/dist/cjs/utils";

describe("NFT Minter", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const payer = provider.wallet as anchor.Wallet;
  const program = anchor.workspace.UtilityStaking as anchor.Program<UtilityStaking>;

  const mintAccount = anchor.web3.Keypair.generate();

  const [constraint_signer_list] = PublicKey.findProgramAddressSync(
    [Buffer.from("constraint_signer_list"),mintAccount.publicKey.toBuffer()],
    program.programId
  );

  const [multi_sig_admin_list] = PublicKey.findProgramAddressSync(
    [Buffer.from("multi_sig_admin_list"),mintAccount.publicKey.toBuffer()],
    program.programId
  );

  it("Create a token!", async () => {

    // ctx: Context<Initialize>,
    // seed: String,
    // constraint_signer: Pubkey,
    // admin_signer: Pubkey,
    // token_name: String,
    // token_symbol: String,
    // token_uri: String,

    const transactionSignature = await program.methods
      .initialize(payer.publicKey,payer.publicKey)
      .accounts({
        payer: payer.publicKey,
        mintAccount: mintAccount.publicKey,
        constraintSignerListAccount: constraint_signer_list,
        multiSigAdminListAccount: multi_sig_admin_list,
        systemProgram: SystemProgram.programId,
        rent: SYSVAR_RENT_PUBKEY,
      })
      .signers([mintAccount])
      .rpc();
    
    const userAccount = await program.account.multiSigAdminList.fetch(
      multi_sig_admin_list
    )

    console.log('userAccount',userAccount)

    console.log("Success!");
    console.log(`   Mint Address: ${mintAccount.publicKey}`);
    console.log(`   Transaction Signature: ${transactionSignature}`);
  });

  it("Buy 1!", async () => {
    // Derive the associated token address account for the mint and payer.
    const [associatedUtilityStakeAccount] = PublicKey.findProgramAddressSync([
      mintAccount.publicKey.toBuffer(),
      payer.publicKey.toBuffer()
    ],
    program.programId);

    // Amount of tokens to mint.
    const amount_in = new anchor.BN(100);
    const min_amount_out = new anchor.BN(100);

    const transactionSignature = await program.methods
      .buy(amount_in,min_amount_out)
      .accounts({
        payer: payer.publicKey,
        mintAccount: mintAccount.publicKey,
        associatedUtilityStakeAccount: associatedUtilityStakeAccount,
        constraintSignerListAccount: constraint_signer_list,
        systemProgram: SystemProgram.programId,
      })
      .rpc();

    console.log("Success!");
    console.log(
      `   Associated Token Account Address: ${associatedUtilityStakeAccount}`
    );
    console.log(`   Transaction Signature: ${transactionSignature}`);
  });
});
