import Head from "next/head";
import Image from "next/image";
import Link from "next/link";
import styles from "../styles/Home.module.css";

export default function Home() {
  return (
    <div className={styles.container}>
      <Head>
        <title>Home</title>
        <meta name="description" content="Home page of multi-chain amm" />
        <link rel="icon" href="/favicon.ico" />
      </Head>

      <main className={styles.main}>
        <h1 className={styles.title}>Welcome to Multi chain amm</h1>

        <div className={styles.grid}>
          the idea is to have an efficient AMM (auto market maker) that can
          support deposit and withdrawal on mutiple chains
        </div>

        <div className={styles.grid}>
          the current implementation is on the Solana chain and demonstarates a
          simple case where ethereum deposit and withrawal is supported
        </div>

        <div className={styles.grid}>
          Once we have a working prototype we will add support for other chains
          like Binance, Polygon, Avalanche, etc
        </div>

        <div className={styles.grid}>
          Once we have an efficient chain like solana it's no longer relevant to
          build an exchange in the traditional sense Instead we can build a
          simple AMM that can support deposit and withdrawal on multiple chains
          with Help of{" "}
          <a className={styles.code} href="https://wormhole.com/">
            Wormhole
          </a>
        </div>

        <div className={styles.grid}>
          The logic to support deposit and withdrawal is implemented in the
          following files
          <ul>
            <li style={{ margin: "2rem auto" }}>
              <Link className={styles.code} href="/eth-to-solana">
                Eth to solana deposit
              </Link>
            </li>
            <li style={{ margin: "2rem auto" }}>
              <Link className={styles.code} href="/solana-to-eth">
                Solana to Eth withdraw
              </Link>
            </li>
          </ul>
        </div>
      </main>
    </div>
  );
}
