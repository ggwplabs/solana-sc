import * as anchor from "@project-serum/anchor";
import { AnchorError, Program } from "@project-serum/anchor";
import { SystemProgram } from "@solana/web3.js";
import { Distribution } from "../../target/types/distribution";
import * as assert from "assert";
import * as utils from "../utils";
import { DistributionTestFixture, prepareDistributionTestFixture } from "./fixture";
import { TOKEN_PROGRAM_ID } from "@project-serum/anchor/dist/cjs/utils/token";

describe("Distribution functional tests", () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.Distribution as Program<Distribution>;

  const playToEarnFundShare: number = 45;
  const stakingFundShare: number = 40;
  const companyFundShare: number = 5;
  const teamFundShare: number = 10;

  let fixture: DistributionTestFixture = null;
  before(async () => {
    fixture = await prepareDistributionTestFixture(program);
    await program.methods.initialize(
      fixture.updateAuth.publicKey,
      playToEarnFundShare,
      stakingFundShare,
      companyFundShare,
      teamFundShare,
    )
      .accounts({
        admin: fixture.admin.publicKey,
        distributionInfo: fixture.info.publicKey,
        ggwpToken: fixture.ggwpToken,
        accumulativeFund: fixture.accumulativeFund,
        accumulativeFundAuth: fixture.accumulativeFundAuth,
        playToEarnFund: fixture.playToEarnFund,
        stakingFund: fixture.stakingFund,
        companyFund: fixture.companyFund,
        teamFund: fixture.teamFund,
        systemProgram: SystemProgram.programId,
      })
      .signers([fixture.admin, fixture.info])
      .rpc();

    const distributionInfoData = await program.account.distributionInfo.fetch(fixture.info.publicKey);
    assert.ok(distributionInfoData.admin.equals(fixture.admin.publicKey));
    assert.ok(distributionInfoData.updateAuth.equals(fixture.updateAuth.publicKey));
    assert.ok(distributionInfoData.ggwpToken.equals(fixture.ggwpToken));
    assert.ok(distributionInfoData.accumulativeFund.equals(fixture.accumulativeFund));
    assert.ok(distributionInfoData.playToEarnFund.equals(fixture.playToEarnFund));
    assert.equal(distributionInfoData.playToEarnFundShare, playToEarnFundShare);
    assert.ok(distributionInfoData.stakingFund.equals(fixture.stakingFund));
    assert.equal(distributionInfoData.stakingFundShare, stakingFundShare);
    assert.ok(distributionInfoData.companyFund.equals(fixture.companyFund));
    assert.equal(distributionInfoData.companyFundShare, companyFundShare);
    assert.ok(distributionInfoData.teamFund.equals(fixture.teamFund));
    assert.equal(distributionInfoData.teamFundShare, teamFundShare);
  });

  it("Distribute GGWP tokens", async () => {
    await program.methods
      .distribute()
      .accounts({
        distributionInfo: fixture.info.publicKey,
        accumulativeFund: fixture.accumulativeFund,
        accumulativeFundAuth: fixture.accumulativeFundAuth,
        playToEarnFund: fixture.playToEarnFund,
        stakingFund: fixture.stakingFund,
        companyFund: fixture.companyFund,
        teamFund: fixture.teamFund,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .rpc();

    assert.equal(await utils.getTokenBalance(fixture.accumulativeFund), 0);
    assert.equal(await utils.getTokenBalance(fixture.playToEarnFund), 4500_000_000_000);
    assert.equal(await utils.getTokenBalance(fixture.stakingFund), 4000_000_000_000);
    assert.equal(await utils.getTokenBalance(fixture.companyFund), 500_000_000_000);
    assert.equal(await utils.getTokenBalance(fixture.teamFund), 1000_000_000_000);
  });

  it("Distribute GGWP tokens with empty accumulative fund", async () => {
    await assert.rejects(program.methods
      .distribute()
      .accounts({
        distributionInfo: fixture.info.publicKey,
        accumulativeFund: fixture.accumulativeFund,
        accumulativeFundAuth: fixture.accumulativeFundAuth,
        playToEarnFund: fixture.playToEarnFund,
        stakingFund: fixture.stakingFund,
        companyFund: fixture.companyFund,
        teamFund: fixture.teamFund,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .rpc(),
      (e: AnchorError) => {
        assert.ok(e.error !== undefined);
        assert.strictEqual(e.error.errorCode.code, "EmptyAccumulativeFund");
        assert.strictEqual(e.error.errorCode.number, 6010);
        assert.strictEqual(e.error.errorMessage, "Empty accumulative fund");
        return true;
      });
  });

  it("Distribute non even number of GGWP tokens", async () => {
    await utils.mintTokens(fixture.ggwpToken, fixture.admin, fixture.accumulativeFund, 7001_000_000_001);
    await program.methods
      .distribute()
      .accounts({
        distributionInfo: fixture.info.publicKey,
        accumulativeFund: fixture.accumulativeFund,
        accumulativeFundAuth: fixture.accumulativeFundAuth,
        playToEarnFund: fixture.playToEarnFund,
        stakingFund: fixture.stakingFund,
        companyFund: fixture.companyFund,
        teamFund: fixture.teamFund,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .rpc();

    assert.equal(await utils.getTokenBalance(fixture.accumulativeFund), 0);
    assert.equal(await utils.getTokenBalance(fixture.playToEarnFund), 4500_000_000_000 + 3150_450_000_000);
    assert.equal(await utils.getTokenBalance(fixture.stakingFund), 4000_000_000_000 + 2800_400_000_000);
    assert.equal(await utils.getTokenBalance(fixture.companyFund), 500_000_000_000 + 350_050_000_000);
    assert.equal(await utils.getTokenBalance(fixture.teamFund), 1000_000_000_000 + 700_100_000_001);
  });
});
