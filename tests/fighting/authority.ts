import * as anchor from "@project-serum/anchor";
import { Program, AnchorError } from "@project-serum/anchor";
import {
  Keypair, SystemProgram, LAMPORTS_PER_SOL
} from "@solana/web3.js";
import { Fighting } from "../../target/types/fighting";
import { Gpass } from "../../target/types/gpass";
import { RewardDistribution } from "../../target/types/reward_distribution";
import { Freezing } from "../../target/types/freezing";
import * as assert from "assert";
import * as utils from "../utils";
import { FightingTestFixture, prepareFightingTestFixture } from "./fixture";

describe("Fighting authority tests", () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  const fighting = anchor.workspace.Fighting as Program<Fighting>;
  const gpass = anchor.workspace.Gpass as Program<Gpass>;
  const rewardDistribution = anchor.workspace.RewardDistribution as Program<RewardDistribution>;
  const freezing = anchor.workspace.Freezing as Program<Freezing>;

  let fixture: FightingTestFixture = null;
  before(async () => {
    fixture = await prepareFightingTestFixture(fighting, gpass, rewardDistribution, freezing);
    await fighting.methods.initialize(fixture.updateAuth.publicKey, fixture.fighting.validator.publicKey, new anchor.BN(100), 100, 200, 8)
      .accounts({
        admin: fixture.admin.publicKey,
        fightingSettings: fixture.fighting.settings.publicKey,
        gpassBurnAuth: fixture.fighting.gpassBurnAuth,
        gpassInfo: fixture.fighting.gpassInfo.publicKey,
        rewardDistributionInfo: fixture.fighting.rewardDistributionInfo.publicKey,
        rewardTransferAuth: fixture.fighting.transferAuth,
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

  it("update reward coefficient with invalid authority", async () => {
    const invalidUpdateAuth = Keypair.generate();
    const newRewardCoefficient = 200;
    await utils.airdropSol(fighting.provider.connection, invalidUpdateAuth.publicKey, 1 * LAMPORTS_PER_SOL);
    await assert.rejects(fighting.methods.updateRewardCoefficient(newRewardCoefficient)
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

  it("update reward coefficient", async () => {
    const newRewardCoefficient = 1230000;
    await fighting.methods.updateRewardCoefficient(newRewardCoefficient)
      .accounts({
        authority: newUpdAuthority.publicKey,
        fightingSettings: fixture.fighting.settings.publicKey,
      })
      .signers([newUpdAuthority])
      .rpc();

    const fightingSettingsData = await fighting.account.fightingSettings.fetch(fixture.fighting.settings.publicKey);
    assert.equal(fightingSettingsData.rewardCoefficient, newRewardCoefficient);
  });

  it("update gpass daily reward coefficient with invalid authority", async () => {
    const invalidUpdateAuth = Keypair.generate();
    const newRewardCoefficient = 200;
    await utils.airdropSol(fighting.provider.connection, invalidUpdateAuth.publicKey, 1 * LAMPORTS_PER_SOL);
    await assert.rejects(fighting.methods.updateGpassDailyRewardCoefficient(newRewardCoefficient)
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

  it("update gpass daily reward coefficient", async () => {
    const newRewardCoefficient = 1234500;
    await fighting.methods.updateGpassDailyRewardCoefficient(newRewardCoefficient)
      .accounts({
        authority: newUpdAuthority.publicKey,
        fightingSettings: fixture.fighting.settings.publicKey,
      })
      .signers([newUpdAuthority])
      .rpc();

    const fightingSettingsData = await fighting.account.fightingSettings.fetch(fixture.fighting.settings.publicKey);
    assert.equal(fightingSettingsData.gpassDailyRewardCoefficient, newRewardCoefficient);
  });

  it("update royalty with invalid authority", async () => {
    const invalidUpdateAuth = Keypair.generate();
    const newRoyalty = 80;
    await utils.airdropSol(fighting.provider.connection, invalidUpdateAuth.publicKey, 1 * LAMPORTS_PER_SOL);
    await assert.rejects(fighting.methods.updateGpassDailyRewardCoefficient(newRoyalty)
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

  it("update royalty", async () => {
    const newRoyalty = 87;
    await fighting.methods.updateRoyalty(newRoyalty)
      .accounts({
        authority: newUpdAuthority.publicKey,
        fightingSettings: fixture.fighting.settings.publicKey,
      })
      .signers([newUpdAuthority])
      .rpc();

    const fightingSettingsData = await fighting.account.fightingSettings.fetch(fixture.fighting.settings.publicKey);
    assert.equal(fightingSettingsData.royalty, newRoyalty);
  });

  it("update validator with invalid authority", async () => {
    const invalidUpdateAuth = Keypair.generate();
    const newValidator = Keypair.generate();
    await utils.airdropSol(fighting.provider.connection, invalidUpdateAuth.publicKey, 1 * LAMPORTS_PER_SOL);
    await assert.rejects(fighting.methods.updateValidator(newValidator.publicKey)
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

  it("update validator", async () => {
    const newValidator = Keypair.generate();
    await fighting.methods.updateValidator(newValidator.publicKey)
      .accounts({
        authority: newUpdAuthority.publicKey,
        fightingSettings: fixture.fighting.settings.publicKey,
      })
      .signers([newUpdAuthority])
      .rpc();

    const fightingSettingsData = await fighting.account.fightingSettings.fetch(fixture.fighting.settings.publicKey);
    assert.ok(fightingSettingsData.validator.equals(newValidator.publicKey));
  });
});
