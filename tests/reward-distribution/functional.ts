import * as anchor from "@project-serum/anchor";
import { Program, AnchorError } from "@project-serum/anchor";
import { SystemProgram, Keypair } from "@solana/web3.js";
import { RewardDistribution } from "../../target/types/reward_distribution";
import * as assert from "assert";
import * as utils from "../utils";
import { RewardDistributionTestFixture, prepareRewardDistributionTestFixture } from "./fixture";
import { TOKEN_PROGRAM_ID } from "@project-serum/anchor/dist/cjs/utils/token";

describe("Reward Distribution functional tests", () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.RewardDistribution as Program<RewardDistribution>;

  let fixture: RewardDistributionTestFixture = null;

  before(async () => {
    fixture = await prepareRewardDistributionTestFixture(program);
    await program.methods.initialize(fixture.updateAuth.publicKey, [fixture.updateAuth.publicKey])
      .accounts({
        admin: fixture.admin.publicKey,
        rewardDistributionInfo: fixture.distribution.info.publicKey,
        ggwpToken: fixture.distribution.ggwpToken,
        playToEarnFund: fixture.distribution.playToEarnFund,
        playToEarnFundAuth: fixture.distribution.playToEarnFundAuth,
        systemProgram: SystemProgram.programId,
      })
      .signers([fixture.admin, fixture.distribution.info])
      .rpc();

    const rewardDistributionInfoData = await program.account.rewardDistributionInfo.fetch(fixture.distribution.info.publicKey);
    assert.ok(rewardDistributionInfoData.admin.equals(fixture.admin.publicKey));
    assert.ok(rewardDistributionInfoData.updateAuth.equals(fixture.updateAuth.publicKey));
    assert.deepStrictEqual(rewardDistributionInfoData.transferAuthList, [fixture.updateAuth.publicKey]);
  });

  it("Transfer tokens from fund with invalid authority", async () => {
    const invalidAuth = Keypair.generate();
    await utils.airdropSol(program.provider.connection, invalidAuth.publicKey, 1_000_000_000);
    await assert.rejects(program.methods.transfer(new anchor.BN(100))
      .accounts({
        authority: invalidAuth.publicKey,
        rewardDistributionInfo: fixture.distribution.info.publicKey,
        playToEarnFund: fixture.distribution.playToEarnFund,
        playToEarnFundAuth: fixture.distribution.playToEarnFundAuth,
        to: fixture.user.ggwpWallet,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .signers([invalidAuth])
      .rpc(),
      (e: AnchorError) => {
        assert.ok(e.error !== undefined);
        assert.strictEqual(e.error.errorCode.code, "InvalidTransferAuthority");
        assert.strictEqual(e.error.errorCode.number, 6005);
        assert.strictEqual(e.error.errorMessage, "Invalid transfer authority");
        return true;
      });
  });

  it("Transfer amount of tokens to user", async () => {
    const amount = 15_000_000_000;
    await program.methods.transfer(new anchor.BN(amount))
      .accounts({
        authority: fixture.updateAuth.publicKey,
        rewardDistributionInfo: fixture.distribution.info.publicKey,
        playToEarnFund: fixture.distribution.playToEarnFund,
        playToEarnFundAuth: fixture.distribution.playToEarnFundAuth,
        to: fixture.user.ggwpWallet,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .signers([fixture.updateAuth])
      .rpc();

    assert.equal(await utils.getTokenBalance(fixture.user.ggwpWallet), amount);
  });
});
