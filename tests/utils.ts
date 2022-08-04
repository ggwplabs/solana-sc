import {
    PublicKey, Connection
} from "@solana/web3.js";

export async function airdropSol(conn: Connection, to: PublicKey, amount: number) {
    const airdropSignature = await conn.requestAirdrop(to, amount);
    const latestBlockHash = await conn.getLatestBlockhash();
    await conn.confirmTransaction({
        blockhash: latestBlockHash.blockhash,
        lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
        signature: airdropSignature,
    });
}

export function currentTimestamp(): number {
    return Math.floor(Date.now() / 1000);
}

export function assertTimestamps(ts1: number, ts2: number, precision?: number): boolean {
    if (precision) {
        console.log(Math.floor(ts1 / precision));
        console.log(Math.floor(ts2 / precision));
        return Math.floor(ts1 / precision) == Math.floor(ts2 / precision);
    }
    else {
        return ts1 == ts2;
    }
}
