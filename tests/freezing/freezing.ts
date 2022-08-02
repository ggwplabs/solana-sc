import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Freezing } from "../../target/types/freezing";

describe("staking-sc", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Freezing as Program<Freezing>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
