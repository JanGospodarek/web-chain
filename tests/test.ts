import * as anchor from "@coral-xyz/anchor";
import { AnchorProgramExample } from "../target/types/anchor_program_example";
import { PublicKey } from "@solana/web3.js";
import { expect } from "chai";

describe("PDAs", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const payer = provider.wallet as anchor.Wallet;
  const program = anchor.workspace
    .AnchorProgramExample as anchor.Program<AnchorProgramExample>;

  const [loanPda] = PublicKey.findProgramAddressSync(
    [Buffer.from("loan_seed"), payer.publicKey.toBuffer()],
    program.programId
  );

  const fetchLoan = async () => await program.account.loanPda.fetch(loanPda);
  it("Initialize acc", async () => {
    const tx = await program.methods
      .init()
      .accounts({
        loan: loanPda,
        payer: payer.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();
    const loan = await fetchLoan();
    expect(loan.loanCount).to.equal(0);
  });

  // it("Create loan pda", async () => {
  //   await program.methods
  //     .createNftLoan(
  //       0,
  //       new anchor.BN(5.14),
  //       new anchor.BN(15),
  //       new anchor.BN(10)
  //     )
  //     .accounts({
  //       payer: payer.publicKey,
  //       loan: loanPda,
  //     })
  //     .rpc();
  // });

  it("View pda data", async () => {
    const pageVisits = ;
    console.log(pageVisits.loanCount);
    // console.log(`Period:${pageVisits[0].period.toNumber()}`);
    // console.log(`Interest:${pageVisits[0].interest.toNumber()}`);
    // console.log(`Amount:${pageVisits[0].reqAmount.toNumber()}`);
    // console.log(`NFT:${pageVisits[0].nftId}`);
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
