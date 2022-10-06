import { Program } from "@project-serum/anchor";
import { findProgramAddressSync } from "@project-serum/anchor/dist/cjs/utils/pubkey";
import { utf8 } from "@project-serum/anchor/dist/cjs/utils/bytes";
import { Keypair, PublicKey } from "@solana/web3.js";
import { RewardDistribution } from "../../target/types/reward_distribution";
import * as utils from "../utils";

export class RewardDistributionTestFixture {
  admin: Keypair;
  updateAuth: Keypair;

  distribution: {
    info: Keypair;
    ggwpToken: PublicKey;
    playToEarnFund: PublicKey;
    playToEarnFundAuth: PublicKey;
  }

  transferAuth: Keypair;
}

export async function prepareRewardDistributionTestFixture(program: Program<RewardDistribution>): Promise<RewardDistributionTestFixture> {
  const admin = Keypair.generate();
  const updateAuth = Keypair.generate();

  await utils.airdropSol(program.provider.connection, admin.publicKey, 200_000_000_000);
  await utils.airdropSol(program.provider.connection, updateAuth.publicKey, 200_000_000_000);

  const rewardDistributionInfo = Keypair.generate();
  const playToEarnFundAuth = findProgramAddressSync(
    [
      utf8.encode(utils.PLAY_TO_EARN_FUND_AUTH_SEED),
      rewardDistributionInfo.publicKey.toBytes(),
    ],
    program.programId
  )[0];

  const transferAuth = Keypair.generate();

  const ggwpToken = await utils.createMint(admin.publicKey, 9);
  const playToEarnFund = await utils.createTokenWallet(ggwpToken, playToEarnFundAuth);
  await utils.mintTokens(ggwpToken, admin, playToEarnFund, 100_000_000_000);

  return {
    admin: admin,
    updateAuth: updateAuth,

    distribution: {
      info: rewardDistributionInfo,
      playToEarnFund: playToEarnFund,
      playToEarnFundAuth: playToEarnFundAuth,
      ggwpToken: ggwpToken,
    },

    transferAuth: transferAuth,
  }
}
