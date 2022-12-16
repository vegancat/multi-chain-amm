import Head from "next/head";
import Image from "next/image";
import Link from "next/link";
import styles from "../styles/Home.module.css";
import { CodeBlock, dracula } from "react-code-blocks";

export default function Home() {
  let code = `
      // Submit transaction - results in a Wormhole message being published
      const transaction = await transferFromSolana(
      connection,
      SOL_BRIDGE_ADDRESS,
      SOL_TOKEN_BRIDGE_ADDRESS,
      payerAddress,
      fromAddress,
      mintAddress,
      amount,
      targetAddress,
      CHAIN_ID_ETH,
      originAddress,
      originChain
    );
    const signed = await wallet.signTransaction(transaction);
    const txid = await connection.sendRawTransaction(signed.serialize());
    await connection.confirmTransaction(txid);
    // Get the sequence number and emitter address required to fetch the signedVAA of our message
    const info = await connection.getTransaction(txid);
    const sequence = parseSequenceFromLogSolana(info);
    const emitterAddress = await getEmitterAddressSolana(SOL_TOKEN_BRIDGE_ADDRESS);
    // Fetch the signedVAA from the Wormhole Network (this may require retries while you wait for confirmation)
    const { signedVAA } = await getSignedVAA(
      WORMHOLE_RPC_HOST,
      CHAIN_ID_SOLANA,
      emitterAddress,
      sequence
    );
    // Redeem on Ethereum
    await redeemOnEth(ETH_TOKEN_BRIDGE_ADDRESS, signer, signedVAA);
`;
  return (
    <div className={styles.container}>
      <Head>
        <title>Home</title>
        <meta name="description" content="Home page of multi-chain amm" />
        <link rel="icon" href="/favicon.ico" />
      </Head>

      <main className={styles.main}>
        <h1 className={styles.title}>Solana to Eth withdraw logic</h1>

        <CodeBlock
          text={code}
          language={"javascript"}
          showLineNumbers
          theme={dracula}
        />
      </main>
    </div>
  );
}
