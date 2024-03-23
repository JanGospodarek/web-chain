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
    const loan = await fetchLoan();
    expect(loan.loanCount).to.equal(0);
  });

  it("Create loan pda", async () => {
    await program.methods
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
    expect(loan.loanCount).to.equal(1);
    expect(loan.loans[0].reqAmount.toNumber()).to.equal(
      5.14 * AMOUNT_MULTIPLIER
    );
  });
});
