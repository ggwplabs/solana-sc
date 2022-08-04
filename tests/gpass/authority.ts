import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import {
  Keypair, SystemProgram, LAMPORTS_PER_SOL, PublicKey
} from "@solana/web3.js";
import { Gpass } from "../../target/types/gpass";
import * as assert from "assert";
import * as utils from "../utils";

describe("GPASS initialize tests", () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.Gpass as Program<Gpass>;

  const admin = Keypair.generate();
  const updateAuth = Keypair.generate();
  const mintersPK = [];
  const minters = [Keypair.generate()].forEach((item) => {
    mintersPK.push(item.publicKey);
  });
  const burnersPK = [];
  const burners = [Keypair.generate(), Keypair.generate()].forEach((item) => {
    burnersPK.push(item.publicKey);
  });
  const burnPeriod = 100;

  const settings = Keypair.generate();
  before(async () => {
    await utils.airdropSol(program.provider.connection, admin.publicKey, 100 * LAMPORTS_PER_SOL);
    await program.methods.initialize(
      new anchor.BN(burnPeriod),
      updateAuth.publicKey,
      mintersPK,
      burnersPK
    )
      .accounts({
        admin: admin.publicKey,
        settings: settings.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .signers([admin, settings])
      .rpc();

    const settingsData = await program.account.settings.fetch(settings.publicKey);
    assert.ok(settingsData.admin.equals(admin.publicKey));
    assert.ok(settingsData.updateAuth.equals(updateAuth.publicKey));
    assert.equal(settingsData.burnPeriod.toNumber(), burnPeriod);
    assert.deepStrictEqual(settingsData.minters, mintersPK);
    assert.deepStrictEqual(settingsData.burners, burnersPK);
  });

});
