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
  const [userInfo] = PublicKey.findProgramAddressSync(
    [Buffer.from("user_info_seed"), payer.publicKey.toBuffer()],
    program.programId
  );
  const loanIds = [];
  const fetchLoan = async () => await program.account.loanPda.fetch(loanPda);
  it("Initialize acc", async () => {
    await program.methods
      .init()
      .accounts({
        loan: loanPda,
        userInfo: userInfo,
        payer: payer.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    const info = await program.account.userInfo.fetch(userInfo);
    expect(info.trustScore).to.equal(100);
  });

  it("Create loan", async () => {
    const id = Math.floor(Math.random() * 100000);

    const tx = await program.methods
      .createNftLoan(
        id,
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
    loanIds.push(id);

    const loan = await fetchLoan();
    console.log("CURRENT LOANS after 1st adding", loan.loans);
    expect(loan.loans.filter((l) => l !== null).length).to.equal(1);
  });
  it("Create second loan", async () => {
    const id = Math.floor(Math.random() * 100000);
    await program.methods
      .createNftLoan(
        id,
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
    loanIds.push(id);
    const loan = await fetchLoan();
    expect(loan.loans.filter((l) => l !== null).length).to.equal(2);
  });
  it("delete loan", async () => {
    const id = loanIds[0];
    await program.methods
      .destroyLoan(id)
      .accounts({
        payer: payer.publicKey,
        loan: loanPda,
      })
      .rpc();
    const loan = await fetchLoan();
    expect(loan.loans.filter((l) => l !== null).length).to.equal(1);
  });
  it("Create 3rd loan", async () => {
    const id = Math.floor(Math.random() * 100000);
    await program.methods
      .createNftLoan(
        id,
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
    loanIds.push(id);

    const loan = await fetchLoan();
    expect(loan.loans.filter((l) => l !== null).length).to.equal(2);
  });
});
