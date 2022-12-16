import Head from "next/head";
import Image from "next/image";
import Link from "next/link";
import styles from "../styles/Home.module.css";
import { CodeBlock, dracula } from "react-code-blocks";

export default function Home() {
  let code = ` 
        // determine destination address - an associated token
        account const solanaMintKey = new PublicKey( (await
        getForeignAssetSolana( connection, SOLANA_TOKEN_BRIDGE_ADDRESS,
        CHAIN_ID_ETH, hexToUint8Array(nativeToHexString(tokenAddress,
        CHAIN_ID_ETH) || "") )) || "" ); const recipientAddress = await
        Token.getAssociatedTokenAddress( ASSOCIATED_TOKEN_PROGRAM_ID,
        TOKEN_PROGRAM_ID, solanaMintKey, walletAddress ); // Submit transaction
        - results in a Wormhole message being published const receipt = await
        transferFromEth( ETH_TOKEN_BRIDGE_ADDRESS, signer, tokenAddress, amount,
        CHAIN_ID_SOLANA, recipientAddress ); // Get the sequence number and
        emitter address required to fetch the signedVAA of our message const
        sequence = parseSequenceFromLogEth(receipt, ETH_BRIDGE_ADDRESS); const
        emitterAddress = getEmitterAddressEth(ETH_TOKEN_BRIDGE_ADDRESS); //
        Fetch the signedVAA from the Wormhole Network (this may require retries
        while you wait for confirmation) const {signedVAA} = await getSignedVAA(
        WORMHOLE_RPC_HOST, CHAIN_ID_ETH, emitterAddress, sequence ); // On
        Solana, we have to post the signedVAA ourselves await postVaaSolana(
        connection, wallet, SOL_BRIDGE_ADDRESS, payerAddress, signedVAA ); //
        Finally, redeem on Solana const transaction = await redeemOnSolana(
        connection, SOL_BRIDGE_ADDRESS, SOL_TOKEN_BRIDGE_ADDRESS, payerAddress,
        signedVAA, isSolanaNative, mintAddress ); const signed = await
        wallet.signTransaction(transaction); const txid = await
        connection.sendRawTransaction(signed.serialize()); await
        connection.confirmTransaction(txid); 
        `;

  return (
    <div className={styles.container}>
      <Head>
        <title>Home</title>
        <meta name="description" content="Home page of multi-chain amm" />
        <link rel="icon" href="/favicon.ico" />
      </Head>

      <main className={styles.main}>
        <h1 className={styles.title}>Eth to Solana deposit logic</h1>

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
