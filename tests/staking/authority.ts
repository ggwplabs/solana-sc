import * as anchor from "@project-serum/anchor";
import { AnchorError, Program } from "@project-serum/anchor";
import { Keypair, SystemProgram, LAMPORTS_PER_SOL } from "@solana/web3.js";
import { Staking } from "../../target/types/staking";
import * as assert from "assert";
import * as utils from "../utils";
import { StakingTestFixture, prepareStakingTestFixture } from "./fixture";

describe("Staking authority tests", () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.Staking as Program<Staking>;

  const epochPeriodDays = 45;
  const minStakeAmount = 3000_000_000_000;
  const holdPeriodDays = 30;
  const holdRoyalty = 15;
  const royalty = 8;
  const aprStart = 50;
  const aprStep = 1;
  const aprEnd = 5;

  let fixture: StakingTestFixture = null;
  before(async () => {
    fixture = await prepareStakingTestFixture(program);
    await program.methods.initialize(
      fixture.updateAuth.publicKey,
      epochPeriodDays,
      new anchor.BN(minStakeAmount),
      holdPeriodDays,
      holdRoyalty,
      royalty,
      aprStart,
      aprStep,
      aprEnd,
    )
      .accounts({
        admin: fixture.admin.publicKey,
        stakingInfo: fixture.staking.info.publicKey,
        ggwpToken: fixture.staking.ggwpToken,
        accumulativeFund: fixture.staking.accumulativeFund,
        stakingFund: fixture.staking.stakingFund,
        stakingFundAuth: fixture.staking.stakingFundAuth,
        treasury: fixture.staking.treasury,
        treasuryAuth: fixture.staking.treasuryAuth,
        systemProgram: SystemProgram.programId,
      })
      .signers([fixture.admin, fixture.staking.info])
      .rpc();

    const stakingInfoData = await program.account.stakingInfo.fetch(fixture.staking.info.publicKey);
    assert.ok(stakingInfoData.admin.equals(fixture.admin.publicKey));
    assert.ok(stakingInfoData.updateAuth.equals(fixture.updateAuth.publicKey));
    assert.ok(stakingInfoData.ggwpToken.equals(fixture.staking.ggwpToken));
    assert.ok(stakingInfoData.stakingFund.equals(fixture.staking.stakingFund));
    assert.ok(stakingInfoData.treasury.equals(fixture.staking.treasury));
    assert.equal(stakingInfoData.epochPeriodDays, epochPeriodDays);
    assert.equal(stakingInfoData.minStakeAmount.toNumber(), minStakeAmount);
    assert.equal(stakingInfoData.holdPeriodDays, holdPeriodDays);
    assert.equal(stakingInfoData.holdRoyalty, holdRoyalty);
    assert.equal(stakingInfoData.royalty, royalty);
    assert.equal(stakingInfoData.aprStart, aprStart);
    assert.equal(stakingInfoData.aprStep, aprStep);
    assert.equal(stakingInfoData.aprEnd, aprEnd);
    assert.equal(stakingInfoData.totalStaked, 0);
  });

  const newAdmin = Keypair.generate();
  it("Update admin with invalid admin", async () => {
    const invalidAuthority = Keypair.generate();
    await utils.airdropSol(program.provider.connection, invalidAuthority.publicKey, 1 * LAMPORTS_PER_SOL);
    await assert.rejects(program.methods
      .updateAdmin(newAdmin.publicKey)
      .accounts({
        authority: invalidAuthority.publicKey,
        stakingInfo:
          fixture.staking.info.publicKey
      })
      .signers([invalidAuthority])
      .rpc(),
      (e: AnchorError) => {
        assert.ok(e.error !== undefined);
        assert.strictEqual(e.error.errorCode.code, "AccessDenied");
        assert.strictEqual(e.error.errorCode.number, 6000);
        assert.strictEqual(e.error.errorMessage, "Access denied");
        return true;
      });
  });

  it("Update admin", async () => {
    await program.methods
      .updateAdmin(newAdmin.publicKey)
      .accounts({
        authority: fixture.admin.publicKey,
        stakingInfo:
          fixture.staking.info.publicKey
      })
      .signers([fixture.admin])
      .rpc();

    const stakingInfoData = await program.account.stakingInfo.fetch(fixture.staking.info.publicKey);
    assert.ok(stakingInfoData.admin.equals(newAdmin.publicKey));
  });

  const newUpdateAuth = Keypair.generate();
  it("Set update authority with invalid admin", async () => {
    await assert.rejects(program.methods
      .setUpdateAuthority(newUpdateAuth.publicKey)
      .accounts({
        authority: fixture.admin.publicKey,
        stakingInfo:
          fixture.staking.info.publicKey
      })
      .signers([fixture.admin])
      .rpc(),
      (e: AnchorError) => {
        assert.ok(e.error !== undefined);
        assert.strictEqual(e.error.errorCode.code, "AccessDenied");
        assert.strictEqual(e.error.errorCode.number, 6000);
        assert.strictEqual(e.error.errorMessage, "Access denied");
        return true;
      });
  });

  it("Set update authority", async () => {
    await program.methods
      .setUpdateAuthority(newUpdateAuth.publicKey)
      .accounts({
        authority: newAdmin.publicKey,
        stakingInfo:
          fixture.staking.info.publicKey
      })
      .signers([newAdmin])
      .rpc();

    const stakingInfoData = await program.account.stakingInfo.fetch(fixture.staking.info.publicKey);
    assert.ok(stakingInfoData.updateAuth.equals(newUpdateAuth.publicKey));
  });

  it("Update royalty with invalid update auth", async () => {
    await assert.rejects(program.methods
      .updateHoldRoyalty(100)
      .accounts({
        authority: fixture.updateAuth.publicKey,
        stakingInfo:
          fixture.staking.info.publicKey
      })
      .signers([fixture.updateAuth])
      .rpc(),
      (e: AnchorError) => {
        assert.ok(e.error !== undefined);
        assert.strictEqual(e.error.errorCode.code, "AccessDenied");
        assert.strictEqual(e.error.errorCode.number, 6000);
        assert.strictEqual(e.error.errorMessage, "Access denied");
        return true;
      });
  });

  it("Update royalty with invalid value", async () => {
    await assert.rejects(program.methods
      .updateHoldRoyalty(101)
      .accounts({
        authority: newUpdateAuth.publicKey,
        stakingInfo:
          fixture.staking.info.publicKey
      })
      .signers([newUpdateAuth])
      .rpc(),
      (e: AnchorError) => {
        assert.ok(e.error !== undefined);
        assert.strictEqual(e.error.errorCode.code, "InvalidHoldRoyalty");
        assert.strictEqual(e.error.errorCode.number, 6005);
        assert.strictEqual(e.error.errorMessage, "Invalid hold royalty");
        return true;
      });
    await assert.rejects(program.methods
      .updateHoldRoyalty(0)
      .accounts({
        authority: newUpdateAuth.publicKey,
        stakingInfo:
          fixture.staking.info.publicKey
      })
      .signers([newUpdateAuth])
      .rpc(),
      (e: AnchorError) => {
        assert.ok(e.error !== undefined);
        assert.strictEqual(e.error.errorCode.code, "InvalidHoldRoyalty");
        assert.strictEqual(e.error.errorCode.number, 6005);
        assert.strictEqual(e.error.errorMessage, "Invalid hold royalty");
        return true;
      });
  });

  it("Update hold royalty", async () => {
    const newRoyalty = 77;
    await program.methods
      .updateHoldRoyalty(newRoyalty)
      .accounts({
        authority: newUpdateAuth.publicKey,
        stakingInfo:
          fixture.staking.info.publicKey
      })
      .signers([newUpdateAuth])
      .rpc();

    const stakingInfoData = await program.account.stakingInfo.fetch(fixture.staking.info.publicKey);
    assert.equal(stakingInfoData.holdRoyalty, newRoyalty);
  });

  it("Update royalty with invalid update auth", async () => {
    await assert.rejects(program.methods
      .updateRoyalty(100)
      .accounts({
        authority: fixture.updateAuth.publicKey,
        stakingInfo:
          fixture.staking.info.publicKey
      })
      .signers([fixture.updateAuth])
      .rpc(),
      (e: AnchorError) => {
        assert.ok(e.error !== undefined);
        assert.strictEqual(e.error.errorCode.code, "AccessDenied");
        assert.strictEqual(e.error.errorCode.number, 6000);
        assert.strictEqual(e.error.errorMessage, "Access denied");
        return true;
      });
  });

  it("Update royalty with invalid value", async () => {
    await assert.rejects(program.methods
      .updateRoyalty(101)
      .accounts({
        authority: newUpdateAuth.publicKey,
        stakingInfo:
          fixture.staking.info.publicKey
      })
      .signers([newUpdateAuth])
      .rpc(),
      (e: AnchorError) => {
        assert.ok(e.error !== undefined);
        assert.strictEqual(e.error.errorCode.code, "InvalidRoyalty");
        assert.strictEqual(e.error.errorCode.number, 6006);
        assert.strictEqual(e.error.errorMessage, "Invalid royalty");
        return true;
      });
    await assert.rejects(program.methods
      .updateRoyalty(0)
      .accounts({
        authority: newUpdateAuth.publicKey,
        stakingInfo:
          fixture.staking.info.publicKey
      })
      .signers([newUpdateAuth])
      .rpc(),
      (e: AnchorError) => {
        assert.ok(e.error !== undefined);
        assert.strictEqual(e.error.errorCode.code, "InvalidRoyalty");
        assert.strictEqual(e.error.errorCode.number, 6006);
        assert.strictEqual(e.error.errorMessage, "Invalid royalty");
        return true;
      });
  });

  it("Update royalty", async () => {
    const newRoyalty = 23;
    await program.methods
      .updateRoyalty(newRoyalty)
      .accounts({
        authority: newUpdateAuth.publicKey,
        stakingInfo:
          fixture.staking.info.publicKey
      })
      .signers([newUpdateAuth])
      .rpc();

    const stakingInfoData = await program.account.stakingInfo.fetch(fixture.staking.info.publicKey);
    assert.equal(stakingInfoData.royalty, newRoyalty);
  });

  it("Update min stake amount with invalid update auth", async () => {
    await assert.rejects(program.methods
      .updateMinStakeAmount(new anchor.BN(100))
      .accounts({
        authority: fixture.updateAuth.publicKey,
        stakingInfo:
          fixture.staking.info.publicKey
      })
      .signers([fixture.updateAuth])
      .rpc(),
      (e: AnchorError) => {
        assert.ok(e.error !== undefined);
        assert.strictEqual(e.error.errorCode.code, "AccessDenied");
        assert.strictEqual(e.error.errorCode.number, 6000);
        assert.strictEqual(e.error.errorMessage, "Access denied");
        return true;
      });
  });

  it("Update min stake amount with invalid value", async () => {
    await assert.rejects(program.methods
      .updateMinStakeAmount(new anchor.BN(0))
      .accounts({
        authority: newUpdateAuth.publicKey,
        stakingInfo:
          fixture.staking.info.publicKey
      })
      .signers([newUpdateAuth])
      .rpc(),
      (e: AnchorError) => {
        assert.ok(e.error !== undefined);
        assert.strictEqual(e.error.errorCode.code, "InvalidMinStakeAmount");
        assert.strictEqual(e.error.errorCode.number, 6003);
        assert.strictEqual(e.error.errorMessage, "Invalid min stake amount");
        return true;
      });
  });

  it("Update min stake amount", async () => {
    const newMinStakeAmount = 5_000_000_000;
    await program.methods
      .updateMinStakeAmount(new anchor.BN(newMinStakeAmount))
      .accounts({
        authority: newUpdateAuth.publicKey,
        stakingInfo:
          fixture.staking.info.publicKey
      })
      .signers([newUpdateAuth])
      .rpc();

    const stakingInfoData = await program.account.stakingInfo.fetch(fixture.staking.info.publicKey);
    assert.equal(stakingInfoData.minStakeAmount.toNumber(), newMinStakeAmount);
  });

  it("Update epoch period with invalid update auth", async () => {
    await assert.rejects(program.methods
      .updateEpochPeriodDays(100)
      .accounts({
        authority: fixture.updateAuth.publicKey,
        stakingInfo:
          fixture.staking.info.publicKey
      })
      .signers([fixture.updateAuth])
      .rpc(),
      (e: AnchorError) => {
        assert.ok(e.error !== undefined);
        assert.strictEqual(e.error.errorCode.code, "AccessDenied");
        assert.strictEqual(e.error.errorCode.number, 6000);
        assert.strictEqual(e.error.errorMessage, "Access denied");
        return true;
      });
  });

  it("Update epoch period with invalid value", async () => {
    await assert.rejects(program.methods
      .updateEpochPeriodDays(0)
      .accounts({
        authority: newUpdateAuth.publicKey,
        stakingInfo:
          fixture.staking.info.publicKey
      })
      .signers([newUpdateAuth])
      .rpc(),
      (e: AnchorError) => {
        assert.ok(e.error !== undefined);
        assert.strictEqual(e.error.errorCode.code, "InvalidEpochPeriodDays");
        assert.strictEqual(e.error.errorCode.number, 6002);
        assert.strictEqual(e.error.errorMessage, "Invalid epoch period days");
        return true;
      });
  });

  it("Update epoch period in days", async () => {
    const newEpochPeriod = 15;
    await program.methods
      .updateEpochPeriodDays(newEpochPeriod)
      .accounts({
        authority: newUpdateAuth.publicKey,
        stakingInfo:
          fixture.staking.info.publicKey
      })
      .signers([newUpdateAuth])
      .rpc();

    const stakingInfoData = await program.account.stakingInfo.fetch(fixture.staking.info.publicKey);
    assert.equal(stakingInfoData.epochPeriodDays, newEpochPeriod);
  });

  it("Update hold period with invalid update auth", async () => {
    await assert.rejects(program.methods
      .updateHoldPeriodDays(100)
      .accounts({
        authority: fixture.updateAuth.publicKey,
        stakingInfo:
          fixture.staking.info.publicKey
      })
      .signers([fixture.updateAuth])
      .rpc(),
      (e: AnchorError) => {
        assert.ok(e.error !== undefined);
        assert.strictEqual(e.error.errorCode.code, "AccessDenied");
        assert.strictEqual(e.error.errorCode.number, 6000);
        assert.strictEqual(e.error.errorMessage, "Access denied");
        return true;
      });
  });

  it("Update hold period with invalid value", async () => {
    await assert.rejects(program.methods
      .updateHoldPeriodDays(0)
      .accounts({
        authority: newUpdateAuth.publicKey,
        stakingInfo:
          fixture.staking.info.publicKey
      })
      .signers([newUpdateAuth])
      .rpc(),
      (e: AnchorError) => {
        assert.ok(e.error !== undefined);
        assert.strictEqual(e.error.errorCode.code, "InvalidHoldPeriodDays");
        assert.strictEqual(e.error.errorCode.number, 6004);
        assert.strictEqual(e.error.errorMessage, "Invalid hold period days");
        return true;
      });
  });

  it("Update hold period in days", async () => {
    const newHoldPeriod = 15;
    await program.methods
      .updateHoldPeriodDays(newHoldPeriod)
      .accounts({
        authority: newUpdateAuth.publicKey,
        stakingInfo:
          fixture.staking.info.publicKey
      })
      .signers([newUpdateAuth])
      .rpc();

    const stakingInfoData = await program.account.stakingInfo.fetch(fixture.staking.info.publicKey);
    assert.equal(stakingInfoData.holdPeriodDays, newHoldPeriod);
  });
});
