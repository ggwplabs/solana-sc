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
    params: Keypair;

    gpassSettings: Keypair;
    gpassMintAuth: PublicKey,
    ggwpToken: PublicKey,
    accumulativeFund: PublicKey;
    treasury: PublicKey;
  }

  user: {
    kp: Keypair;
    gpassWallet: PublicKey;
    ggwpWallet: PublicKey;
  }
}

export async function prepareFreezingTestFixture(freezing: Program<Freezing>, gpass: Program<Gpass>): Promise<FreezingTestFixture> {
  const admin = Keypair.generate();
  const updateAuth = Keypair.generate();
  const user = Keypair.generate();

  await utils.airdropSol(freezing.provider.connection, admin.publicKey, 200_000_000_000);
  await utils.airdropSol(freezing.provider.connection, user.publicKey, 200_000_000_000);
  await utils.airdropSol(freezing.provider.connection, updateAuth.publicKey, 200_000_000_000);

  const freezingParams = Keypair.generate();

  const ggwpToken = await utils.createMint(admin.publicKey, 9);
  const accumulativeFund = await utils.createTokenWallet(ggwpToken, admin.publicKey);
  const treasury = await utils.createTokenWallet(ggwpToken, admin.publicKey);
  const userGgwpTokenWallet = await utils.createTokenWallet(ggwpToken, user.publicKey);
  await utils.mintTokens(ggwpToken, admin, userGgwpTokenWallet, 100_000_000_000);

  const gpassSettings = Keypair.generate();
  const gpassMintAuth = findProgramAddressSync(
    [
      utf8.encode(utils.GPASS_MINT_AUTH_SEED),
      freezingParams.publicKey.toBytes(),
      gpassSettings.publicKey.toBytes(),
    ],
    freezing.programId
  )[0];

  await gpass.methods.initialize(
    new anchor.BN(30 * 60),
    updateAuth.publicKey,
    [gpassMintAuth],
    [])
    .accounts({
      admin: admin.publicKey,
      settings: gpassSettings.publicKey,
      systemProgram: SystemProgram.programId,
    })
    .signers([admin, gpassSettings])
    .rpc();

  const userGpassWallet = findProgramAddressSync(
    [
      utf8.encode(utils.USER_WALLET_SEED),
      gpassSettings.publicKey.toBytes(),
      user.publicKey.toBytes(),
    ],
    gpass.programId
  )[0];

  await gpass.methods.createWallet()
    .accounts({
      payer: admin.publicKey,
      user: user.publicKey,
      settings: gpassSettings.publicKey,
      wallet: userGpassWallet,
      systemProgram: SystemProgram.programId
    })
    .signers([admin])
    .rpc();

  return {
    admin: admin,
    updateAuth: updateAuth,

    freezing: {
      params: freezingParams,

      gpassSettings: gpassSettings,
      gpassMintAuth: gpassMintAuth,
      ggwpToken: ggwpToken,
      accumulativeFund: accumulativeFund,
      treasury: treasury,
    },
    user: {
      kp: user,
      ggwpWallet: userGgwpTokenWallet,
      gpassWallet: userGpassWallet,
    }
  }
}
