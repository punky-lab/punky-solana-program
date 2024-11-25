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

  const deriveGameAccountPDA = async (userPubkey: PublicKey) => {
    const [pda] = PublicKey.findProgramAddressSync(
      [Buffer.from("game_account"), userPubkey.toBuffer()],
      program.programId
    );
    return pda;
  };

  it("Is initialized!", async () => {
    const gameAccount = await deriveGameAccountPDA(user.publicKey);

    const tx = await program.methods
      .initialize()
      .accounts({
        signer: user.publicKey,
        gameAccount,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    // Fetch and verify the account data
    const accountData = await program.account.gameAccount.fetch(gameAccount);
    assert.equal(accountData.happiness, 500);
    assert.equal(accountData.fitness, 500);
    assert.equal(accountData.loyalty, 500);
    assert.equal(accountData.balance.toNumber(), 1000);
    assert.equal(accountData.initialized, true);
  });

  it("Fails to initialize an already initialized account", async () => {
    const gameAccount = await deriveGameAccountPDA(user.publicKey);

    try {
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

  it("Can pet the pet", async () => {
    const gameAccount = await deriveGameAccountPDA(user.publicKey);

    await program.methods
      .petPet()
      .accounts({
        signer: user.publicKey,
        gameAccount,
      })
      .rpc();

    const accountData = await program.account.gameAccount.fetch(gameAccount);
    assert.equal(accountData.fitness, 490); // 500 - 10
    assert.equal(accountData.happiness, 505); // 500 + 5
    assert.equal(accountData.loyalty, 505); // 500 + 5
    assert.equal(accountData.balance.toNumber(), 1001); // 1000 + 1
  });

  it("Can feed the pet", async () => {
    const gameAccount = await deriveGameAccountPDA(user.publicKey);

    await program.methods
      .feedPet()
      .accounts({
        signer: user.publicKey,
        gameAccount,
      })
      .rpc();

    const accountData = await program.account.gameAccount.fetch(gameAccount);
    assert.equal(accountData.fitness, 500);
    assert.equal(accountData.happiness, 510);
    assert.equal(accountData.loyalty, 510);
    assert.equal(accountData.balance.toNumber(), 1000);
  });
});
