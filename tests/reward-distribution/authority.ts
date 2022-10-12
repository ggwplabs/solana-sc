import * as anchor from "@project-serum/anchor";
import { Program, AnchorError } from "@project-serum/anchor";
import { Keypair, SystemProgram, LAMPORTS_PER_SOL } from "@solana/web3.js";
import { RewardDistribution } from "../../target/types/reward_distribution";
import * as assert from "assert";
import * as utils from "../utils";
import { RewardDistributionTestFixture, prepareRewardDistributionTestFixture } from "./fixture";

describe("Reward Distribution authority tests", () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.RewardDistribution as Program<RewardDistribution>;

  let fixture: RewardDistributionTestFixture = null;
  before(async () => {
    fixture = await prepareRewardDistributionTestFixture(program);
    await program.methods.initialize(fixture.updateAuth.publicKey, [fixture.transferAuth.publicKey])
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
  });

  const newAdmin = Keypair.generate();
  it("update admin with invalid admin", async () => {
    const invalidAdmin = Keypair.generate();
    await utils.airdropSol(program.provider.connection, invalidAdmin.publicKey, 1 * LAMPORTS_PER_SOL);
    await assert.rejects(program.methods.updateAdmin(newAdmin.publicKey)
      .accounts({
        authority: invalidAdmin.publicKey,
        rewardDistributionInfo: fixture.distribution.info.publicKey,
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
    await program.methods.updateAdmin(newAdmin.publicKey)
      .accounts({
        authority: fixture.admin.publicKey,
        rewardDistributionInfo: fixture.distribution.info.publicKey,
      })
      .signers([fixture.admin])
      .rpc();

    const distributionInfoData = await program.account.rewardDistributionInfo.fetch(fixture.distribution.info.publicKey);
    assert.ok(distributionInfoData.admin.equals(newAdmin.publicKey));
  });

  const newUpdAuthority = Keypair.generate();
  it("set update authority with invalid admin", async () => {
    const invalidUpdateAuth = Keypair.generate();
    await utils.airdropSol(program.provider.connection, invalidUpdateAuth.publicKey, 1 * LAMPORTS_PER_SOL);
    await assert.rejects(program.methods.setUpdateAuthority(newUpdAuthority.publicKey)
      .accounts({
        authority: invalidUpdateAuth.publicKey,
        rewardDistributionInfo: fixture.distribution.info.publicKey,
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
    await program.methods.setUpdateAuthority(newUpdAuthority.publicKey)
      .accounts({
        authority: newAdmin.publicKey,
        rewardDistributionInfo: fixture.distribution.info.publicKey,
      })
      .signers([newAdmin])
      .rpc();

    const distributionInfoData = await program.account.rewardDistributionInfo.fetch(fixture.distribution.info.publicKey);
    assert.ok(distributionInfoData.updateAuth.equals(newUpdAuthority.publicKey));
  });

  it("update transfer auth list with invalid authority", async () => {
    const invalidUpdateAuth = Keypair.generate();
    const newTransferAuthList = [Keypair.generate().publicKey, Keypair.generate().publicKey];
    await utils.airdropSol(program.provider.connection, invalidUpdateAuth.publicKey, 1 * LAMPORTS_PER_SOL);
    await assert.rejects(program.methods.updateTransferAuthorityList(newTransferAuthList)
      .accounts({
        authority: invalidUpdateAuth.publicKey,
        rewardDistributionInfo: fixture.distribution.info.publicKey,
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

  it("update transfer auth list with invalid list len", async () => {
    const newTransferAuthList = [
      Keypair.generate().publicKey, Keypair.generate().publicKey,
      Keypair.generate().publicKey, Keypair.generate().publicKey,
      Keypair.generate().publicKey, Keypair.generate().publicKey,
      Keypair.generate().publicKey, Keypair.generate().publicKey,
    ];
    await assert.rejects(program.methods.updateTransferAuthorityList(newTransferAuthList)
      .accounts({
        authority: newUpdAuthority.publicKey,
        rewardDistributionInfo: fixture.distribution.info.publicKey,
      })
      .signers([newUpdAuthority])
      .rpc(),
      (e: AnchorError) => {
        assert.ok(e.error !== undefined);
        assert.strictEqual(e.error.errorCode.code, "InvalidTransferAuthList");
        assert.strictEqual(e.error.errorCode.number, 6004);
        assert.strictEqual(e.error.errorMessage, "Invalid transfer auth list");
        return true;
      }
    );
  });

  it("update transfer auth list", async () => {
    const newTransferAuthList = [Keypair.generate().publicKey, Keypair.generate().publicKey];
    await program.methods.updateTransferAuthorityList(newTransferAuthList)
      .accounts({
        authority: newUpdAuthority.publicKey,
        rewardDistributionInfo: fixture.distribution.info.publicKey,
      })
      .signers([newUpdAuthority])
      .rpc();

    const distributionInfoData = await program.account.rewardDistributionInfo.fetch(fixture.distribution.info.publicKey);
    assert.deepStrictEqual(distributionInfoData.transferAuthList, newTransferAuthList);
  });
});
