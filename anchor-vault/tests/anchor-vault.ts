import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AnchorVault } from "../target/types/anchor_vault";
import { PublicKey, Keypair, SystemProgram } from "@solana/web3.js";

describe("anchor-vault", () => {
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.AnchorVault as Program<AnchorVault>;

  const provider = anchor.getProvider();

  const signer = new Keypair();

  before(async () => {
    const tx = await provider.connection.requestAirdrop(
     signer.publicKey,
      1_000_000_000
    );

    await provider.connection.confirmTransaction(tx, "confirmed");

    const signerBal = await provider.connection.getBalance(signer.publicKey);
  })

  it("init", async () => {

    const vaultState = PublicKey.findProgramAddressSync(
    [Buffer.from("state"), signer.publicKey.toBuffer()],
    program.programId
  )[0];

  const vault = PublicKey.findProgramAddressSync(
    [Buffer.from("vault"), vaultState.toBuffer()],
    program.programId
  )[0];

    const tx = await program.methods.init()
    .accounts({
      signer: signer.publicKey,
      vaultState,
      vault,
      systemProgram: SystemProgram.programId
    })
    .signers([signer])
    .rpc();
    console.log("transaction signature", tx);
  });
});
