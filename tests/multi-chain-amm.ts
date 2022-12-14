import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { MultiChainAmm } from "../target/types/multi_chain_amm";

describe("multi-chain-amm", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.MultiChainAmm as Program<MultiChainAmm>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
