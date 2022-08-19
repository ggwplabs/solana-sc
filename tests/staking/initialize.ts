import * as anchor from "@project-serum/anchor";
import { AnchorError, Program } from "@project-serum/anchor";
import { SystemProgram } from "@solana/web3.js";
import { Staking } from "../../target/types/staking";
import * as assert from "assert";
import { StakingTestFixture, prepareStakingTestFixture } from "./fixture";

describe("Staking initialize tests", () => {
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
  });

  it("Initialize with invalid epoch period days", async () => {
    await assert.rejects(program.methods.initialize(
      fixture.updateAuth.publicKey,
      0,
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
      .rpc(),
      (e: AnchorError) => {
        assert.ok(e.error !== undefined);
        assert.strictEqual(e.error.errorCode.code, "InvalidEpochPeriodDays");
        assert.strictEqual(e.error.errorCode.number, 6002);
        assert.strictEqual(e.error.errorMessage, "Invalid epoch period days");
        return true;
      }
    );

    await assert.rejects(program.methods.initialize(
      fixture.updateAuth.publicKey,
      epochPeriodDays,
      new anchor.BN(0),
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
      .rpc(),
      (e: AnchorError) => {
        assert.ok(e.error !== undefined);
        assert.strictEqual(e.error.errorCode.code, "InvalidMinStakeAmount");
        assert.strictEqual(e.error.errorCode.number, 6003);
        assert.strictEqual(e.error.errorMessage, "Invalid min stake amount");
        return true;
      }
    );

    await assert.rejects(program.methods.initialize(
      fixture.updateAuth.publicKey,
      epochPeriodDays,
      new anchor.BN(minStakeAmount),
      0,
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
      .rpc(),
      (e: AnchorError) => {
        assert.ok(e.error !== undefined);
        assert.strictEqual(e.error.errorCode.code, "InvalidHoldPeriodDays");
        assert.strictEqual(e.error.errorCode.number, 6004);
        assert.strictEqual(e.error.errorMessage, "Invalid hold period days");
        return true;
      }
    );

    await assert.rejects(program.methods.initialize(
      fixture.updateAuth.publicKey,
      epochPeriodDays,
      new anchor.BN(minStakeAmount),
      holdPeriodDays,
      0,
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
      .rpc(),
      (e: AnchorError) => {
        assert.ok(e.error !== undefined);
        assert.strictEqual(e.error.errorCode.code, "InvalidHoldRoyalty");
        assert.strictEqual(e.error.errorCode.number, 6005);
        assert.strictEqual(e.error.errorMessage, "Invalid hold royalty");
        return true;
      }
    );

    await assert.rejects(program.methods.initialize(
      fixture.updateAuth.publicKey,
      epochPeriodDays,
      new anchor.BN(minStakeAmount),
      holdPeriodDays,
      holdRoyalty,
      0,
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
      .rpc(),
      (e: AnchorError) => {
        assert.ok(e.error !== undefined);
        assert.strictEqual(e.error.errorCode.code, "InvalidRoyalty");
        assert.strictEqual(e.error.errorCode.number, 6006);
        assert.strictEqual(e.error.errorMessage, "Invalid royalty");
        return true;
      }
    );

    await assert.rejects(program.methods.initialize(
      fixture.updateAuth.publicKey,
      epochPeriodDays,
      new anchor.BN(minStakeAmount),
      holdPeriodDays,
      holdRoyalty,
      royalty,
      0,
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
      .rpc(),
      (e: AnchorError) => {
        assert.ok(e.error !== undefined);
        assert.strictEqual(e.error.errorCode.code, "InvalidAPR");
        assert.strictEqual(e.error.errorCode.number, 6007);
        assert.strictEqual(e.error.errorMessage, "Invalid APR");
        return true;
      }
    );

    await assert.rejects(program.methods.initialize(
      fixture.updateAuth.publicKey,
      epochPeriodDays,
      new anchor.BN(minStakeAmount),
      holdPeriodDays,
      holdRoyalty,
      royalty,
      aprStart,
      0,
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
      .rpc(),
      (e: AnchorError) => {
        assert.ok(e.error !== undefined);
        assert.strictEqual(e.error.errorCode.code, "InvalidAPR");
        assert.strictEqual(e.error.errorCode.number, 6007);
        assert.strictEqual(e.error.errorMessage, "Invalid APR");
        return true;
      }
    );

    await assert.rejects(program.methods.initialize(
      fixture.updateAuth.publicKey,
      epochPeriodDays,
      new anchor.BN(minStakeAmount),
      holdPeriodDays,
      holdRoyalty,
      royalty,
      aprStart,
      aprStep,
      0,
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
      .rpc(),
      (e: AnchorError) => {
        assert.ok(e.error !== undefined);
        assert.strictEqual(e.error.errorCode.code, "InvalidAPR");
        assert.strictEqual(e.error.errorCode.number, 6007);
        assert.strictEqual(e.error.errorMessage, "Invalid APR");
        return true;
      }
    );
  });

  it("Initialize", async () => {
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
});
