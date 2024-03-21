import * as anchor from "@coral-xyz/anchor";
import { AnchorProgramExample } from "../target/types/anchor_program_example";
import { PublicKey } from "@solana/web3.js";

describe("PDAs", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const payer = provider.wallet as anchor.Wallet;
  const program = anchor.workspace
    .AnchorProgramExample as anchor.Program<AnchorProgramExample>;

  const [loanPda] = PublicKey.findProgramAddressSync(
    [Buffer.from("seed_one"), payer.publicKey.toBuffer()],
    program.programId
  );

  it("Create loan pda", async () => {
    await program.methods
      .createNftLoan(
        0,
        new anchor.BN(5.14),
        new anchor.BN(15),
        new anchor.BN(10),
        "seed_one"
      )
      .accounts({
        payer: payer.publicKey,
        loan: loanPda,
      })
      .rpc();
  });

  it("View pda data", async () => {
    const pageVisits = await program.account.nftLoan.fetch(loanPda);
    console.log(`Period:${pageVisits.period.toNumber()}`);
    console.log(`Interest:${pageVisits.interest.toNumber()}`);
    console.log(`Amount:${pageVisits.reqAmount.toNumber()}`);
    console.log(`NFT:${pageVisits.nftId}`);
  });
});
// it("Visit the page!", async () => {
//   await program.methods
//     .incrementPageVisits()
//     .accounts({
//       user: payer.publicKey,
//       loan: pageVisitPDA,
//     })
//     .rpc();
// });
