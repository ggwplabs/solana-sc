import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { findProgramAddressSync } from "@project-serum/anchor/dist/cjs/utils/pubkey";
import { utf8 } from "@project-serum/anchor/dist/cjs/utils/bytes";
import {
  Keypair, SystemProgram, PublicKey
} from "@solana/web3.js";
import { Freezing } from "../../target/types/freezing";
import { Gpass } from "../../target/types/gpass";
import * as utils from "../utils";

export class FreezingTestFixture {
  admin: Keypair;
  updateAuth: Keypair;

  freezing: {
    info: Keypair;
    gpassInfo: Keypair;
    gpassMintAuth: PublicKey;
    ggwpToken: PublicKey;
    accumulativeFund: PublicKey;
    treasury: PublicKey;
    treasuryAuth: PublicKey;
  }

  user: {
    kp: Keypair;
    info: PublicKey;
    gpassWallet: PublicKey;
    ggwpWallet: PublicKey;
  }
}

export async function prepareFreezingTestFixture(freezing: Program<Freezing>, gpass: Program<Gpass>, gpassBurnPeriod?: number): Promise<FreezingTestFixture> {
  const admin = Keypair.generate();
  const updateAuth = Keypair.generate();
  const user = Keypair.generate();

  await utils.airdropSol(freezing.provider.connection, admin.publicKey, 200_000_000_000);
  await utils.airdropSol(freezing.provider.connection, user.publicKey, 200_000_000_000);
  await utils.airdropSol(freezing.provider.connection, updateAuth.publicKey, 200_000_000_000);

  const freezingInfo = Keypair.generate();

  const ggwpToken = await utils.createMint(admin.publicKey, 9);
  const accumulativeFund = await utils.createTokenWallet(ggwpToken, admin.publicKey);
  const treasuryAuth = findProgramAddressSync(
    [
      utf8.encode(utils.TREASURY_AUTH_SEED),
      freezingInfo.publicKey.toBytes(),
    ],
    freezing.programId,
  )[0];
  const treasury = await utils.createTokenWallet(ggwpToken, treasuryAuth);
  const userGgwpTokenWallet = await utils.createTokenWallet(ggwpToken, user.publicKey);
  await utils.mintTokens(ggwpToken, admin, userGgwpTokenWallet, 100_000_000_000);

  const gpassInfo = Keypair.generate();
  const gpassMintAuth = findProgramAddressSync(
    [
      utf8.encode(utils.GPASS_MINT_AUTH_SEED),
      freezingInfo.publicKey.toBytes(),
      gpassInfo.publicKey.toBytes(),
    ],
    freezing.programId
  )[0];

  let burnPeriod = gpassBurnPeriod ? gpassBurnPeriod : 30 * 60;
  await gpass.methods.initialize(
    new anchor.BN(burnPeriod),
    updateAuth.publicKey,
    [gpassMintAuth],
    [])
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

  return {
    admin: admin,
    updateAuth: updateAuth,

    freezing: {
      info: freezingInfo,
      gpassInfo: gpassInfo,
      gpassMintAuth: gpassMintAuth,
      ggwpToken: ggwpToken,
      accumulativeFund: accumulativeFund,
      treasury: treasury,
      treasuryAuth: treasuryAuth,
    },
    user: {
      kp: user,
      info: userFreezingInfo,
      ggwpWallet: userGgwpTokenWallet,
      gpassWallet: userGpassWallet,
    }
  }
}
