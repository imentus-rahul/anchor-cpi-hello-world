import * as anchor from '@project-serum/anchor'
// import { Connection } from '@solana/web3.js'
// import { Program } from '@project-serum/anchor'

import { Keypair } from '@solana/web3.js'
import { BN } from 'bn.js'

// import { Puppet } from './target/types/puppet.ts'
// import { PuppetMaster } from './target/types/puppet_master'

// // required for way 01 and 02: setting up anchor provider using a .env file
import dotenv from 'dotenv';
dotenv.config();


async function main() {

  let data_to_put = 7;

  // // required for way 02: setting up anchor provider using a .env file
  // let connection = new Connection("http://127.0.0.1:8899");
  // let wallet = Keypair.fromSecretKey(new Uint8Array([132,34,100,14,70,70,194,21,163,48,137,212,202,213,124,235,82,94,112,143,214,133,85,249,35,233,13,68,5,35,80,23,192,232,202,113,148,148,218,203,236,244,69,109,134,227,95,205,78,16,17,135,87,129,201,132,213,18,94,147,114,197,246,108]))

  const provider = anchor.AnchorProvider.local();                     // way 01 : local solana config get
  // const provider = anchor.AnchorProvider.env();                    // way 02 : using a .env file
  // const provider = new anchor.AnchorProvider(connection, wallet )  // way 03`: Hardcoding the details for provider here itself

  console.log("🚀 ~ file: client_invoker.mjs ~ line 12 ~ main ~ provider connection", provider.connection._rpcEndpoint);
  console.log("🚀 ~ file: client_invoker.mjs ~ line 12 ~ main ~ provider wallet", provider.wallet.publicKey.toBase58());

  anchor.setProvider(provider)


  const puppetProgram = anchor.workspace.Puppet
  // console.log("🚀 ~ file: client_invoker.mjs ~ line 17 ~ main ~ puppetProgram", puppetProgram);
  console.log("🚀 ~ file: client_invoker.mjs ~ line 17 ~ main ~ puppetProgram programId", puppetProgram.programId.toBase58());
  console.log("🚀 ~ file: client_invoker.mjs ~ line 17 ~ main ~ puppetProgram rpc", puppetProgram.rpc);

  const puppetMasterProgram = anchor.workspace.PuppetMaster
  // console.log("🚀 ~ file: client_invoker.mjs ~ line 32 ~ main ~ puppetMasterProgram", puppetMasterProgram);
  console.log("🚀 ~ file: client_invoker.mjs ~ line 32 ~ main ~ puppetMasterProgram programId", puppetMasterProgram.programId.toBase58());
  console.log("🚀 ~ file: client_invoker.mjs ~ line 32 ~ main ~ puppetMasterProgram rpc", puppetMasterProgram.rpc);


  const puppetKeypair = Keypair.generate()
  console.log("🚀 ~ file: client_invoker.mjs ~ line 22 ~ main ~ puppetKeypair", puppetKeypair.publicKey.toBase58());

  const authorityKeypair = Keypair.generate()
  console.log("🚀 ~ file: client_invoker.mjs ~ line 24 ~ main ~ authorityKeypair", authorityKeypair.publicKey.toBase58());

  console.log("🚀 ~ file: client_invoker.mjs ~ line 41 ~ main ~ provider.wallet.payer", provider.wallet.payer.publicKey.toBase58());
  console.log("🚀 ~ file: client_invoker.mjs ~ line 41 ~ main ~ typeof(provider.wallet.payer)", typeof (provider.wallet.payer));


  // Invoking Initialize Endpoint
  let transaction_id = await puppetProgram.methods
    .initialize(authorityKeypair.publicKey)   //camelCase
    .accounts({
      puppet: puppetKeypair.publicKey,
      user: provider.wallet.publicKey,
    })
    .signers([puppetKeypair])
    .rpc()
  console.log("🚀 ~ file: client_invoker.mjs ~ line 42 ~ main ~ initialize transaction_id: ", transaction_id);

  // // Invoking set_data Endpoint
  // let some_u64_data = new BN(data_to_put);
  // transaction_id = await puppetProgram.methods
  //   .setData(some_u64_data)                      //camelCase
  //   .accounts({
  //     puppet: puppetKeypair.publicKey,
  //     authority: authorityKeypair.publicKey,
  //   })
  //   .signers([authorityKeypair])
  //   .rpc()
  // console.log("🚀 ~ file: client_invoker.mjs ~ line 64 ~ main ~ setData transaction_id", transaction_id);

  async function cpi() {
    // await puppetProgram.methods
    //   .initialize(authorityKeypair.publicKey)
    //   .accounts({
    //     puppet: puppetKeypair.publicKey,
    //     user: provider.wallet.publicKey,
    //   })
    //   .signers([puppetKeypair])
    //   .rpc()


    // puppet master contract endpoint invoker
    // If above code for initializing is commented:
    // AnchorError: AnchorError caused by account: puppet. Error Code: AccountNotInitialized. Error Number: 3012. Error Message: The program expected this account to be already initialized.
    let some_u64_data = new BN(data_to_put);
    let return_value = await puppetMasterProgram.methods
      .pullStrings(some_u64_data)
      .accounts({
        puppetProgram: puppetProgram.programId,
        puppet: puppetKeypair.publicKey,
        authority: authorityKeypair.publicKey,
      })
      .signers([authorityKeypair])
      .rpc()
    console.log("🚀 ~ file: client_invoker.mjs ~ line 102 ~ cpi ~ return_value", return_value);

    let fetch_data_from_account = (await puppetProgram.account.data.fetch(puppetKeypair.publicKey)).data.toNumber()
    console.log("🚀 ~ file: client_invoker.mjs ~ line 105 ~ cpi ~ fetch_data_from_account", fetch_data_from_account);
  }

  await cpi()


}

main()


// Privilege Extension
// CPIs extend the privileges of the caller to the callee. 
// The puppet account was passed as a mutable account to the puppet-master 
// but it was still mutable in the puppet program as well

// Privilege extension is convenient but also dangerous. 
// If a CPI is unintentionally made to a malicious program, 
// this program has the same privileges as the caller. 
// Anchor protects you from CPIs to malicious programs with two measures. 
// First, the Program<'info, T> type checks that the given account is the expected program T. 
// Should you ever forget to use the Program type, the automatically generated cpi function 
// (in the previous example this was puppet::cpi::set_data) also checks that the cpi_program argument equals the expected program.