import * as anchor from "@project-serum/anchor";
import { Program, AnchorError } from "@project-serum/anchor";
import {
  Keypair, SystemProgram, LAMPORTS_PER_SOL
} from "@solana/web3.js";
import { Fighting } from "../../target/types/fighting";
import { Gpass } from "../../target/types/gpass";
import * as assert from "assert";
import * as utils from "../utils";
import { FightingTestFixture, prepareFightingTestFixture } from "./fixture";

describe("Fighting functional tests", () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  const fighting = anchor.workspace.Fighting as Program<Fighting>;
  const gpass = anchor.workspace.Gpass as Program<Gpass>;

  let fixture: FightingTestFixture = null;
  before(async () => {
    fixture = await prepareFightingTestFixture(fighting, gpass);
    await fighting.methods.initialize(fixture.updateAuth.publicKey, new anchor.BN(100))
      .accounts({
        admin: fixture.admin.publicKey,
        fightingSettings: fixture.fighting.settings.publicKey,
        gpassBurnAuth: fixture.fighting.gpassBurnAuth,
        gpassInfo: fixture.fighting.gpassInfo.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .signers([fixture.admin, fixture.fighting.settings])
      .rpc();
  });

  // userId.toArrayLike(Buffer, "le", 8),

});
