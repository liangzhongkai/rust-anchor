import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AnchorTest } from "../target/types/anchor_test";

describe("anchor-test", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.anchorTest as Program<AnchorTest>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);

    // Fetch and print transaction logs
    const connection = program.provider.connection;
    const confirmedTx = await connection.getTransaction(tx, {
      commitment: "confirmed",
      maxSupportedTransactionVersion: 0,
    });
    if (confirmedTx?.meta?.logMessages) {
      console.log("Transaction logs:");
      confirmedTx.meta.logMessages.forEach((log) => console.log(log));
    }
  });
});
