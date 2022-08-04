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