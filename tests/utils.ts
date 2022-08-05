import {
    PublicKey, Connection
} from "@solana/web3.js";

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

export function assertTimestamps(ts1: number, ts2: number, precision?: number): boolean {
    if (precision) {
        return Math.abs(ts1 - ts2) <= precision;
    }
    else {
        return ts1 == ts2;
    }
}
