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

describe("Fighting authority tests", () => {
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

  const newAdmin = Keypair.generate();
  it("update admin with invalid admin", async () => {
    const invalidAdmin = Keypair.generate();
    await utils.airdropSol(fighting.provider.connection, invalidAdmin.publicKey, 1 * LAMPORTS_PER_SOL);
    await assert.rejects(fighting.methods.updateAdmin(newAdmin.publicKey)
      .accounts({
        authority: invalidAdmin.publicKey,
        fightingSettings: fixture.fighting.settings.publicKey,
      })
      .signers([invalidAdmin])
      .rpc(),
      (e: AnchorError) => {
        assert.ok(e.error !== undefined);
        assert.strictEqual(e.error.errorCode.code, "AccessDenied");
        assert.strictEqual(e.error.errorCode.number, 6000);
        assert.strictEqual(e.error.errorMessage, "Access denied");
        return true;
      }
    );
  });

  it("update admin", async () => {
    await fighting.methods.updateAdmin(newAdmin.publicKey)
      .accounts({
        authority: fixture.admin.publicKey,
        fightingSettings: fixture.fighting.settings.publicKey,
      })
      .signers([fixture.admin])
      .rpc();

    const fightingSettingsData = await fighting.account.fightingSettings.fetch(fixture.fighting.settings.publicKey);
    assert.ok(fightingSettingsData.admin.equals(newAdmin.publicKey));
  });

  const newUpdAuthority = Keypair.generate();
  it("set update authority with invalid admin", async () => {
    const invalidUpdateAuth = Keypair.generate();
    await utils.airdropSol(fighting.provider.connection, invalidUpdateAuth.publicKey, 1 * LAMPORTS_PER_SOL);
    await assert.rejects(fighting.methods.setUpdateAuthority(newUpdAuthority.publicKey)
      .accounts({
        authority: invalidUpdateAuth.publicKey,
        fightingSettings: fixture.fighting.settings.publicKey,
      })
      .signers([invalidUpdateAuth])
      .rpc(),
      (e: AnchorError) => {
        assert.ok(e.error !== undefined);
        assert.strictEqual(e.error.errorCode.code, "AccessDenied");
        assert.strictEqual(e.error.errorCode.number, 6000);
        assert.strictEqual(e.error.errorMessage, "Access denied");
        return true;
      }
    );
  });

  it("set update authority", async () => {
    await fighting.methods.setUpdateAuthority(newUpdAuthority.publicKey)
      .accounts({
        authority: newAdmin.publicKey,
        fightingSettings: fixture.fighting.settings.publicKey,
      })
      .signers([newAdmin])
      .rpc();

    const fightingSettingsData = await fighting.account.fightingSettings.fetch(fixture.fighting.settings.publicKey);
    assert.ok(fightingSettingsData.updateAuth.equals(newUpdAuthority.publicKey));
  });

  it("update afk timeout with invalid authority", async () => {
    const invalidUpdateAuth = Keypair.generate();
    const newAfkTimeout = 200;
    await utils.airdropSol(fighting.provider.connection, invalidUpdateAuth.publicKey, 1 * LAMPORTS_PER_SOL);
    await assert.rejects(fighting.methods.updateAfkTimeout(new anchor.BN(newAfkTimeout))
      .accounts({
        authority: invalidUpdateAuth.publicKey,
        fightingSettings: fixture.fighting.settings.publicKey,
      })
      .signers([invalidUpdateAuth])
      .rpc(),
      (e: AnchorError) => {
        assert.ok(e.error !== undefined);
        assert.strictEqual(e.error.errorCode.code, "AccessDenied");
        assert.strictEqual(e.error.errorCode.number, 6000);
        assert.strictEqual(e.error.errorMessage, "Access denied");
        return true;
      }
    );
  });

  it("update afk timeout", async () => {
    const newAfkTimeout = 80000;
    await fighting.methods.updateAfkTimeout(new anchor.BN(newAfkTimeout))
      .accounts({
        authority: newUpdAuthority.publicKey,
        fightingSettings: fixture.fighting.settings.publicKey,
      })
      .signers([newUpdAuthority])
      .rpc();

    const fightingSettingsData = await fighting.account.fightingSettings.fetch(fixture.fighting.settings.publicKey);
    assert.equal(fightingSettingsData.afkTimeout.toNumber(), newAfkTimeout);
  });
});
