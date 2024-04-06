import * as anchor from "@coral-xyz/anchor";
import { AnchorProgramExample } from "../target/types/anchor_program_example";
import { PublicKey } from "@solana/web3.js";
import { expect } from "chai";
import { sleep } from "../utils/sleep";

const AMOUNT_MULTIPLIER = 1000000;
describe("PDAs", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const payer = anchor.web3.Keypair.generate();

  const program = anchor.workspace
    .AnchorProgramExample as anchor.Program<AnchorProgramExample>;

  const [loanPda] = PublicKey.findProgramAddressSync(
    [Buffer.from("prefix_loan_seed"), payer.publicKey.toBuffer()],
    program.programId
  );
  const [userInfo] = PublicKey.findProgramAddressSync(
    [Buffer.from("user_info_seed"), payer.publicKey.toBuffer()],
    program.programId
  );
  const tomek = anchor.web3.Keypair.generate();

  before(async () => {
    // //AirDrop
    await provider.connection.requestAirdrop(tomek.publicKey, 10000000000);
    await provider.connection.requestAirdrop(payer.publicKey, 10000000000);
    await sleep(500);
  });

  const [tomeksLoanPda] = PublicKey.findProgramAddressSync(
    [Buffer.from("prefix_loan_seed"), tomek.publicKey.toBuffer()],
    program.programId
  );
  const [tomeksUserInfo] = PublicKey.findProgramAddressSync(
    [Buffer.from("user_info_seed"), tomek.publicKey.toBuffer()],
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
      })
      .signers([payer])
      .rpc({
        skipPreflight: true,
      });

    const info = await program.account.userInfo.fetch(userInfo);
    const pda = await fetchLoan();
    console.log("SPACE", pda);
    console.log("USER INFO", info);
    expect(info.trustScore).to.equal(100);
  });

  it("Create loan", async () => {
    const id = Math.floor(Math.random() * 100000);

    await program.methods
      .createNftLoan(
        id,
        0,
        new anchor.BN(5.14 * AMOUNT_MULTIPLIER),
        new anchor.BN(15),
        new anchor.BN(Date.now())
      )
      .accounts({
        payer: payer.publicKey,
        loan: loanPda,
      })
      .signers([payer])

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
      .signers([payer])

      .rpc();
    loanIds.push(id);
    const loan = await fetchLoan();
    expect(loan.loans.filter((l) => l !== null).length).to.equal(2);
    await sleep(500);
  });
  it("delete loan", async () => {
    const id = loanIds[1];
    await program.methods
      .destroyLoan(id)
      .accounts({
        payer: payer.publicKey,
        loan: loanPda,
      })
      .signers([payer])

      .rpc();
    const loan = await fetchLoan();
    expect(loan.loans.filter((l) => l !== null).length).to.equal(1);
  });

  it("Initialize tomeks acc", async () => {
    await program.methods
      .init()
      .accounts({
        loan: tomeksLoanPda,
        userInfo: tomeksUserInfo,
        payer: tomek.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([tomek])
      .rpc({
        skipPreflight: true,
      });

    const info = await program.account.userInfo.fetch(tomeksUserInfo);
    expect(info.trustScore).to.equal(100);
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
    // Accept loan
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

    // Repay loan
    await program.methods
      .repayLoan(id, new anchor.BN(3 * AMOUNT_MULTIPLIER))
      .accounts({
        payer: tomek.publicKey,
        loan: loanPda,
        borrowerUserInfo: userInfo,
      })
      .signers([tomek])
      .rpc();
    await sleep(500);
  });
  it("repay rest of the loan", async () => {
    const loans_prev = await fetchLoan();
    let id = -1;

    for (let i = 0; i < loans_prev.loans.length; i++) {
      if (loans_prev.loans[i] !== null) {
        id = loans_prev.loans[i].loanId;
        break;
      }
    }
    // Repay loan
    await program.methods
      .repayLoan(id, new anchor.BN(2 * AMOUNT_MULTIPLIER))
      .accounts({
        payer: tomek.publicKey,
        loan: loanPda,
        borrowerUserInfo: userInfo,
      })
      .signers([tomek])
      .rpc();
    await sleep(500);
    const fetched = await fetchLoan();
    const borrowerUserInfo = await program.account.userInfo.fetch(userInfo);
    const loans = fetched.loans.filter((l) => l !== null);
    console.log("BORROWER USER INFO trustscore:", borrowerUserInfo.trustScore);
    console.log("CURRENT LOANS after repaying", loans);
  });
});
