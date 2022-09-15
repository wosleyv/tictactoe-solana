import * as anchor from "@project-serum/anchor";

export const fundWallet = async (
  connection: anchor.web3.Connection,
  wallet: anchor.web3.PublicKey,
  amount: number
) => {
  try {
    const airdropSignature = await connection.requestAirdrop(
      wallet,
      amount * anchor.web3.LAMPORTS_PER_SOL
    );

    const latestBlockHash = await connection.getLatestBlockhash();

    await connection.confirmTransaction({
      blockhash: latestBlockHash.blockhash,
      lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
      signature: airdropSignature,
    });
  } catch (error) {
    console.error(error);
  }
};
