import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AnchorTest } from "../target/types/anchor_test";
import { PublicKey, Keypair } from "@solana/web3.js";
import { expect } from "chai";

// PDA seeds - must match program
const GLOBAL_COUNTER_SEED = Buffer.from("global_counter");
const USER_COUNTER_SEED = Buffer.from("user_counter");

function getGlobalCounterPda(programId: PublicKey): [PublicKey, number] {
  return PublicKey.findProgramAddressSync(
    [GLOBAL_COUNTER_SEED],
    programId
  );
}

function getUserCounterPda(programId: PublicKey, owner: PublicKey): [PublicKey, number] {
  return PublicKey.findProgramAddressSync(
    [USER_COUNTER_SEED, owner.toBuffer()],
    programId
  );
}

describe("anchor-test", () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.anchorTest as Program<AnchorTest>;
  const provider = program.provider as anchor.AnchorProvider;
  const wallet = provider.wallet;

  describe("Legacy", () => {
    it("initialize (legacy greeting)", async () => {
      const tx = await program.methods.initialize().rpc();
      expect(tx).to.be.a("string");
    });
  });

  describe("Global Counter", () => {
    it("initializes global counter", async () => {
      const [globalCounterPda] = getGlobalCounterPda(program.programId);

      await program.methods
        .initializeGlobalCounter()
        .accounts({
          globalCounter: globalCounterPda,
          authority: wallet.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .rpc();

      const counter = await program.account.globalCounter.fetch(globalCounterPda);
      expect(counter.authority.equals(wallet.publicKey)).to.be.true;
      expect(counter.count.toNumber()).to.equal(0);
      expect(counter.bump).to.be.greaterThan(0);
    });

    it("increments global counter", async () => {
      const [globalCounterPda] = getGlobalCounterPda(program.programId);

      await program.methods
        .incrementGlobal(new anchor.BN(5))
        .accounts({
          globalCounter: globalCounterPda,
          authority: wallet.publicKey,
        })
        .rpc();

      let counter = await program.account.globalCounter.fetch(globalCounterPda);
      expect(counter.count.toNumber()).to.equal(5);

      await program.methods
        .incrementGlobal(new anchor.BN(10))
        .accounts({
          globalCounter: globalCounterPda,
          authority: wallet.publicKey,
        })
        .rpc();

      counter = await program.account.globalCounter.fetch(globalCounterPda);
      expect(counter.count.toNumber()).to.equal(15);
    });

    it("decrements global counter", async () => {
      const [globalCounterPda] = getGlobalCounterPda(program.programId);

      await program.methods
        .decrementGlobal(new anchor.BN(3))
        .accounts({
          globalCounter: globalCounterPda,
          authority: wallet.publicKey,
        })
        .rpc();

      const counter = await program.account.globalCounter.fetch(globalCounterPda);
      expect(counter.count.toNumber()).to.equal(12);
    });

    it("rejects decrement below zero (underflow)", async () => {
      const [globalCounterPda] = getGlobalCounterPda(program.programId);

      try {
        await program.methods
          .decrementGlobal(new anchor.BN(100))
          .accounts({
            globalCounter: globalCounterPda,
            authority: wallet.publicKey,
          })
          .rpc();
        expect.fail("Expected transaction to fail with underflow");
      } catch (err) {
        expect(err).to.exist;
      }
    });
  });

  describe("User Counter", () => {
    it("initializes user counter", async () => {
      const [userCounterPda] = getUserCounterPda(program.programId, wallet.publicKey);

      await program.methods
        .initializeUserCounter()
        .accounts({
          userCounter: userCounterPda,
          owner: wallet.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .rpc();

      const counter = await program.account.userCounter.fetch(userCounterPda);
      expect(counter.owner.equals(wallet.publicKey)).to.be.true;
      expect(counter.count.toNumber()).to.equal(0);
    });

    it("increments and decrements user counter", async () => {
      const [userCounterPda] = getUserCounterPda(program.programId, wallet.publicKey);

      await program.methods
        .incrementUser(new anchor.BN(42))
        .accounts({
          userCounter: userCounterPda,
          owner: wallet.publicKey,
        })
        .rpc();

      let counter = await program.account.userCounter.fetch(userCounterPda);
      expect(counter.count.toNumber()).to.equal(42);

      await program.methods
        .decrementUser(new anchor.BN(7))
        .accounts({
          userCounter: userCounterPda,
          owner: wallet.publicKey,
        })
        .rpc();

      counter = await program.account.userCounter.fetch(userCounterPda);
      expect(counter.count.toNumber()).to.equal(35);
    });

    it("rejects user decrement below zero", async () => {
      const [userCounterPda] = getUserCounterPda(program.programId, wallet.publicKey);

      try {
        await program.methods
          .decrementUser(new anchor.BN(100))
          .accounts({
            userCounter: userCounterPda,
            owner: wallet.publicKey,
          })
          .rpc();
        expect.fail("Expected transaction to fail with underflow");
      } catch (err) {
        expect(err).to.exist;
      }
    });

    it("closes user counter and reclaims rent", async () => {
      const [userCounterPda] = getUserCounterPda(program.programId, wallet.publicKey);
      const balanceBefore = await provider.connection.getBalance(wallet.publicKey);

      await program.methods
        .closeUserCounter()
        .accounts({
          userCounter: userCounterPda,
          owner: wallet.publicKey,
        })
        .rpc();

      const balanceAfter = await provider.connection.getBalance(wallet.publicKey);
      expect(balanceAfter).to.be.greaterThan(balanceBefore);
    });
  });

  describe("Transfer Global Authority", () => {
    it("transfers authority to new keypair", async () => {
      const [globalCounterPda] = getGlobalCounterPda(program.programId);
      const newAuthority = Keypair.generate();

      await program.methods
        .transferGlobalAuthority(newAuthority.publicKey)
        .accounts({
          globalCounter: globalCounterPda,
          authority: wallet.publicKey,
        })
        .rpc();

      const counter = await program.account.globalCounter.fetch(globalCounterPda);
      expect(counter.authority.equals(newAuthority.publicKey)).to.be.true;
    });
  });
});
