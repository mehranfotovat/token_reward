import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { TokenReward } from "../target/types/token_reward";
import { createMint, createAccount as createAccountSpl, createAssociatedTokenAccount, mintTo } from "@solana/spl-token";

describe("token_reward", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const payer = new anchor.web3.Keypair();

  const DECIMAL = 6
  const program = anchor.workspace.TokenReward as Program<TokenReward>;

  it("AirDrop SOL!", async () => {
    const signature = await provider.connection.requestAirdrop(payer.publicKey, 1000000000);
    await provider.connection.confirmTransaction(signature);
    // get balance of account
    const balance = await provider.connection.getBalance(payer.publicKey);
    console.log("Your SOL balance", balance);


  });

  it("Mint Token!", async () => {
    // Add your test here.
    // const tx = await program.methods.initialize().rpc();
    // console.log("Your transaction signature", tx);

    // token mint is the account that holds data about token 
    // this will return a pubkey for token mint
    const tokenMint = await createMint(
      provider.connection,
      payer,
      payer.publicKey,
      null, //payer.publicKey,
      DECIMAL,
    );
    console.log("Token Mint", tokenMint.toBase58());

    // const tokenAccount = await createAccountSpl(
    //   provider.connection,
    //   payer,
    //   tokenMint,
    //   payer.publicKey,
    // );

    // const associatedTokenAccount = await createAssociatedTokenAccount(
    //   provider.connection,
    //   payer,
    //   tokenMint,
    //   payer.publicKey,
    // );

  });
});
