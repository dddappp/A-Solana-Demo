import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { ASolanaDemo } from "../target/types/a_solana_demo";
import { Connection, Keypair, LAMPORTS_PER_SOL } from '@solana/web3.js';

describe("a-solana-demo", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.ASolanaDemo as Program<ASolanaDemo>;

  let provider = anchor.AnchorProvider.env();
  const authority = provider.wallet.publicKey;

  it("Is initialized!", async () => {

    // ----------------------------------------------------------
    // Human's wallet
    const human = Keypair.generate();
    const connection = anchor.AnchorProvider.env().connection;
    // Request sol airdrop (for human to be able to do transactions)
    await requestAirdrop(connection, human.publicKey, LAMPORTS_PER_SOL)

    // ----------------------------------------------------------
    // Tag
    //
    const tagName = "hello";
    let [tag] = anchor.web3.PublicKey.findProgramAddressSync(
        [
            Buffer.from("Tag"),
            Buffer.from(tagName)
        ],
        program.programId
    );

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

    // ----------------------------------------------------------
    // Article
    //
    const articleId = new anchor.BN(1, 128);
    let [article] = anchor.web3.PublicKey.findProgramAddressSync(
        [
            Buffer.from("Article"),
            articleId.toBuffer("le", 16),
        ],
        program.programId
    );

    const tx_3 = await program.methods.createArticle(
        articleId,
        "hello",
        "world",
        human.publicKey,
    ).accounts(
        {
           article,
           authority: human.publicKey, //authority,
           systemProgram: anchor.web3.SystemProgram.programId,
        }
    )
    .signers(
        [human]
    )
    .rpc();
    console.log("Your transaction signature", tx_3);

    //
    // update
    const tx_4 = await program.methods.updateArticle(
        "hello",
        "world!",
        human.publicKey,
    ).accounts(
        {
           article,
           authority: human.publicKey, //authority,
           systemProgram: anchor.web3.SystemProgram.programId,
        }
    )
    .signers(
        [human]
    )
    .rpc();
    console.log("Your transaction signature", tx_4);
    //

    // Fetch the state struct from the network.
    const accountState_2 = await program.account.article.fetch(article);
    console.log("account state: ", accountState_2);

  });
});


async function requestAirdrop(connection: Connection, address: anchor.web3.PublicKey, lamports: number) {
    const tx = await connection.requestAirdrop(address, lamports);
    await connection.confirmTransaction(tx);
}