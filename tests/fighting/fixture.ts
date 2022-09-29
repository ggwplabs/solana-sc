import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { findProgramAddressSync } from "@project-serum/anchor/dist/cjs/utils/pubkey";
import { utf8 } from "@project-serum/anchor/dist/cjs/utils/bytes";
import {
  Keypair, SystemProgram, PublicKey
} from "@solana/web3.js";
import { Fighting } from "../../target/types/fighting";
import { Gpass } from "../../target/types/gpass";
import * as utils from "../utils";

export class FightingTestFixture {
  admin: Keypair;
  updateAuth: Keypair;

  fighting: {
    settings: Keypair;
    gpassInfo: Keypair;
    gpassBurnAuth: PublicKey;
    ggwpToken: PublicKey;
    accumulativeFund: PublicKey;
  }

  user: {
    kp: Keypair;
    info: PublicKey;
    gpassWallet: PublicKey;
    ggwpWallet: PublicKey;
  }
}

export async function prepareFightingTestFixture(fighting: Program<Fighting>, gpass: Program<Gpass>, gpassBurnPeriod?: number): Promise<FightingTestFixture> {
  const admin = Keypair.generate();
  const updateAuth = Keypair.generate();
  const user = Keypair.generate();

  await utils.airdropSol(fighting.provider.connection, admin.publicKey, 200_000_000_000);
  await utils.airdropSol(fighting.provider.connection, user.publicKey, 200_000_000_000);
  await utils.airdropSol(fighting.provider.connection, updateAuth.publicKey, 200_000_000_000);

  const fightingSettings = Keypair.generate();

  const ggwpToken = await utils.createMint(admin.publicKey, 9);
  const accumulativeFund = await utils.createTokenWallet(ggwpToken, admin.publicKey);
  const userGgwpTokenWallet = await utils.createTokenWallet(ggwpToken, user.publicKey);
  await utils.mintTokens(ggwpToken, admin, userGgwpTokenWallet, 100_000_000_000);

  const gpassInfo = Keypair.generate();
  const gpassBurnAuth = findProgramAddressSync(
    [
      utf8.encode(utils.GPASS_BURN_AUTH_SEED),
      fightingSettings.publicKey.toBytes(),
      gpassInfo.publicKey.toBytes(),
    ],
    fighting.programId
  )[0];

  let burnPeriod = gpassBurnPeriod ? gpassBurnPeriod : 30 * 60;
  await gpass.methods.initialize(
    new anchor.BN(burnPeriod),
    updateAuth.publicKey,
    [admin.publicKey],
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

  return {
    admin: admin,
    updateAuth: updateAuth,

    fighting: {
      settings: fightingSettings,
      gpassInfo: gpassInfo,
      gpassBurnAuth: gpassBurnAuth,
      ggwpToken: ggwpToken,
      accumulativeFund: accumulativeFund,
    },
    user: {
      kp: user,
      info: userFightingInfo,
      ggwpWallet: userGgwpTokenWallet,
      gpassWallet: userGpassWallet,
    }
  }
}
