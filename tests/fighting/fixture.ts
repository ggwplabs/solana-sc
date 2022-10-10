import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { findProgramAddressSync } from "@project-serum/anchor/dist/cjs/utils/pubkey";
import { utf8 } from "@project-serum/anchor/dist/cjs/utils/bytes";
import {
  Keypair, SystemProgram, PublicKey
} from "@solana/web3.js";
import { Fighting } from "../../target/types/fighting";
import { Gpass } from "../../target/types/gpass";
import { RewardDistribution } from "../../target/types/reward_distribution";
import * as utils from "../utils";
import { Freezing } from "../../target/types/freezing";
import { TOKEN_PROGRAM_ID } from "@project-serum/anchor/dist/cjs/utils/token";

export class FightingTestFixture {
  admin: Keypair;
  updateAuth: Keypair;

  fighting: {
    settings: Keypair;
    gpassInfo: Keypair;
    freezingInfo: Keypair;
    freezingTreasury: PublicKey;
    freezingTreasuryAuth: PublicKey;
    rewardDistributionInfo: Keypair;
    gpassBurnAuth: PublicKey;
    gpassMintAuth: PublicKey;
    transferAuth: PublicKey;
    ggwpToken: PublicKey;
    accumulativeFund: PublicKey;
    playToEarnFund: PublicKey;
    playToEarnFundAuth: PublicKey;
  }

  user: {
    kp: Keypair;
    info: PublicKey;
    freezingInfo: PublicKey;
    gpassWallet: PublicKey;
    ggwpWallet: PublicKey;
  }
}

