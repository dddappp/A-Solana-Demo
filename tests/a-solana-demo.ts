import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { ASolanaDemo } from "../target/types/a_solana_demo";
import { Connection, Keypair, LAMPORTS_PER_SOL } from '@solana/web3.js';

describe("a-solana-demo", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.ASolanaDemo as Program<ASolanaDemo>;

  const authority = provider.wallet.publicKey;

  it("Is initialized!", async () => {

    // ----------------------------------------------------------
    // Human's wallet
    const human = Keypair.generate();
    const connection = anchor.AnchorProvider.env().connection;
    // Request sol airdrop (for human to be able to do transactions)
    await requestAirdrop(connection, human.publicKey, LAMPORTS_PER_SOL)
    // ----------------------------------------------------------

    const tagName = "hello";
    let [tag] = anchor.web3.PublicKey.findProgramAddressSync(
        [
            Buffer.from("Tag"),
            Buffer.from(tagName)
        ],
        program.programId
    );

    // Add your test here.
    const tx = await program.methods.createTag(
        tagName,
        "world",
    ).accounts(
        {
           tag,
           authority: human.publicKey, //authority,
           systemProgram: anchor.web3.SystemProgram.programId,
        }
    )
    .signers(
        [human]
    )
    .rpc();
    console.log("Your transaction signature", tx);

    //
    // update
    const tx_2 = await program.methods.updateTag(
        tagName,
        "world!",
    ).accounts(
        {
           tag,
           authority: human.publicKey, //authority,
           systemProgram: anchor.web3.SystemProgram.programId,
        }
    )
    .signers(
        [human]
    )
    .rpc();
    console.log("Your transaction signature", tx_2);
    //

    // Fetch the state struct from the network.
    const accountState = await program.account.tag.fetch(tag);
    console.log("account state: ", accountState);

  });
});


async function requestAirdrop(connection: Connection, address: anchor.web3.PublicKey, lamports: number) {
    const tx = await connection.requestAirdrop(address, lamports);
    await connection.confirmTransaction(tx);
}