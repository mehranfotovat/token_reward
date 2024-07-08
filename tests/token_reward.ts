import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { TokenReward } from "../target/types/token_reward";
import { createMint, createAccount as createAccountSpl, createAssociatedTokenAccount, getOrCreateAssociatedTokenAccount, mintTo, transfer, getAssociatedTokenAddress, TOKEN_PROGRAM_ID, MINT_SIZE, createInitializeMintInstruction, createAssociatedTokenAccountInstruction } from "@solana/spl-token";

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

  // it("Mint Token and transfer using client!", async () => {
  //   console.log("payer address", payer.publicKey.toBase58());

  //   // token mint is the account that holds data about token 
  //   // this will return a pubkey for token mint
  //   const tokenMint = await createMint(
  //     provider.connection,
  //     payer,
  //     payer.publicKey,
  //     null, //payer.publicKey,
  //     DECIMAL,
  //   );
  //   console.log("Token Mint", tokenMint.toBase58());

  //   const tokenAccount = await getOrCreateAssociatedTokenAccount(
  //     provider.connection,
  //     payer,
  //     tokenMint,
  //     payer.publicKey,
  //   )

  //   console.log("Token Account: ", tokenAccount.address.toBase58())

  //   // Our token has two decimal places
  //   const MINOR_UNITS_PER_MAJOR_UNITS = Math.pow(10, 6);

  //   const transactionSignature = await mintTo(
  //     provider.connection,
  //     payer,
  //     tokenMint,
  //     tokenAccount.address,
  //     payer.publicKey,
  //     100 * MINOR_UNITS_PER_MAJOR_UNITS
  //   );

  //   console.log('transactionSignature of minting to associate token account: ', transactionSignature);

  //   const tokenRecieverAddress = new anchor.web3.Keypair();
  
  //   const tokenRecieverTokenAddress = await getOrCreateAssociatedTokenAccount(
  //     provider.connection,
  //     payer,
  //     tokenMint,
  //     tokenRecieverAddress.publicKey
  //   );
  
  //   console.log("token reciever ata account: ", tokenRecieverTokenAddress.address.toBase58());

  //   const transferSignature = await transfer(
  //     provider.connection,
  //     payer,
  //     tokenAccount.address,
  //     tokenRecieverTokenAddress.address,
  //     payer.publicKey,
  //     5 * MINOR_UNITS_PER_MAJOR_UNITS
  //   );

  //   console.log("transaction of token transfer: ", transactionSignature)
  
  // });

  // it("Create PDA mint and transfer nft!", async () => {
  //   const [tokenPDA] = anchor.web3.PublicKey.findProgramAddressSync(
  //     [payer.publicKey.toBuffer()],
  //     program.programId
  //   );

  //   const creatingPDAProgramTx = await program.methods.initialize("Mehran Token", new anchor.BN(1000)).accounts({
  //     tokenMint: tokenPDA,
  //     payer: payer.publicKey
  //   }).signers([payer]).rpc();

  //   console.log(await program.account.myToken.fetch(tokenPDA.toBase58()));
  //   console.log(creatingPDAProgramTx)
  //   console.log(tokenPDA.toBase58())
  // });

  it("Create Token With Program!", async () => {
    const MINOR_UNITS_PER_MAJOR_UNITS = Math.pow(10, 6);
    const [tokenPDA, bump] = anchor.web3.PublicKey.findProgramAddressSync(
      [payer.publicKey.toBuffer()],
      program.programId
    );

    //TODO: token mint
    const tokenMint = await createMint(
      provider.connection,
      payer,
      tokenPDA,
      null,
      DECIMAL
    );
    //TODO: ata
    const tokenAccount = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      payer,
      tokenMint,
      tokenPDA,
      true
    );

    console.log("bump is: ", bump);
    console.log("Payer account: ", payer.publicKey.toBase58());
    console.log("My program token Mint address: ", tokenMint.toBase58());
    console.log("My ATA account: ", tokenAccount.address.toBase58());
    console.log("My token PDA account: ", tokenPDA.toBase58());

    const mintTokenTx = await program.methods.mintToken(new anchor.BN(100 * MINOR_UNITS_PER_MAJOR_UNITS)).accounts({
      tokenMint: tokenMint,
      tokenAccount: tokenAccount.address,
      payer: payer.publicKey,
      tokenPda: tokenPDA
    }).signers([payer]).rpc();
    console.log("token mint transaction: ",mintTokenTx);
  });
});
