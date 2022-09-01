import * as anchor from "@project-serum/anchor";
import { AnchorError, Program } from "@project-serum/anchor";
import { Keypair, SystemProgram, LAMPORTS_PER_SOL } from "@solana/web3.js";
import { Distribution } from "../../target/types/distribution";
import * as assert from "assert";
import * as utils from "../utils";
import { DistributionTestFixture, prepareDistributionTestFixture } from "./fixture";

describe("Distribution authority tests", () => {
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

  const newAdmin = Keypair.generate();
  it("Update admin with invalid admin", async () => {
    const invalidAuthority = Keypair.generate();
    await utils.airdropSol(program.provider.connection, invalidAuthority.publicKey, 1 * LAMPORTS_PER_SOL);
    await assert.rejects(program.methods
      .updateAdmin(newAdmin.publicKey)
      .accounts({
        authority: invalidAuthority.publicKey,
        distributionInfo:
          fixture.info.publicKey
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
        distributionInfo:
          fixture.info.publicKey
      })
      .signers([fixture.admin])
      .rpc();

    const distributionInfoData = await program.account.distributionInfo.fetch(fixture.info.publicKey);
    assert.ok(distributionInfoData.admin.equals(newAdmin.publicKey));
  });

  const newUpdateAuth = Keypair.generate();
  it("Set update authority with invalid admin", async () => {
    await assert.rejects(program.methods
      .setUpdateAuthority(newUpdateAuth.publicKey)
      .accounts({
        authority: fixture.admin.publicKey,
        distributionInfo:
          fixture.info.publicKey
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
        distributionInfo:
          fixture.info.publicKey
      })
      .signers([newAdmin])
      .rpc();

    const distributionInfoData = await program.account.distributionInfo.fetch(fixture.info.publicKey);
    assert.ok(distributionInfoData.updateAuth.equals(newUpdateAuth.publicKey));
  });

  it("Update shares with invalid authority", async () => {
    await assert.rejects(program.methods
      .updateShares(45, 40, 5, 10)
      .accounts({
        authority: fixture.updateAuth.publicKey,
        distributionInfo:
          fixture.info.publicKey
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

  it("Update shares with invalid shares values", async () => {
    await assert.rejects(program.methods
      .updateShares(101, 40, 5, 10)
      .accounts({
        authority: newUpdateAuth.publicKey,
        distributionInfo:
          fixture.info.publicKey
      })
      .signers([newUpdateAuth])
      .rpc(),
      (e: AnchorError) => {
        assert.ok(e.error !== undefined);
        assert.strictEqual(e.error.errorCode.code, "InvalidShare");
        assert.strictEqual(e.error.errorCode.number, 6008);
        assert.strictEqual(e.error.errorMessage, "Invalid share percent value");
        return true;
      });

    await assert.rejects(program.methods
      .updateShares(45, 101, 5, 10)
      .accounts({
        authority: newUpdateAuth.publicKey,
        distributionInfo:
          fixture.info.publicKey
      })
      .signers([newUpdateAuth])
      .rpc(),
      (e: AnchorError) => {
        assert.ok(e.error !== undefined);
        assert.strictEqual(e.error.errorCode.code, "InvalidShare");
        assert.strictEqual(e.error.errorCode.number, 6008);
        assert.strictEqual(e.error.errorMessage, "Invalid share percent value");
        return true;
      });

    await assert.rejects(program.methods
      .updateShares(45, 40, 101, 10)
      .accounts({
        authority: newUpdateAuth.publicKey,
        distributionInfo:
          fixture.info.publicKey
      })
      .signers([newUpdateAuth])
      .rpc(),
      (e: AnchorError) => {
        assert.ok(e.error !== undefined);
        assert.strictEqual(e.error.errorCode.code, "InvalidShare");
        assert.strictEqual(e.error.errorCode.number, 6008);
        assert.strictEqual(e.error.errorMessage, "Invalid share percent value");
        return true;
      });

    await assert.rejects(program.methods
      .updateShares(45, 40, 5, 101)
      .accounts({
        authority: newUpdateAuth.publicKey,
        distributionInfo:
          fixture.info.publicKey
      })
      .signers([newUpdateAuth])
      .rpc(),
      (e: AnchorError) => {
        assert.ok(e.error !== undefined);
        assert.strictEqual(e.error.errorCode.code, "InvalidShare");
        assert.strictEqual(e.error.errorCode.number, 6008);
        assert.strictEqual(e.error.errorMessage, "Invalid share percent value");
        return true;
      });

    await assert.rejects(program.methods
      .updateShares(45, 40, 6, 10)
      .accounts({
        authority: newUpdateAuth.publicKey,
        distributionInfo:
          fixture.info.publicKey
      })
      .signers([newUpdateAuth])
      .rpc(),
      (e: AnchorError) => {
        assert.ok(e.error !== undefined);
        assert.strictEqual(e.error.errorCode.code, "InvalidShare");
        assert.strictEqual(e.error.errorCode.number, 6008);
        assert.strictEqual(e.error.errorMessage, "Invalid share percent value");
        return true;
      });
  });
});
