import { PROGRAM_ID as TOKEN_METADATA_PROGRAM_ID } from "@metaplex-foundation/mpl-token-metadata";
import * as anchor from "@coral-xyz/anchor";
import { UtilityStaking } from "../target/types/utility_staking";
import { PublicKey, SYSVAR_RENT_PUBKEY, SystemProgram } from "@solana/web3.js";
import {
  getAssociatedTokenAddressSync,
  ASSOCIATED_TOKEN_PROGRAM_ID,
  TOKEN_PROGRAM_ID,
} from "@solana/spl-token";
import uniqid from 'uniqid';

describe("NFT Minter", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const payer = provider.wallet as anchor.Wallet;
  const program = anchor.workspace.UtilityStaking as anchor.Program<UtilityStaking>;

  const seed = uniqid()

  // Derive the PDA to use as mint account address.
  // This same PDA is also used as the mint authority.
  const [mintPDA] = PublicKey.findProgramAddressSync(
    [Buffer.from(seed)],
    program.programId
  );

  const [collateralPDA] = PublicKey.findProgramAddressSync(
    [Buffer.from("collateral"),mintPDA.toBuffer()],
    program.programId
  );

  const [constraint_signer_list] = PublicKey.findProgramAddressSync(
    [Buffer.from("constraint_signer_list"),mintPDA.toBuffer()],
    program.programId
  );

  const [multi_sig_admin_list] = PublicKey.findProgramAddressSync(
    [Buffer.from("multi_sig_admin_list"),mintPDA.toBuffer()],
    program.programId
  );

  const metadata = {
    name: "20Vision",
    symbol: "/20vision",
    uri: "https://raw.githubusercontent.com/20vision/example/master/hello.json",
  };

  it("Create a token!", async () => {
    // Derive the metadata account address.
    const [metadataAddress] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("metadata"),
        TOKEN_METADATA_PROGRAM_ID.toBuffer(),
        mintPDA.toBuffer(),
      ],
      TOKEN_METADATA_PROGRAM_ID
    );

    // ctx: Context<Initialize>,
    // seed: String,
    // constraint_signer: Pubkey,
    // admin_signer: Pubkey,
    // token_name: String,
    // token_symbol: String,
    // token_uri: String,

    const transactionSignature = await program.methods
      .initialize(seed,payer.publicKey,payer.publicKey, metadata.name, metadata.symbol, metadata.uri)
      .accounts({
        payer: payer.publicKey,
        mintAccount: mintPDA,
        collateralAccount: collateralPDA,
        constraintSigner: constraint_signer_list,
        multiSigAdminList: multi_sig_admin_list,
        metadataAccount: metadataAddress,
        tokenProgram: TOKEN_PROGRAM_ID,
        tokenMetadataProgram: TOKEN_METADATA_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
        rent: SYSVAR_RENT_PUBKEY,
      })
      .rpc();

    console.log("Success!");
    console.log(`   Mint Address: ${mintPDA}`);
    console.log(`   Transaction Signature: ${transactionSignature}`);
  });

  it("Mint 1 Token!", async () => {
    // Derive the associated token address account for the mint and payer.
    const associatedTokenAccountAddress = getAssociatedTokenAddressSync(
      mintPDA,
      payer.publicKey
    );

    // Amount of tokens to mint.
    const amount = new anchor.BN(100);

    const transactionSignature = await program.methods
      .mintToken(seed,amount)
      .accounts({
        payer: payer.publicKey,
        mintAccount: mintPDA,
        associatedTokenAccount: associatedTokenAccountAddress,
        tokenProgram: TOKEN_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
      })
      .rpc();

    console.log("Success!");
    console.log(
      `   Associated Token Account Address: ${associatedTokenAccountAddress}`
    );
    console.log(`   Transaction Signature: ${transactionSignature}`);
  });
});
