import * as anchor from "@project-serum/anchor";
import { Program, AnchorError } from "@project-serum/anchor";
import { SystemProgram } from "@solana/web3.js";
import { Fighting } from "../../target/types/fighting";
import { Gpass } from "../../target/types/gpass";
import { Freezing } from "../../target/types/freezing";
import { RewardDistribution } from "../../target/types/reward_distribution";
import * as assert from "assert";
import * as utils from "../utils";
import { FightingTestFixture, prepareFightingTestFixture } from "./fixture";
import { findProgramAddressSync } from "@project-serum/anchor/dist/cjs/utils/pubkey";
import { utf8 } from "@project-serum/anchor/dist/cjs/utils/bytes";
import { TOKEN_PROGRAM_ID } from "@project-serum/anchor/dist/cjs/utils/token";

describe("Fighting functional tests", () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  const fighting = anchor.workspace.Fighting as Program<Fighting>;
  const gpass = anchor.workspace.Gpass as Program<Gpass>;
  const rewardDistribution = anchor.workspace.RewardDistribution as Program<RewardDistribution>;
  const freezing = anchor.workspace.Freezing as Program<Freezing>;

  let fixture: FightingTestFixture = null;
  const afkTimeout = 3;
  const royalty = 8;
  const rewardCoefficient = 20000;
  const gpassDailyRewardCoefficient = 10;
  const userGpassBalance = 5;
  const gameId: number = 1;
  let gameInfo = null;

  const freezingRewardPeriod = 3;
  before(async () => {
    fixture = await prepareFightingTestFixture(fighting, gpass, rewardDistribution, freezing, freezingRewardPeriod);
    await fighting.methods.initialize(fixture.updateAuth.publicKey, new anchor.BN(afkTimeout), rewardCoefficient, gpassDailyRewardCoefficient, royalty)
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

    const fightingSettingsData = await fighting.account.fightingSettings.fetch(fixture.fighting.settings.publicKey);
    assert.ok(fightingSettingsData.admin.equals(fixture.admin.publicKey));
    assert.ok(fightingSettingsData.updateAuth.equals(fixture.updateAuth.publicKey));
    assert.equal(fightingSettingsData.afkTimeout.toNumber(), afkTimeout);
    assert.equal(fightingSettingsData.rewardCoefficient, rewardCoefficient);
    assert.equal(fightingSettingsData.gpassDailyRewardCoefficient, gpassDailyRewardCoefficient);
    assert.equal(fightingSettingsData.royalty, royalty);

    gameInfo = findProgramAddressSync(
      [
        utf8.encode(utils.GAME_INFO_SEED),
        fixture.fighting.settings.publicKey.toBytes(),
        fixture.user.kp.publicKey.toBytes(),
        new anchor.BN(gameId).toArrayLike(Buffer, "le", 8),
      ],
      fighting.programId,
    )[0];
  });

  it("User first time start game without GPASS", async () => {
    await assert.rejects(fighting.methods.startGame()
      .accounts({
        user: fixture.user.kp.publicKey,
        userInfo: fixture.user.info,
        fightingSettings: fixture.fighting.settings.publicKey,
        gpassInfo: fixture.fighting.gpassInfo.publicKey,
        gpassBurnAuth: fixture.fighting.gpassBurnAuth,
        userGpassWallet: fixture.user.gpassWallet,
        gpassProgram: gpass.programId,
        systemProgram: SystemProgram.programId,
      })
      .signers([fixture.user.kp])
      .rpc(),
      (e: AnchorError) => {
        assert.ok(e.error !== undefined);
        assert.strictEqual(e.error.errorCode.code, "NotEnoughGpass");
        assert.strictEqual(e.error.errorCode.number, 6004);
        assert.strictEqual(e.error.errorMessage, "Not enough gpass for game");
        return true;
      });
  });

  it("User freezing ggwp to get GPASS", async () => {
    const userFreezeAmount = 10_870_000_000; // 10 GGWP + royalty percent
    await freezing.methods.freeze(new anchor.BN(userFreezeAmount))
      .accounts({
        user: fixture.user.kp.publicKey,
        accumulativeFund: fixture.fighting.accumulativeFund,
        freezingInfo: fixture.fighting.freezingInfo.publicKey,
        gpassInfo: fixture.fighting.gpassInfo.publicKey,
        gpassMintAuth: fixture.fighting.gpassMintAuth,
        treasury: fixture.fighting.freezingTreasury,
        userGgwpWallet: fixture.user.ggwpWallet,
        userGpassWallet: fixture.user.gpassWallet,
        userInfo: fixture.user.freezingInfo,
        systemProgram: SystemProgram.programId,
        gpassProgram: gpass.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .signers([fixture.user.kp])
      .rpc();

    await utils.sleep(freezingRewardPeriod);

    const userWalletData = await gpass.account.wallet.fetch(fixture.user.gpassWallet);
    assert.equal(userWalletData.amount.toNumber(), userGpassBalance);
  });

  it("User first time start game", async () => {
    await fighting.methods.startGame()
      .accounts({
        user: fixture.user.kp.publicKey,
        userInfo: fixture.user.info,
        fightingSettings: fixture.fighting.settings.publicKey,
        gpassInfo: fixture.fighting.gpassInfo.publicKey,
        gpassBurnAuth: fixture.fighting.gpassBurnAuth,
        userGpassWallet: fixture.user.gpassWallet,
        gpassProgram: gpass.programId,
        systemProgram: SystemProgram.programId,
      })
      .signers([fixture.user.kp])
      .rpc();

    const userFightingInfoData = await fighting.account.userFightingInfo.fetch(fixture.user.info);
    assert.equal(userFightingInfoData.inGame, true);
    assert.notEqual(userFightingInfoData.inGameTime.toNumber(), 0);
    const userGpassWalletData = await gpass.account.wallet.fetch(fixture.user.gpassWallet);
    assert.equal(userGpassWalletData.amount.toNumber(), userGpassBalance - 1);
  });

  it("User restart the game", async () => {
    await assert.rejects(fighting.methods.startGame()
      .accounts({
        user: fixture.user.kp.publicKey,
        userInfo: fixture.user.info,
        fightingSettings: fixture.fighting.settings.publicKey,
        gpassInfo: fixture.fighting.gpassInfo.publicKey,
        gpassBurnAuth: fixture.fighting.gpassBurnAuth,
        userGpassWallet: fixture.user.gpassWallet,
        gpassProgram: gpass.programId,
        systemProgram: SystemProgram.programId,
      })
      .signers([fixture.user.kp])
      .rpc(),
      (e: AnchorError) => {
        assert.ok(e.error !== undefined);
        assert.strictEqual(e.error.errorCode.code, "StillInGame");
        assert.strictEqual(e.error.errorCode.number, 6005);
        assert.strictEqual(e.error.errorMessage, "Still in game");
        return true;
      });
  });

  it("User try to restart the game after afk timeout", async () => {
    await utils.sleep(afkTimeout * 2);
    await fighting.methods.startGame()
      .accounts({
        user: fixture.user.kp.publicKey,
        userInfo: fixture.user.info,
        fightingSettings: fixture.fighting.settings.publicKey,
        gpassInfo: fixture.fighting.gpassInfo.publicKey,
        gpassBurnAuth: fixture.fighting.gpassBurnAuth,
        userGpassWallet: fixture.user.gpassWallet,
        gpassProgram: gpass.programId,
        systemProgram: SystemProgram.programId,
      })
      .signers([fixture.user.kp])
      .rpc();

    const userFightingInfoData = await fighting.account.userFightingInfo.fetch(fixture.user.info);
    assert.equal(userFightingInfoData.inGame, false);
    assert.notEqual(userFightingInfoData.inGameTime.toNumber(), 0);
    const userGpassWalletData = await gpass.account.wallet.fetch(fixture.user.gpassWallet);
    assert.equal(userGpassWalletData.amount.toNumber(), userGpassBalance - 1);
  });

  it("Try to finalize game with user not in game", async () => {
    await assert.rejects(fighting.methods.finalizeGame(new anchor.BN(gameId), { win: {} }, [])
      .accounts({
        user: fixture.user.kp.publicKey,
        validator: fixture.admin.publicKey,
        userInfo: fixture.user.info,
        userGgwpWallet: fixture.user.ggwpWallet,
        gameInfo: gameInfo,
        accumulativeFund: fixture.fighting.accumulativeFund,
        playToEarnFund: fixture.fighting.playToEarnFund,
        playToEarnFundAuth: fixture.fighting.playToEarnFundAuth,
        rewardDistributionInfo: fixture.fighting.rewardDistributionInfo.publicKey,
        rewardTransferAuth: fixture.fighting.transferAuth,
        freezingInfo: fixture.fighting.freezingInfo.publicKey,
        fightingSettings: fixture.fighting.settings.publicKey,
        systemProgram: SystemProgram.programId,
        rewardDistributionProgram: rewardDistribution.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .signers([fixture.user.kp, fixture.admin])
      .rpc(),
      (e: AnchorError) => {
        assert.ok(e.error !== undefined);
        assert.strictEqual(e.error.errorCode.code, "UserNotInGame");
        assert.strictEqual(e.error.errorCode.number, 6006);
        assert.strictEqual(e.error.errorMessage, "User not in game");
        return true;
      });
  });

  it("Start game after AFK kick", async () => {
    await fighting.methods.startGame()
      .accounts({
        user: fixture.user.kp.publicKey,
        userInfo: fixture.user.info,
        fightingSettings: fixture.fighting.settings.publicKey,
        gpassInfo: fixture.fighting.gpassInfo.publicKey,
        gpassBurnAuth: fixture.fighting.gpassBurnAuth,
        userGpassWallet: fixture.user.gpassWallet,
        gpassProgram: gpass.programId,
        systemProgram: SystemProgram.programId,
      })
      .signers([fixture.user.kp])
      .rpc();

    const userFightingInfoData = await fighting.account.userFightingInfo.fetch(fixture.user.info);
    assert.equal(userFightingInfoData.inGame, true);
    assert.notEqual(userFightingInfoData.inGameTime.toNumber(), 0);
    const userGpassWalletData = await gpass.account.wallet.fetch(fixture.user.gpassWallet);
    assert.equal(userGpassWalletData.amount.toNumber(), userGpassBalance - 2);
  });

  it("Try to finalize game with empty actions log", async () => {
    await assert.rejects(fighting.methods.finalizeGame(new anchor.BN(gameId), { win: {} }, [])
      .accounts({
        user: fixture.user.kp.publicKey,
        validator: fixture.admin.publicKey,
        userInfo: fixture.user.info,
        userGgwpWallet: fixture.user.ggwpWallet,
        gameInfo: gameInfo,
        accumulativeFund: fixture.fighting.accumulativeFund,
        playToEarnFund: fixture.fighting.playToEarnFund,
        playToEarnFundAuth: fixture.fighting.playToEarnFundAuth,
        rewardDistributionInfo: fixture.fighting.rewardDistributionInfo.publicKey,
        rewardTransferAuth: fixture.fighting.transferAuth,
        freezingInfo: fixture.fighting.freezingInfo.publicKey,
        fightingSettings: fixture.fighting.settings.publicKey,
        systemProgram: SystemProgram.programId,
        rewardDistributionProgram: rewardDistribution.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .signers([fixture.user.kp, fixture.admin])
      .rpc(),
      (e: AnchorError) => {
        assert.ok(e.error !== undefined);
        assert.strictEqual(e.error.errorCode.code, "InvalidActionsLogSize");
        assert.strictEqual(e.error.errorCode.number, 6007);
        assert.strictEqual(e.error.errorMessage, "Invalid actions log size");
        return true;
      });
  });

  it("Try to finalize game with invalid actions log size", async () => {
    let actions = [];
    for (let i = 0; i < 55; i++) { // MAX + 1
      actions.push({ who: { player: {} }, action: { armShort: {} } });
    }
    await assert.rejects(fighting.methods.finalizeGame(new anchor.BN(gameId), { win: {} }, actions)
      .accounts({
        user: fixture.user.kp.publicKey,
        validator: fixture.admin.publicKey,
        userInfo: fixture.user.info,
        userGgwpWallet: fixture.user.ggwpWallet,
        gameInfo: gameInfo,
        accumulativeFund: fixture.fighting.accumulativeFund,
        playToEarnFund: fixture.fighting.playToEarnFund,
        playToEarnFundAuth: fixture.fighting.playToEarnFundAuth,
        rewardDistributionInfo: fixture.fighting.rewardDistributionInfo.publicKey,
        rewardTransferAuth: fixture.fighting.transferAuth,
        freezingInfo: fixture.fighting.freezingInfo.publicKey,
        fightingSettings: fixture.fighting.settings.publicKey,
        systemProgram: SystemProgram.programId,
        rewardDistributionProgram: rewardDistribution.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .signers([fixture.user.kp, fixture.admin])
      .rpc(),
      (e: AnchorError) => {
        assert.ok(e.error !== undefined);
        assert.strictEqual(e.error.errorCode.code, "InvalidActionsLogSize");
        assert.strictEqual(e.error.errorCode.number, 6007);
        assert.strictEqual(e.error.errorMessage, "Invalid actions log size");
        return true;
      });
  });

  it("Finalize game", async () => {
    let validatorBalanceBefore = await fighting.provider.connection.getBalance(fixture.admin.publicKey);
    let accumulativeFundBalanceBefore = await utils.getTokenBalance(fixture.fighting.accumulativeFund);
    let userGGWPBalanceBefore = await utils.getTokenBalance(fixture.user.ggwpWallet);

    let actions = [
      { who: { player: {} }, action: { armShort: {} } },
      { who: { bot: {} }, action: { legShort: {} } },
      { who: { player: {} }, action: { legLong: {} } },
    ];
    await fighting.methods.finalizeGame(new anchor.BN(gameId), { win: {} }, actions)
      .accounts({
        user: fixture.user.kp.publicKey,
        validator: fixture.admin.publicKey,
        userInfo: fixture.user.info,
        userGgwpWallet: fixture.user.ggwpWallet,
        gameInfo: gameInfo,
        accumulativeFund: fixture.fighting.accumulativeFund,
        playToEarnFund: fixture.fighting.playToEarnFund,
        playToEarnFundAuth: fixture.fighting.playToEarnFundAuth,
        rewardDistributionInfo: fixture.fighting.rewardDistributionInfo.publicKey,
        rewardTransferAuth: fixture.fighting.transferAuth,
        freezingInfo: fixture.fighting.freezingInfo.publicKey,
        fightingSettings: fixture.fighting.settings.publicKey,
        systemProgram: SystemProgram.programId,
        rewardDistributionProgram: rewardDistribution.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .signers([fixture.user.kp, fixture.admin])
      .rpc();

    const userFightingInfoData = await fighting.account.userFightingInfo.fetch(fixture.user.info);
    assert.equal(userFightingInfoData.inGame, false);
    assert.notEqual(userFightingInfoData.inGameTime.toNumber(), 0);
    const userGpassWalletData = await gpass.account.wallet.fetch(fixture.user.gpassWallet);
    assert.equal(userGpassWalletData.amount.toNumber(), userGpassBalance - 2);
    const gameInfoData = await fighting.account.gameInfo.fetch(gameInfo);
    assert.equal(gameInfoData.id.toNumber(), gameId);
    assert.ok(gameInfoData.result["win"] !== undefined);
    assert.deepStrictEqual(gameInfoData.actionsLog, actions);
    assert.equal(await utils.getTokenBalance(fixture.fighting.accumulativeFund), accumulativeFundBalanceBefore + 400_000);
    assert.equal(await utils.getTokenBalance(fixture.user.ggwpWallet), userGGWPBalanceBefore + 4_600_000);

    let validatorBalance = await fighting.provider.connection.getBalance(fixture.admin.publicKey);
    console.log("Actions log write cost: ", validatorBalanceBefore - validatorBalance);
    console.log("Actions log write cost: ", utils.amountToUiAmount(validatorBalanceBefore - validatorBalance, 9));
  });
});
