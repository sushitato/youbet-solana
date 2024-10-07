import * as anchor from "@coral-xyz/anchor";
import { Program, BN } from "@coral-xyz/anchor";
import { YoubetSolanaProgram } from "../target/types/youbet_solana_program";
import { assert } from "chai";

import {
  ComputeBudgetProgram,
  LAMPORTS_PER_SOL,
  Connection,
  Keypair,
  PublicKey,
  SystemProgram,
  SYSVAR_RENT_PUBKEY,
  Transaction,
  SYSVAR_SLOT_HASHES_PUBKEY,
  Commitment,
} from "@solana/web3.js";

const ADMIN_CONFIG_PREFIX: string = "ADMIN_CONFIG";
const PROJECT_PREFIX: string = "PROJECT";
const TASK_PREFIX: string = "TASK";
const PROJECT_USER_POINT_PREFIX: string = "PROJECT_USER_POINT";
const WALLET_PREFIX: string = "WALLET";
const GITHUB_PREFIX: string = "GITHUB";
const DONATE_POOL_PREFIX: string = "DONATE_POOL";
const REWARD_PREFIX: string = "REWARD";

const delayTimeCount = 1000;

function delay(ms: number) {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

describe("youbet-solana-program", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace
    .YoubetSolanaProgram as Program<YoubetSolanaProgram>;
  const feeAndRentKeypair: Keypair = Keypair.generate();
  let connection = anchor.AnchorProvider.env().connection;

  const project_id = "project_repo#githubid";
  const task_id = "task_repo#githubid";
  const github_id = "github_id";
  const wallet = feeAndRentKeypair.publicKey;

  function getAdminConfigAccountPdaAndBump(): [PublicKey, number] {
    return PublicKey.findProgramAddressSync(
      [Buffer.from(ADMIN_CONFIG_PREFIX)],
      program.programId
    );
  }
  function getProjectAccountPdaAndBump(
    project_id: string
  ): [PublicKey, number] {
    return PublicKey.findProgramAddressSync(
      [Buffer.from(PROJECT_PREFIX), Buffer.from(project_id)],
      program.programId
    );
  }
  function getTaskAccountPdaAndBump(task_id: string): [PublicKey, number] {
    return PublicKey.findProgramAddressSync(
      [Buffer.from(TASK_PREFIX), Buffer.from(task_id)],
      program.programId
    );
  }
  function getWalletAccountPdaAndBump(wallet: PublicKey): [PublicKey, number] {
    return PublicKey.findProgramAddressSync(
      [Buffer.from(WALLET_PREFIX), wallet.toBuffer()],
      program.programId
    );
  }
  function getGithubAccountPdaAndBump(github: string): [PublicKey, number] {
    return PublicKey.findProgramAddressSync(
      [Buffer.from(GITHUB_PREFIX), Buffer.from(github)],
      program.programId
    );
  }
  function getProjectUserPointPdaAndBump(
    project_id: string,
    wallet: PublicKey
  ): [PublicKey, number] {
    return PublicKey.findProgramAddressSync(
      [
        Buffer.from(PROJECT_USER_POINT_PREFIX),
        Buffer.from(project_id),
        wallet.toBuffer(),
      ],
      program.programId
    );
  }
  function getDonatePoolPdaAndBump(): [PublicKey, number] {
    return PublicKey.findProgramAddressSync(
      [Buffer.from(DONATE_POOL_PREFIX)],
      program.programId
    );
  }
  function getRewardPdaAndBump(wallet: PublicKey): [PublicKey, number] {
    return PublicKey.findProgramAddressSync(
      [Buffer.from(REWARD_PREFIX), wallet.toBuffer()],
      program.programId
    );
  }
  it("test all", async () => {
    await connection.requestAirdrop(
      feeAndRentKeypair.publicKey,
      20 * LAMPORTS_PER_SOL
    );
    await delay(delayTimeCount);

    // const account1: Keypair = Keypair.generate();
    // await connection.requestAirdrop(
    //   feeAndRentKeypair.publicKey,
    //   20 * LAMPORTS_PER_SOL
    // );
    // await delay(delayTimeCount);
    // const account2: Keypair = Keypair.generate();
    // await connection.requestAirdrop(
    //   feeAndRentKeypair.publicKey,
    //   20 * LAMPORTS_PER_SOL
    // );
    // await delay(delayTimeCount);

    //////////////////////////
    const [adminConfig, adminBump] = getAdminConfigAccountPdaAndBump();
    let initializeAccounts = {
      feeAndRentPayer: feeAndRentKeypair.publicKey,
      authority: feeAndRentKeypair.publicKey,
      adminConfig,
      systemProgram: new PublicKey("11111111111111111111111111111111"),
      rent: SYSVAR_RENT_PUBKEY,
    };
    const tx0 = await program.methods
      .initialize()
      .accounts(initializeAccounts)
      .signers([feeAndRentKeypair, feeAndRentKeypair])
      .rpc();
    console.log("tx0 signature", tx0);
    const adminConfigData = await program.account.adminConfigAccount.fetch(
      adminConfig.toBase58()
    );
    console.log(adminConfigData);
    ////
    const [project, projectBump] = getProjectAccountPdaAndBump(project_id);
    let createProjectAccounts = {
      feeAndRentPayer: feeAndRentKeypair.publicKey,
      project,
      systemProgram: new PublicKey("11111111111111111111111111111111"),
      rent: SYSVAR_RENT_PUBKEY,
    };
    const tx = await program.methods
      .createProject(project_id, project_id)
      .accounts(createProjectAccounts)
      .signers([feeAndRentKeypair])
      .rpc();
    console.log("tx signature", tx);
    const projectData = await program.account.projectAccount.fetch(
      project.toBase58()
    );
    console.log(projectData);
    assert(projectData.projectId == project_id, "1");
    assert(projectData.name == project_id, "2");
    //////////////////
    const [task, taskBump] = getTaskAccountPdaAndBump(task_id);
    let createTaskAccounts = {
      feeAndRentPayer: feeAndRentKeypair.publicKey,
      task,
      project,
      systemProgram: new PublicKey("11111111111111111111111111111111"),
      rent: SYSVAR_RENT_PUBKEY,
    };
    const tx1 = await program.methods
      .createTask(task_id, task_id, project_id, projectBump)
      .accounts(createTaskAccounts)
      .signers([feeAndRentKeypair])
      .rpc();
    console.log("tx1 signature", tx1);
    const data1 = await program.account.taskAccount.fetch(task.toBase58());
    console.log(data1);
    ////////////////////////////
    const [walletAccount, walletBump] = getWalletAccountPdaAndBump(wallet);
    const [githubAccount, githubBump] = getGithubAccountPdaAndBump(github_id);

    let createLinkWalletAccounts = {
      feeAndRentPayer: feeAndRentKeypair.publicKey,
      adminConfig,
      walletAccount,
      githubAccount,
      systemProgram: new PublicKey("11111111111111111111111111111111"),
      rent: SYSVAR_RENT_PUBKEY,
    };
    const tx2 = await program.methods
      .linkWallet(wallet, github_id, adminBump)
      .accounts(createLinkWalletAccounts)
      .signers([feeAndRentKeypair])
      .rpc();
    console.log("tx2 signature", tx2);
    const data2 = await program.account.walletAccount.fetch(
      walletAccount.toBase58()
    );
    console.log(data2);
    const data22 = await program.account.githubAccount.fetch(
      githubAccount.toBase58()
    );
    console.log(data22);
    ///////////////////////
    const [projectUserPoint, projectUserPointBump] =
      getProjectUserPointPdaAndBump(project_id, wallet);
    const confirmTaskAccounts = {
      feeAndRentPayer: feeAndRentKeypair.publicKey,
      task,
      project,
      githubAccount,
      walletAccount,
      projectUserPoint,
      systemProgram: new PublicKey("11111111111111111111111111111111"),
      rent: SYSVAR_RENT_PUBKEY,
    };
    const tx3 = await program.methods
      .confirmTask(task_id, github_id, 10, taskBump, githubBump, walletBump)
      .accounts(confirmTaskAccounts)
      .signers([feeAndRentKeypair])
      .rpc();
    console.log("tx3 signature", tx3);
    const data3 = await program.account.projectUserPointAccount.fetch(
      projectUserPoint.toBase58()
    );
    console.log(data3);
    const projectData1 = await program.account.projectAccount.fetch(
      project.toBase58()
    );
    console.log(projectData1);
    const taskData1 = await program.account.taskAccount.fetch(task.toBase58());
    console.log(taskData1);
    ////////////////////////////
    const [donatePoolPda, donatePoolBump] = getDonatePoolPdaAndBump();
    const [rewardPda, rewardBump] = getRewardPdaAndBump(
      feeAndRentKeypair.publicKey
    );
    let donateProjectAccounts = {
      feeAndRentPayer: feeAndRentKeypair.publicKey,
      project,
      donatePool: donatePoolPda,
      projectUserPoint1: projectUserPoint,
      reward1: rewardPda,
      systemProgram: new PublicKey("11111111111111111111111111111111"),
      rent: SYSVAR_RENT_PUBKEY,
    };
    const tx4 = await program.methods
      .donateToProject(
        new BN(LAMPORTS_PER_SOL),
        project_id,
        projectBump,
        donatePoolBump,
        feeAndRentKeypair.publicKey,
        rewardBump,
        projectUserPointBump,
        feeAndRentKeypair.publicKey,
        rewardBump,
        projectUserPointBump,
        feeAndRentKeypair.publicKey,
        rewardBump,
        projectUserPointBump
      )
      .accounts(donateProjectAccounts)
      .signers([feeAndRentKeypair])
      .rpc();
    console.log("tx4 signature", tx4);
    const poolBalance = await connection.getBalance(donatePoolPda);
    console.log(poolBalance);
    const currentBalance = await connection.getBalance(
      feeAndRentKeypair.publicKey
    );
    console.log(currentBalance);
    /////////////////
    let claimRewardAccounts = {
      feeAndRentPayer: feeAndRentKeypair.publicKey,
      donatePool: donatePoolPda,
      reward: rewardPda,
      systemProgram: new PublicKey("11111111111111111111111111111111"),
      rent: SYSVAR_RENT_PUBKEY,
    };
    const tx5 = await program.methods
      .claimReward(donatePoolBump, rewardBump)
      .accounts(claimRewardAccounts)
      .signers([feeAndRentKeypair])
      .rpc();

    console.log("tx5 signature", tx5);
    const currentBalance1 = await connection.getBalance(
      feeAndRentKeypair.publicKey
    );
    console.log(currentBalance1);
  });
});
