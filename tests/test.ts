import * as anchor from "@coral-xyz/anchor";
import { AnchorProgramExample } from "../target/types/anchor_program_example";
import { PublicKey } from "@solana/web3.js";
import { expect } from "chai";
import { sleep } from "../utils/sleep";

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
      .rpc({
        skipPreflight: true,
      });

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
    await sleep(500);
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
  // it("Create 3rd loan", async () => {
  //   const id = Math.floor(Math.random() * 100000);
  //   await program.methods
  //     .createNftLoan(
  //       id,
  //       2,
  //       new anchor.BN(6.14 * AMOUNT_MULTIPLIER),
  //       new anchor.BN(16),
  //       new anchor.BN(11)
  //     )
  //     .accounts({
  //       payer: payer.publicKey,
  //       loan: loanPda,
  //     })
  //     .rpc();
  //   loanIds.push(id);

  //   const loan = await fetchLoan();
  //   expect(loan.loans.filter((l) => l !== null).length).to.equal(2);
  // });

  const tomek = anchor.web3.Keypair.generate();

  before(async () => {
    // //AirDrop
    await provider.connection.requestAirdrop(tomek.publicKey, 10000000000);
    await sleep(500);
  });

  it("accept loan and repay", async () => {
    const loans_prev = await fetchLoan();
    let id = -1;

    for (let i = 0; i < loans_prev.loans.length; i++) {
      if (loans_prev.loans[i] !== null) {
        id = loans_prev.loans[i].loanId;
        break;
      }
    }

    await program.methods
      .acceptOffer(id)
      .accounts({
        borrower: payer.publicKey,
        payer: tomek.publicKey,
        loan: loanPda,
      })
      .signers([tomek])
      .rpc();
    await sleep(500);
    const loan = await fetchLoan();

    await program.methods
      .repayLoan(id, new anchor.BN(3 * AMOUNT_MULTIPLIER))
      .accounts({
        payer: payer.publicKey,
        loan: loanPda,
      })
      .rpc();
    await sleep(500);
    const fetched = await fetchLoan();
    const loans = fetched.loans.filter((l) => l !== null);

    console.log("CURRENT LOANS after repaying", loans);
  });
});
