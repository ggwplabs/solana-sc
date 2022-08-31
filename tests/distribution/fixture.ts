import { Program } from "@project-serum/anchor";
import { findProgramAddressSync } from "@project-serum/anchor/dist/cjs/utils/pubkey";
import { utf8 } from "@project-serum/anchor/dist/cjs/utils/bytes";
import { Keypair, PublicKey } from "@solana/web3.js";
import { Distribution } from "../../target/types/distribution";
import * as utils from "../utils";

export class DistributionTestFixture {
  admin: Keypair;
  updateAuth: Keypair;
  info: Keypair;
  ggwpToken: PublicKey;

  accumulativeFund: PublicKey;
  accumulativeFundAuth: PublicKey;

  playToEarnFund: PublicKey;
  stakingFund: PublicKey;
  companyFund: PublicKey;
  teamFund: PublicKey;
}

export async function prepareDistributionTestFixture(distribution: Program<Distribution>): Promise<DistributionTestFixture> {
  const admin = Keypair.generate();
  const updateAuth = Keypair.generate();

  await utils.airdropSol(distribution.provider.connection, admin.publicKey, 200_000_000_000);
  await utils.airdropSol(distribution.provider.connection, updateAuth.publicKey, 200_000_000_000);

  const distributionInfo = Keypair.generate();

  const ggwpToken = await utils.createMint(admin.publicKey, 9);
  const accumulativeFundAuth = findProgramAddressSync(
    [
      utf8.encode(utils.ACCUMULATIVE_FUND_AUTH_SEED),
      distributionInfo.publicKey.toBytes(),
    ],
    distribution.programId,
  )[0];
  const accumulativeFund = await utils.createTokenWallet(ggwpToken, accumulativeFundAuth);
  await utils.mintTokens(ggwpToken, admin, accumulativeFund, 10000_000_000_000);

  // Fund authorities just for test
  const playToEarnFundAuth = Keypair.generate();
  const playToEarnFund = await utils.createTokenWallet(ggwpToken, playToEarnFundAuth.publicKey);
  const stakingFundAuth = Keypair.generate();
  const stakingFund = await utils.createTokenWallet(ggwpToken, stakingFundAuth.publicKey);
  const companyFundAuth = Keypair.generate();
  const companyFund = await utils.createTokenWallet(ggwpToken, companyFundAuth.publicKey);
  const teamFundAuth = Keypair.generate();
  const teamFund = await utils.createTokenWallet(ggwpToken, teamFundAuth.publicKey);

  return {
    admin: admin,
    updateAuth: updateAuth,
    info: distributionInfo,
    ggwpToken: ggwpToken,

    accumulativeFund: accumulativeFund,
    accumulativeFundAuth: accumulativeFundAuth,

    playToEarnFund: playToEarnFund,
    stakingFund: stakingFund,
    companyFund: companyFund,
    teamFund: teamFund,
  }
}