export async function prepareFightingTestFixture(
  fighting: Program<Fighting>,
  gpass: Program<Gpass>,
  rewardDistribution: Program<RewardDistribution>,
  freezing: Program<Freezing>,
  freezingRewardPeriod?: number,
  gpassBurnPeriod?: number
): Promise<FightingTestFixture> {
  const admin = Keypair.generate();
  const updateAuth = Keypair.generate();
  const user = Keypair.generate();

  await utils.airdropSol(fighting.provider.connection, admin.publicKey, 200_000_000_000);
  await utils.airdropSol(fighting.provider.connection, user.publicKey, 200_000_000_000);
  await utils.airdropSol(fighting.provider.connection, updateAuth.publicKey, 200_000_000_000);

  const gpassInfo = Keypair.generate();
  const fightingSettings = Keypair.generate();
  const rewardDistributionInfo = Keypair.generate();
  const freezingInfo = Keypair.generate();

  const playToEarnFundAuth = findProgramAddressSync(
    [
      utf8.encode(utils.PLAY_TO_EARN_FUND_AUTH_SEED),
      rewardDistributionInfo.publicKey.toBytes(),
    ],
    rewardDistribution.programId
  )[0];

  const freezingTreasuryAuth = findProgramAddressSync(
    [
      utf8.encode(utils.TREASURY_AUTH_SEED),
      freezingInfo.publicKey.toBytes(),
    ],
    freezing.programId,
  )[0];

  const ggwpToken = await utils.createMint(admin.publicKey, 9);
  const freezingTreasury = await utils.createTokenWallet(ggwpToken, freezingTreasuryAuth);
  const accumulativeFund = await utils.createTokenWallet(ggwpToken, admin.publicKey);
  const userGgwpTokenWallet = await utils.createTokenWallet(ggwpToken, user.publicKey);
  const playToEarnFund = await utils.createTokenWallet(ggwpToken, playToEarnFundAuth);
  await utils.mintTokens(ggwpToken, admin, userGgwpTokenWallet, 100_000_000_000);
  await utils.mintTokens(ggwpToken, admin, playToEarnFund, 100_000_000_000);

  const gpassBurnAuth = findProgramAddressSync(
    [
      utf8.encode(utils.GPASS_BURN_AUTH_SEED),
      fightingSettings.publicKey.toBytes(),
      gpassInfo.publicKey.toBytes(),
    ],
    fighting.programId
  )[0];

  const gpassMintAuth = findProgramAddressSync(
    [
      utf8.encode(utils.GPASS_MINT_AUTH_SEED),
      freezingInfo.publicKey.toBytes(),
      gpassInfo.publicKey.toBytes(),
    ],
    freezing.programId
  )[0];

  const transferAuth = findProgramAddressSync(
    [
      utf8.encode(utils.REWARD_TRANSFER_AUTH_SEED),
      fightingSettings.publicKey.toBytes(),
      rewardDistributionInfo.publicKey.toBytes(),
    ],
    fighting.programId
  )[0];

  let burnPeriod = gpassBurnPeriod ? gpassBurnPeriod : 30 * 60;
  await gpass.methods.initialize(
    new anchor.BN(burnPeriod),
    updateAuth.publicKey,
    [gpassMintAuth],
    [gpassBurnAuth])
    .accounts({
      admin: admin.publicKey,
      gpassInfo: gpassInfo.publicKey,
      systemProgram: SystemProgram.programId,
    })
    .signers([admin, gpassInfo])
    .rpc();

  const userGpassWallet = findProgramAddressSync(
    [
      utf8.encode(utils.USER_WALLET_SEED),
      gpassInfo.publicKey.toBytes(),
      user.publicKey.toBytes(),
    ],
    gpass.programId,
  )[0];

  const userFightingInfo = findProgramAddressSync(
    [
      utf8.encode(utils.USER_INFO_SEED),
      fightingSettings.publicKey.toBytes(),
      user.publicKey.toBytes(),
    ],
    fighting.programId,
  )[0];

  const userFreezingInfo = findProgramAddressSync(
    [
      utf8.encode(utils.USER_INFO_SEED),
      freezingInfo.publicKey.toBytes(),
      user.publicKey.toBytes(),
    ],
    freezing.programId,
  )[0];

  await gpass.methods.createWallet()
    .accounts({
      payer: admin.publicKey,
      user: user.publicKey,
      gpassInfo: gpassInfo.publicKey,
      wallet: userGpassWallet,
      systemProgram: SystemProgram.programId
    })
    .signers([admin])
    .rpc();

  await rewardDistribution.methods.initialize(updateAuth.publicKey, [transferAuth])
    .accounts({
      admin: admin.publicKey,
      ggwpToken: ggwpToken,
      playToEarnFund: playToEarnFund,
      playToEarnFundAuth: playToEarnFundAuth,
      rewardDistributionInfo: rewardDistributionInfo.publicKey,
      systemProgram: SystemProgram.programId,
    })
    .signers([admin, rewardDistributionInfo])
    .rpc();

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

  let rewardPeriod = freezingRewardPeriod ? freezingRewardPeriod : 3;
  await freezing.methods.initialize(
    updateAuth.publicKey,
    new anchor.BN(rewardPeriod),
    8,
    15,
    new anchor.BN(2),
    rewardTable,
  )
    .accounts({
      admin: admin.publicKey,
      freezingInfo: freezingInfo.publicKey,
      ggwpToken: ggwpToken,
      accumulativeFund: accumulativeFund,
      gpassInfo: gpassInfo.publicKey,
      gpassMintAuth: gpassMintAuth,
      treasury: freezingTreasury,
      treasuryAuth: freezingTreasuryAuth,
      systemProgram: SystemProgram.programId,
      tokenProgram: TOKEN_PROGRAM_ID,
    })
    .signers([admin, freezingInfo])
    .rpc();

  return {
    admin: admin,
    updateAuth: updateAuth,

    fighting: {
      settings: fightingSettings,
      gpassInfo: gpassInfo,
      freezingInfo: freezingInfo,
      freezingTreasury: freezingTreasury,
      freezingTreasuryAuth: freezingTreasuryAuth,
      rewardDistributionInfo: rewardDistributionInfo,
      gpassBurnAuth: gpassBurnAuth,
      gpassMintAuth: gpassMintAuth,
      transferAuth: transferAuth,
      ggwpToken: ggwpToken,
      accumulativeFund: accumulativeFund,
      playToEarnFund: playToEarnFund,
      playToEarnFundAuth: playToEarnFundAuth,
    },
    user: {
      kp: user,
      info: userFightingInfo,
      freezingInfo: userFreezingInfo,
      ggwpWallet: userGgwpTokenWallet,
      gpassWallet: userGpassWallet,
    }
  }
}
