import * as anchor from "@project-serum/anchor";
import { AnchorError, Program } from "@project-serum/anchor";
import { SystemProgram } from "@solana/web3.js";
import { Freezing } from "../../target/types/freezing";
import { Gpass } from "../../target/types/gpass";
import * as assert from "assert";
import * as utils from "../utils";
import { FreezingTestFixture, prepareFreezingTestFixture } from "./fixture";
import { TOKEN_PROGRAM_ID } from "@project-serum/anchor/dist/cjs/utils/token";

describe("Freezing bad cases tests", () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  const freezingProgram = anchor.workspace.Freezing as Program<Freezing>;
  const gpassProgram = anchor.workspace.Gpass as Program<Gpass>;

  const rewardPeriod = 20;
  const royalty = 8;
  const unfreezeRoyalty = 15;
  const unfreezeLockPeriod = 10;
  const rewardTable = [
    {
      ggwpAmount: new anchor.BN(10_000_000_000),
      gpassAmount: new anchor.BN(5),
    },
    {
      ggwpAmount: new anchor.BN(20_000_000_000),
      gpassAmount: new anchor.BN(10),
    },
    {
      ggwpAmount: new anchor.BN(30_000_000_000),
      gpassAmount: new anchor.BN(15),
    }
  ];

  let fixture: FreezingTestFixture;
  before(async () => {
    fixture = await prepareFreezingTestFixture(freezingProgram, gpassProgram);
    await freezingProgram.methods.initialize(
      fixture.updateAuth.publicKey,
      new anchor.BN(rewardPeriod),
      royalty,
      unfreezeRoyalty,
      new anchor.BN(unfreezeLockPeriod),
      rewardTable,
    )
      .accounts({
        admin: fixture.admin.publicKey,
        freezingInfo: fixture.freezing.info.publicKey,
        accumulativeFund: fixture.freezing.accumulativeFund,
        gpassInfo: fixture.freezing.gpassInfo.publicKey,
        gpassMintAuth: fixture.freezing.gpassMintAuth,
        treasury: fixture.freezing.treasury,
        treasuryAuth: fixture.freezing.treasuryAuth,
        ggwpToken: fixture.freezing.ggwpToken,
        systemProgram: SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .signers([fixture.admin, fixture.freezing.info])
      .rpc();
  });

  it("User freeze zero amount", async () => {
    await assert.rejects(freezingProgram.methods.freeze(new anchor.BN(0))
      .accounts({
        user: fixture.user.kp.publicKey,
        userInfo: fixture.user.info,
        userGgwpWallet: fixture.user.ggwpWallet,
        userGpassWallet: fixture.user.gpassWallet,
        freezingInfo: fixture.freezing.info.publicKey,
        gpassInfo: fixture.freezing.gpassInfo.publicKey,
        gpassMintAuth: fixture.freezing.gpassMintAuth,
        accumulativeFund: fixture.freezing.accumulativeFund,
        treasury: fixture.freezing.treasury,
        systemProgram: SystemProgram.programId,
        gpassProgram: gpassProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .signers([fixture.user.kp])
      .rpc(),
      (e: AnchorError) => {
        assert.ok(e.error !== undefined);
        assert.strictEqual(e.error.errorCode.code, "ZeroFreezingAmount");
        assert.strictEqual(e.error.errorCode.number, 6015);
        assert.strictEqual(e.error.errorMessage, "Freezing amount cannot be zero");
        return true;
      }
    );
  });

  it("User trying to withdraw before freeze", async () => {
    await assert.rejects(freezingProgram.methods.withdrawGpass()
      .accounts({
        user: fixture.user.kp.publicKey,
        userInfo: fixture.user.info,
        userGpassWallet: fixture.user.gpassWallet,
        freezingInfo: fixture.freezing.info.publicKey,
        gpassInfo: fixture.freezing.gpassInfo.publicKey,
        gpassMintAuth: fixture.freezing.gpassMintAuth,
        gpassProgram: gpassProgram.programId,
      })
      .signers([fixture.user.kp])
      .rpc(),
      (e: AnchorError) => {
        assert.ok(e.error !== undefined);
        assert.strictEqual(e.error.errorCode.code, "AccountNotInitialized");
        assert.strictEqual(e.error.errorCode.number, 3012);
        assert.strictEqual(e.error.errorMessage, "The program expected this account to be already initialized");
        return true;
      }
    );
  });

  it("Trying to unfreeze before freeze", async () => {
    await assert.rejects(freezingProgram.methods.unfreeze()
      .accounts({
        user: fixture.user.kp.publicKey,
        userInfo: fixture.user.info,
        userGgwpWallet: fixture.user.ggwpWallet,
        userGpassWallet: fixture.user.gpassWallet,
        freezingInfo: fixture.freezing.info.publicKey,
        gpassInfo: fixture.freezing.gpassInfo.publicKey,
        gpassMintAuth: fixture.freezing.gpassMintAuth,
        accumulativeFund: fixture.freezing.accumulativeFund,
        treasury: fixture.freezing.treasury,
        treasuryAuth: fixture.freezing.treasuryAuth,
        gpassProgram: gpassProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .signers([fixture.user.kp])
      .rpc(),
      (e: AnchorError) => {
        assert.ok(e.error !== undefined);
        assert.strictEqual(e.error.errorCode.code, "AccountNotInitialized");
        assert.strictEqual(e.error.errorCode.number, 3012);
        assert.strictEqual(e.error.errorMessage, "The program expected this account to be already initialized");
        return true;
      }
    );
  });

  const userFreezeAmount = 10_870_000_000; // 10 GGWP + royalty percent
  it("User freeze amount of GGWP", async () => {
    await freezingProgram.methods.freeze(new anchor.BN(userFreezeAmount))
      .accounts({
        user: fixture.user.kp.publicKey,
        userInfo: fixture.user.info,
        userGgwpWallet: fixture.user.ggwpWallet,
        userGpassWallet: fixture.user.gpassWallet,
        freezingInfo: fixture.freezing.info.publicKey,
        gpassInfo: fixture.freezing.gpassInfo.publicKey,
        gpassMintAuth: fixture.freezing.gpassMintAuth,
        accumulativeFund: fixture.freezing.accumulativeFund,
        treasury: fixture.freezing.treasury,
        systemProgram: SystemProgram.programId,
        gpassProgram: gpassProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .signers([fixture.user.kp])
      .rpc();

    const userInfoData = await freezingProgram.account.userInfo.fetch(fixture.user.info);
    assert.ok(utils.assertWithPrecission(userInfoData.freezedAmount.toNumber(), userFreezeAmount - utils.calcRoyaltyAmount(userFreezeAmount, royalty), 1));
    const userWalletData = await gpassProgram.account.wallet.fetch(fixture.user.gpassWallet);
    assert.equal(userWalletData.amount.toNumber(), 5);
    assert.ok(utils.assertWithPrecission(await utils.getTokenBalance(fixture.freezing.accumulativeFund), utils.calcRoyaltyAmount(userFreezeAmount, royalty), 1));
    assert.ok(utils.assertWithPrecission(await utils.getTokenBalance(fixture.freezing.treasury), userFreezeAmount - utils.calcRoyaltyAmount(userFreezeAmount, royalty), 1));
    const freezingInfoData = await freezingProgram.account.freezingInfo.fetch(fixture.freezing.info.publicKey);
    assert.ok(utils.assertWithPrecission(freezingInfoData.totalFreezed.toNumber(), userFreezeAmount - utils.calcRoyaltyAmount(userFreezeAmount, royalty), 1));
    const gpassInfoData = await gpassProgram.account.gpassInfo.fetch(fixture.freezing.gpassInfo.publicKey);
    assert.equal(gpassInfoData.totalAmount.toNumber(), 5);
  });

  it("User trying to withdraw before period passed", async () => {
    await assert.rejects(freezingProgram.methods.withdrawGpass()
      .accounts({
        user: fixture.user.kp.publicKey,
        userInfo: fixture.user.info,
        userGpassWallet: fixture.user.gpassWallet,
        freezingInfo: fixture.freezing.info.publicKey,
        gpassInfo: fixture.freezing.gpassInfo.publicKey,
        gpassMintAuth: fixture.freezing.gpassMintAuth,
        gpassProgram: gpassProgram.programId,
      })
      .signers([fixture.user.kp])
      .rpc(),
      (e: AnchorError) => {
        assert.ok(e.error !== undefined);
        assert.strictEqual(e.error.errorCode.code, "ZeroGpassEarned");
        assert.strictEqual(e.error.errorCode.number, 6018);
        assert.strictEqual(e.error.errorMessage, "Zero GPASS earned");
        return true;
      }
    );
  });

  it("Additional freeze not avaliable", async () => {
    await assert.rejects(freezingProgram.methods.freeze(new anchor.BN(userFreezeAmount))
      .accounts({
        user: fixture.user.kp.publicKey,
        userInfo: fixture.user.info,
        userGgwpWallet: fixture.user.ggwpWallet,
        userGpassWallet: fixture.user.gpassWallet,
        freezingInfo: fixture.freezing.info.publicKey,
        gpassInfo: fixture.freezing.gpassInfo.publicKey,
        gpassMintAuth: fixture.freezing.gpassMintAuth,
        accumulativeFund: fixture.freezing.accumulativeFund,
        treasury: fixture.freezing.treasury,
        systemProgram: SystemProgram.programId,
        gpassProgram: gpassProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .signers([fixture.user.kp])
      .rpc(),
      (e: AnchorError) => {
        assert.ok(e.error !== undefined);
        assert.strictEqual(e.error.errorCode.code, "AdditionalFreezingNotAvailable");
        assert.strictEqual(e.error.errorCode.number, 6017);
        assert.strictEqual(e.error.errorMessage, "Additional freezing is not available");
        return true;
      }
    );

    const userInfoData = await freezingProgram.account.userInfo.fetch(fixture.user.info);
    assert.ok(utils.assertWithPrecission(userInfoData.freezedAmount.toNumber(), userFreezeAmount - utils.calcRoyaltyAmount(userFreezeAmount, royalty), 1));
    const userWalletData = await gpassProgram.account.wallet.fetch(fixture.user.gpassWallet);
    assert.equal(userWalletData.amount.toNumber(), 5);
    assert.ok(utils.assertWithPrecission(await utils.getTokenBalance(fixture.freezing.accumulativeFund), utils.calcRoyaltyAmount(userFreezeAmount, royalty), 1));
    assert.ok(utils.assertWithPrecission(await utils.getTokenBalance(fixture.freezing.treasury), userFreezeAmount - utils.calcRoyaltyAmount(userFreezeAmount, royalty), 1));
    const freezingInfoData = await freezingProgram.account.freezingInfo.fetch(fixture.freezing.info.publicKey);
    assert.ok(utils.assertWithPrecission(freezingInfoData.totalFreezed.toNumber(), userFreezeAmount - utils.calcRoyaltyAmount(userFreezeAmount, royalty), 1));
    const gpassInfoData = await gpassProgram.account.gpassInfo.fetch(fixture.freezing.gpassInfo.publicKey);
    assert.equal(gpassInfoData.totalAmount.toNumber(), 5);
  });

  it("Unfreeze full amount of GGWP", async () => {
    const userInfoDataBefore = await freezingProgram.account.userInfo.fetch(fixture.user.info);
    const freezedAmountBefore = userInfoDataBefore.freezedAmount.toNumber();
    const accumulativeFundAmountBefore = await utils.getTokenBalance(fixture.freezing.accumulativeFund);
    await freezingProgram.methods.unfreeze()
      .accounts({
        user: fixture.user.kp.publicKey,
        userInfo: fixture.user.info,
        userGgwpWallet: fixture.user.ggwpWallet,
        userGpassWallet: fixture.user.gpassWallet,
        freezingInfo: fixture.freezing.info.publicKey,
        gpassInfo: fixture.freezing.gpassInfo.publicKey,
        gpassMintAuth: fixture.freezing.gpassMintAuth,
        accumulativeFund: fixture.freezing.accumulativeFund,
        treasury: fixture.freezing.treasury,
        treasuryAuth: fixture.freezing.treasuryAuth,
        gpassProgram: gpassProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .signers([fixture.user.kp])
      .rpc();

    const userInfoData = await freezingProgram.account.userInfo.fetch(fixture.user.info);
    assert.equal(userInfoData.freezedAmount.toNumber(), 0);
    const userWalletData = await gpassProgram.account.wallet.fetch(fixture.user.gpassWallet);
    assert.equal(userWalletData.amount.toNumber(), 5);
    assert.ok(utils.assertWithPrecission(await utils.getTokenBalance(fixture.freezing.accumulativeFund), accumulativeFundAmountBefore + utils.calcRoyaltyAmount(freezedAmountBefore, unfreezeRoyalty), 1));
    assert.equal(await utils.getTokenBalance(fixture.freezing.treasury), 0);
    const freezingInfoData = await freezingProgram.account.freezingInfo.fetch(fixture.freezing.info.publicKey);
    assert.equal(freezingInfoData.totalFreezed.toNumber(), 0);
    const gpassInfoData = await gpassProgram.account.gpassInfo.fetch(fixture.freezing.gpassInfo.publicKey);
    assert.equal(gpassInfoData.totalAmount.toNumber(), 5);
  });

  it("Trying to unfreeze zero amount", async () => {
    await assert.rejects(freezingProgram.methods.unfreeze()
      .accounts({
        user: fixture.user.kp.publicKey,
        userInfo: fixture.user.info,
        userGgwpWallet: fixture.user.ggwpWallet,
        userGpassWallet: fixture.user.gpassWallet,
        freezingInfo: fixture.freezing.info.publicKey,
        gpassInfo: fixture.freezing.gpassInfo.publicKey,
        gpassMintAuth: fixture.freezing.gpassMintAuth,
        accumulativeFund: fixture.freezing.accumulativeFund,
        treasury: fixture.freezing.treasury,
        treasuryAuth: fixture.freezing.treasuryAuth,
        gpassProgram: gpassProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .signers([fixture.user.kp])
      .rpc(),
      (e: AnchorError) => {
        assert.ok(e.error !== undefined);
        assert.strictEqual(e.error.errorCode.code, "ZeroUnfreezingAmount");
        assert.strictEqual(e.error.errorCode.number, 6016);
        assert.strictEqual(e.error.errorMessage, "Unfreezing amount cannot be zero");
        return true;
      }
    );
  });
});
