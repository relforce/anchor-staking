import { BN, web3 } from "@project-serum/anchor";
import { WMP_STAKING_PROGRAM_ID } from "./program-id";

export async function calculateGlobalDataPda(programId: web3.PublicKey = WMP_STAKING_PROGRAM_ID) {
    const prefix = "global-data";
    let seeds = [
        Buffer.from(prefix, "utf-8")
    ];
    return await web3.PublicKey.findProgramAddress(seeds, programId);
}

export async function calculateStakePoolPda(id: number, programId: web3.PublicKey = WMP_STAKING_PROGRAM_ID) {
    const prefix = "stake-pool";
    let seeds = [
        Buffer.from(prefix, "utf-8"),
        new BN(id).toArrayLike(Buffer, "le", 2)
    ];
    return await web3.PublicKey.findProgramAddress(seeds, programId);
}