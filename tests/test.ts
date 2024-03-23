import * as anchor from "@coral-xyz/anchor";
import { AnchorProgramExample } from "../target/types/anchor_program_example";
import { PublicKey } from "@solana/web3.js";
import { expect } from "chai";

const AMOUNT_MULTIPLIER = 1000000;
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
    await program.methods
      .init()
      .accounts({
        loan: loanPda,
        payer: payer.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();
  });

  it("Create loan", async () => {
    const tx = await program.methods
      .createNftLoan(
        0,
        new anchor.BN(5.14 * AMOUNT_MULTIPLIER),
        new anchor.BN(15),
        new anchor.BN(10)
      )
      .accounts({
        payer: payer.publicKey,
        loan: loanPda,
      })
      .rpc();
    const loan = await fetchLoan();
    console.log("CURRENT LOANS after 1st adding", loan.loans);
    expect(loan.loans.filter((l) => l !== null).length).to.equal(1);
  });
  it("Create second loan", async () => {
    await program.methods
      .createNftLoan(
        1,
        new anchor.BN(6.14 * AMOUNT_MULTIPLIER),
        new anchor.BN(16),
        new anchor.BN(11)
      )
      .accounts({
        payer: payer.publicKey,
        loan: loanPda,
      })
      .rpc();
    const loan = await fetchLoan();
    expect(loan.loans.filter((l) => l !== null).length).to.equal(2);
  });
  it("delete loan", async () => {
    await program.methods
      .destroyLoan(0)
      .accounts({
        payer: payer.publicKey,
        loan: loanPda,
      })
      .rpc();
    const loan = await fetchLoan();
    expect(loan.loans.filter((l) => l !== null).length).to.equal(1);
  });
  it("Create 3rd loan", async () => {
    await program.methods
      .createNftLoan(
        2,
        new anchor.BN(6.14 * AMOUNT_MULTIPLIER),
        new anchor.BN(16),
        new anchor.BN(11)
      )
      .accounts({
        payer: payer.publicKey,
        loan: loanPda,
      })
      .rpc();
    const loan = await fetchLoan();
    expect(loan.loans.filter((l) => l !== null).length).to.equal(2);
  });
});
