import { Program } from "@project-serum/anchor";
import { findProgramAddressSync } from "@project-serum/anchor/dist/cjs/utils/pubkey";
import { utf8 } from "@project-serum/anchor/dist/cjs/utils/bytes";
import { Keypair, PublicKey } from "@solana/web3.js";
import { Staking } from "../../target/types/staking";
import * as utils from "../utils";

export class StakingTestFixture {
    admin: Keypair;
    updateAuth: Keypair;

    staking: {
        info: Keypair;
        ggwpToken: PublicKey;
        stakingFund: PublicKey;
        stakingFundAuth: PublicKey;
        accumulativeFund: PublicKey;
        treasury: PublicKey;
        treasuryAuth: PublicKey,
    }

    user: {
        kp: Keypair;
        info: PublicKey;
        ggwpWallet: PublicKey;
    }
}

export async function prepareStakingTestFixture(staking: Program<Staking>): Promise<StakingTestFixture> {
    const admin = Keypair.generate();
    const updateAuth = Keypair.generate();
    const user = Keypair.generate();

    await utils.airdropSol(staking.provider.connection, admin.publicKey, 200_000_000_000);
    await utils.airdropSol(staking.provider.connection, user.publicKey, 200_000_000_000);
    await utils.airdropSol(staking.provider.connection, updateAuth.publicKey, 200_000_000_000);

    const stakingInfo = Keypair.generate();

    const ggwpToken = await utils.createMint(admin.publicKey, 9);
    const accumulativeFund = await utils.createTokenWallet(ggwpToken, admin.publicKey);
    const treasuryAuth = findProgramAddressSync(
        [
            utf8.encode(utils.TREASURY_AUTH_SEED),
            stakingInfo.publicKey.toBytes(),
        ],
        staking.programId,
    )[0];
    const treasury = await utils.createTokenWallet(ggwpToken, treasuryAuth);
    const stakingFundAuth = findProgramAddressSync(
        [
            utf8.encode(utils.STAKING_FUND_AUTH_SEED),
            stakingInfo.publicKey.toBytes(),
        ],
        staking.programId,
    )[0];
    const stakingFund = await utils.createTokenWallet(ggwpToken, stakingFundAuth);

    const userGgwpTokenWallet = await utils.createTokenWallet(ggwpToken, user.publicKey);
    await utils.mintTokens(ggwpToken, admin, userGgwpTokenWallet, 10000_000_000_000);

    const userInfo = findProgramAddressSync(
        [
            utf8.encode(utils.USER_INFO_SEED),
            stakingInfo.publicKey.toBytes(),
            user.publicKey.toBytes(),
        ],
        staking.programId,
    )[0];

    return {
        admin: admin,
        updateAuth: updateAuth,

        staking: {
            info: stakingInfo,
            ggwpToken: ggwpToken,
            stakingFund: stakingFund,
            stakingFundAuth: stakingFundAuth,
            accumulativeFund: accumulativeFund,
            treasury: treasury,
            treasuryAuth: treasuryAuth,
        },
        user: {
            kp: user,
            info: userInfo,
            ggwpWallet: userGgwpTokenWallet,
        }
    }
}
