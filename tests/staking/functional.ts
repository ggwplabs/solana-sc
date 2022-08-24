import * as anchor from "@project-serum/anchor";
import { AnchorError, Program } from "@project-serum/anchor";
import { SystemProgram } from "@solana/web3.js";
import { Staking } from "../../target/types/staking";
import * as assert from "assert";
import * as utils from "../utils";
import { StakingTestFixture, prepareStakingTestFixture } from "./fixture";
import { TOKEN_PROGRAM_ID } from "@project-serum/anchor/dist/cjs/utils/token";

describe("Staking functional tests", () => {
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

  it("User stake amount of GGWP less than min stake amount", async () => {
    const stakeAmount = 5_000_000_000;
    await assert.rejects(program.methods.stake(new anchor.BN(stakeAmount))
      .accounts({
        user: fixture.user.kp.publicKey,
        stakingInfo: fixture.staking.info.publicKey,
        userInfo: fixture.user.info,
        userGgwpWallet: fixture.user.ggwpWallet,
        accumulativeFund: fixture.staking.accumulativeFund,
        treasury: fixture.staking.treasury,
        systemProgram: SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .signers([fixture.user.kp])
      .rpc(),
      (e: AnchorError) => {
        assert.ok(e.error !== undefined);
        assert.strictEqual(e.error.errorCode.code, "MinStakeAmountExceeded");
        assert.strictEqual(e.error.errorCode.number, 6017);
        assert.strictEqual(e.error.errorMessage, "Minimum stake amount exceeded");
        return true;
      }
    );
  });

  it("User stake amount of GGWP", async () => {
    const userTokenBalanceBefore = await utils.getTokenBalance(fixture.user.ggwpWallet);
    const stakeAmount = 5000_000_000_000;
    await program.methods.stake(new anchor.BN(stakeAmount))
      .accounts({
        user: fixture.user.kp.publicKey,
        stakingInfo: fixture.staking.info.publicKey,
        userInfo: fixture.user.info,
        userGgwpWallet: fixture.user.ggwpWallet,
        accumulativeFund: fixture.staking.accumulativeFund,
        treasury: fixture.staking.treasury,
        systemProgram: SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .signers([fixture.user.kp])
      .rpc();

    const stakingInfoData = await program.account.stakingInfo.fetch(fixture.staking.info.publicKey);
    assert.equal(stakingInfoData.totalStaked.toNumber(), stakeAmount - utils.calcRoyaltyAmount(stakeAmount, stakingInfoData.royalty));
    const userInfoData = await program.account.userInfo.fetch(fixture.user.info);
    assert.equal(userInfoData.amount.toNumber(), stakeAmount - utils.calcRoyaltyAmount(stakeAmount, stakingInfoData.royalty));
    assert.equal(await utils.getTokenBalance(fixture.user.ggwpWallet), userTokenBalanceBefore - stakeAmount);
  });
});
