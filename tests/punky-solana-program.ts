import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PunkySolanaProgram } from "../target/types/punky_solana_program";
import { PublicKey } from "@solana/web3.js";
import { assert } from "chai";

describe("punky-solana-program", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace
    .PunkySolanaProgram as Program<PunkySolanaProgram>;
  const user = provider.wallet;

  // Add helper function to derive PDA
  const deriveGameAccountPDA = async (userPubkey: PublicKey) => {
    const [pda] = PublicKey.findProgramAddressSync(
      [Buffer.from("game_account"), userPubkey.toBuffer()],
      program.programId
    );
    return pda;
  };

  it("Is initialized!", async () => {
    // Derive the game account PDA for this user
    const gameAccount = await deriveGameAccountPDA(user.publicKey);

    const tx = await program.methods
      .initialize()
      .accounts({
        signer: user.publicKey,
        gameAccount,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    // console.log("Your transaction signature", tx);

    // Fetch and verify the account data
    const accountData = await program.account.gameAccount.fetch(gameAccount);
    assert.equal(accountData.health, 500);
    assert.equal(accountData.fitness, 500);
    assert.equal(accountData.loyalty, 500);
    assert.equal(accountData.balance.toNumber(), 1000);
  });

  it("Fails to initialize an already initialized account", async () => {
    const gameAccount = await deriveGameAccountPDA(user.publicKey);

    try {
      // Attempt to initialize again
      await program.methods
        .initialize()
        .accounts({
          signer: user.publicKey,
          gameAccount,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .rpc();

      assert.fail("Should have thrown an error");
    } catch (error) {
      assert.include(
        error.message,
        "custom program error: 0x0"
      );
    }
  });
});
