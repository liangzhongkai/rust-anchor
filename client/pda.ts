/**
 * PDA derivation helpers for anchor-test Counter program.
 * Use these to derive account addresses on the client side.
 */
import { PublicKey } from "@solana/web3.js";

export const PROGRAM_ID = new PublicKey("3BxPymFpACUfcSpNrW813gy9EXQMZ99GA9sGCoehxZ8m");

const GLOBAL_COUNTER_SEED = Buffer.from("global_counter");
const USER_COUNTER_SEED = Buffer.from("user_counter");

/**
 * Derive the global counter PDA. Single account per program.
 */
export function getGlobalCounterPda(programId: PublicKey = PROGRAM_ID): [PublicKey, number] {
  return PublicKey.findProgramAddressSync([GLOBAL_COUNTER_SEED], programId);
}

/**
 * Derive a user's counter PDA. One account per user (owner).
 */
export function getUserCounterPda(
  owner: PublicKey,
  programId: PublicKey = PROGRAM_ID
): [PublicKey, number] {
  return PublicKey.findProgramAddressSync(
    [USER_COUNTER_SEED, owner.toBuffer()],
    programId
  );
}
