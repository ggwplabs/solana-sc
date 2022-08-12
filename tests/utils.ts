import * as anchor from "@project-serum/anchor";
import {
    PublicKey, Connection, Keypair, SYSVAR_RENT_PUBKEY
} from "@solana/web3.js";

const tokenProgram = anchor.Spl.token();

export const GPASS_MINT_AUTH_SEED = "gpass_mint_auth";
export const TREASURY_AUTH_SEED = "treasury_auth";
export const USER_INFO_SEED = "user_info";
export const USER_WALLET_SEED = "user_gpass_wallet";

export async function airdropSol(conn: Connection, to: PublicKey, amount: number) {
    const airdropSignature = await conn.requestAirdrop(to, amount);
    const latestBlockHash = await conn.getLatestBlockhash();
    await conn.confirmTransaction({
        blockhash: latestBlockHash.blockhash,
        lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
        signature: airdropSignature,
    });
}

export async function sleep(seconds: number) {
    return new Promise(resolve => setTimeout(resolve, seconds * 1000));
}

export function currentTimestamp(): number {
    return Math.floor(Date.now() / 1000);
}

export function assertWithPrecission(val1: number, val2: number, precision?: number): boolean {
    if (precision) {
        return Math.abs(val1 - val2) <= precision;
    }
    else {
        return val1 == val2;
    }
}

export async function createMint(authority: PublicKey, decimals: number): Promise<PublicKey> {
    const mintKP = Keypair.generate();
    await tokenProgram.methods.initializeMint(decimals, authority, null)
        .accounts({
            mint: mintKP.publicKey,
            rent: SYSVAR_RENT_PUBKEY,
        })
        .signers([mintKP])
        .preInstructions([await tokenProgram.account.mint.createInstruction(mintKP)])
        .rpc();

    return mintKP.publicKey;
}

export async function createTokenWallet(mint: PublicKey, authority: PublicKey): Promise<PublicKey> {
    const wallet = Keypair.generate();
    await tokenProgram.methods.initializeAccount()
        .accounts({
            account: wallet.publicKey,
            mint: mint,
            authority: authority,
            rent: SYSVAR_RENT_PUBKEY,
        })
        .signers([wallet])
        .preInstructions([await tokenProgram.account.token.createInstruction(wallet)])
        .rpc();

    return wallet.publicKey;
}

export async function mintTokens(mint: PublicKey, authority: Keypair, wallet: PublicKey, amount: number): Promise<void> {
    await tokenProgram.methods.mintTo(new anchor.BN(amount))
        .accounts({
            mint: mint,
            authority: authority.publicKey,
            to: wallet,
        })
        .signers([authority])
        .rpc();
}

export async function getTokenBalance(wallet: PublicKey): Promise<number> {
    const walletData = await tokenProgram.account.token.fetch(wallet);
    return walletData.amount.toNumber();
}

export function calcRoyaltyAmount(amount: number, royalty: number): number {
    return amount / 100 * royalty;
}
