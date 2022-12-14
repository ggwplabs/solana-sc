import * as anchor from "@project-serum/anchor";
import { AnchorError, Program } from "@project-serum/anchor";
import { findProgramAddressSync } from "@project-serum/anchor/dist/cjs/utils/pubkey";
import { utf8 } from "@project-serum/anchor/dist/cjs/utils/bytes";
import {
  Keypair, SystemProgram, LAMPORTS_PER_SOL
} from "@solana/web3.js";
import { Gpass } from "../../target/types/gpass";
import * as assert from "assert";
import * as utils from "../utils";

describe("GPASS functional tests", () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.Gpass as Program<Gpass>;

  const admin = Keypair.generate();
  const updateAuth = Keypair.generate();
  const mintersPK = [];
  const minters = [Keypair.generate()];
  minters.forEach((item) => {
    mintersPK.push(item.publicKey);
  });
  const burnersPK = [];
  const burners = [Keypair.generate(), Keypair.generate()];
  burners.forEach((item) => {
    burnersPK.push(item.publicKey);
  });
  const burnPeriod = 5;

  const user1 = Keypair.generate();
  let user1WalletPK;
  const user2 = Keypair.generate();
  let user2WalletPK;

  const gpassInfo = Keypair.generate();
  before(async () => {
    await utils.airdropSol(program.provider.connection, user1.publicKey, 100 * LAMPORTS_PER_SOL);
    await utils.airdropSol(program.provider.connection, user2.publicKey, 100 * LAMPORTS_PER_SOL);
    await utils.airdropSol(program.provider.connection, admin.publicKey, 100 * LAMPORTS_PER_SOL);
    await program.methods.initialize(
      new anchor.BN(burnPeriod),
      updateAuth.publicKey,
      mintersPK,
      burnersPK
    )
      .accounts({
        admin: admin.publicKey,
        gpassInfo: gpassInfo.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .signers([admin, gpassInfo])
      .rpc();

    const gpassInfoData = await program.account.gpassInfo.fetch(gpassInfo.publicKey);
    assert.ok(gpassInfoData.admin.equals(admin.publicKey));
    assert.ok(gpassInfoData.updateAuth.equals(updateAuth.publicKey));
    assert.equal(gpassInfoData.burnPeriod.toNumber(), burnPeriod);
    assert.equal(gpassInfoData.totalAmount.toNumber(), 0);
    assert.deepStrictEqual(gpassInfoData.minters, mintersPK);
    assert.deepStrictEqual(gpassInfoData.burners, burnersPK);

    user1WalletPK = findProgramAddressSync(
      [
        utf8.encode(utils.USER_WALLET_SEED),
        gpassInfo.publicKey.toBytes(),
        user1.publicKey.toBytes(),
      ],
      program.programId
    )[0];
    user2WalletPK = findProgramAddressSync(
      [
        utf8.encode(utils.USER_WALLET_SEED),
        gpassInfo.publicKey.toBytes(),
        user2.publicKey.toBytes(),
      ],
      program.programId
    )[0];
  });

  it("User1 create wallet for himself", async () => {
    const currentTimeStamp = utils.currentTimestamp();
    await program.methods.createWallet()
      .accounts({
        payer: user1.publicKey,
        gpassInfo: gpassInfo.publicKey,
        user: user1.publicKey,
        wallet: user1WalletPK,
        systemProgram: SystemProgram.programId,
      })
      .signers([user1])
      .rpc();

    const user1WalletData = await program.account.wallet.fetch(user1WalletPK);
    assert.equal(user1WalletData.amount.toNumber(), 0);
    assert.ok(utils.assertWithPrecission(user1WalletData.lastBurned.toNumber(), currentTimeStamp, 5));
  });

  it("Payer create wallet for user2", async () => {
    const payer = Keypair.generate();
    await utils.airdropSol(program.provider.connection, payer.publicKey, 1 * LAMPORTS_PER_SOL);
    const currentTimeStamp = utils.currentTimestamp();
    await program.methods.createWallet()
      .accounts({
        payer: payer.publicKey,
        gpassInfo: gpassInfo.publicKey,
        user: user2.publicKey,
        wallet: user2WalletPK,
        systemProgram: SystemProgram.programId,
      })
      .signers([payer])
      .rpc();

    const user2WalletData = await program.account.wallet.fetch(user2WalletPK);
    assert.equal(user2WalletData.amount.toNumber(), 0);
    assert.ok(utils.assertWithPrecission(user2WalletData.lastBurned.toNumber(), currentTimeStamp, 5));
  });

  it("Mint amount of gpass to user1 with invalid authority", async () => {
    const amount = 100;
    await assert.rejects(program.methods.mintTo(new anchor.BN(amount))
      .accounts({
        authority: admin.publicKey,
        gpassInfo: gpassInfo.publicKey,
        to: user1WalletPK,
      })
      .signers([admin])
      .rpc(),
      (e: AnchorError) => {
        assert.ok(e.error !== undefined);
        assert.strictEqual(e.error.errorCode.code, "InvalidMintAuthority");
        assert.strictEqual(e.error.errorCode.number, 6005);
        assert.strictEqual(e.error.errorMessage, "Invalid mint authority");
        return true;
      }
    );
  });

  it("Mint invalid amount of gpass to user1", async () => {
    const amount = 0;
    await assert.rejects(program.methods.mintTo(new anchor.BN(amount))
      .accounts({
        authority: admin.publicKey,
        gpassInfo: gpassInfo.publicKey,
        to: user1WalletPK,
      })
      .signers([admin])
      .rpc(),
      (e: AnchorError) => {
        assert.ok(e.error !== undefined);
        assert.strictEqual(e.error.errorCode.code, "ZeroMintAmount");
        assert.strictEqual(e.error.errorCode.number, 6007);
        assert.strictEqual(e.error.errorMessage, "Mint amount cannot be zero");
        return true;
      }
    );
  });

  it("Burn amount of gpass to user1 with invalid authority", async () => {
    const amount = 100;
    await assert.rejects(program.methods.burn(new anchor.BN(amount))
      .accounts({
        authority: admin.publicKey,
        gpassInfo: gpassInfo.publicKey,
        from: user1WalletPK,
      })
      .signers([admin])
      .rpc(),
      (e: AnchorError) => {
        assert.ok(e.error !== undefined);
        assert.strictEqual(e.error.errorCode.code, "InvalidBurnAuthority");
        assert.strictEqual(e.error.errorCode.number, 6006);
        assert.strictEqual(e.error.errorMessage, "Invalid burn authority");
        return true;
      }
    );
  });

  it("Burn invalid amount of gpass from user1", async () => {
    const amount = 0;
    await assert.rejects(program.methods.burn(new anchor.BN(amount))
      .accounts({
        authority: admin.publicKey,
        gpassInfo: gpassInfo.publicKey,
        from: user1WalletPK,
      })
      .signers([admin])
      .rpc(),
      (e: AnchorError) => {
        assert.ok(e.error !== undefined);
        assert.strictEqual(e.error.errorCode.code, "ZeroBurnAmount");
        assert.strictEqual(e.error.errorCode.number, 6008);
        assert.strictEqual(e.error.errorMessage, "Burn amount cannot be zero");
        return true;
      }
    );
  });

  const user1Amount = 1000;
  it("Mint amount of gpass to user1 wallet", async () => {
    await program.methods.mintTo(new anchor.BN(user1Amount))
      .accounts({
        authority: minters[0].publicKey,
        gpassInfo: gpassInfo.publicKey,
        to: user1WalletPK,
      })
      .signers([minters[0]])
      .rpc();

    const user1WalletData = await program.account.wallet.fetch(user1WalletPK);
    assert.equal(user1WalletData.amount.toNumber(), user1Amount);
    const gpassInfoData = await program.account.gpassInfo.fetch(gpassInfo.publicKey);
    assert.equal(gpassInfoData.totalAmount.toNumber(), user1Amount);
  });

  it("Burn amount of gpass from user1 wallet", async () => {
    await program.methods.burn(new anchor.BN(user1Amount / 2))
      .accounts({
        authority: burners[0].publicKey,
        gpassInfo: gpassInfo.publicKey,
        from: user1WalletPK,
      })
      .signers([burners[0]])
      .rpc();

    const user1WalletData = await program.account.wallet.fetch(user1WalletPK);
    assert.equal(user1WalletData.amount.toNumber(), user1Amount / 2);
    const gpassInfoData = await program.account.gpassInfo.fetch(gpassInfo.publicKey);
    assert.equal(gpassInfoData.totalAmount.toNumber(), user1Amount / 2);
  });

  it("Wait burn period and mint gpass to user1 wallet", async () => {
    await utils.sleep(burnPeriod);
    const currentTimeStamp = utils.currentTimestamp();
    await program.methods.mintTo(new anchor.BN(user1Amount))
      .accounts({
        authority: minters[0].publicKey,
        gpassInfo: gpassInfo.publicKey,
        to: user1WalletPK,
      })
      .signers([minters[0]])
      .rpc();

    const user1WalletData = await program.account.wallet.fetch(user1WalletPK);
    assert.equal(user1WalletData.amount.toNumber(), user1Amount);
    assert.ok(utils.assertWithPrecission(user1WalletData.lastBurned.toNumber(), currentTimeStamp, 5));
    const gpassInfoData = await program.account.gpassInfo.fetch(gpassInfo.publicKey);
    assert.equal(gpassInfoData.totalAmount.toNumber(), user1Amount);
  });

  it("Wait burn period and burn gpass from user1 wallet", async () => {
    await utils.sleep(burnPeriod);
    const currentTimeStamp = utils.currentTimestamp();
    await program.methods.burn(new anchor.BN(10))
      .accounts({
        authority: burners[0].publicKey,
        gpassInfo: gpassInfo.publicKey,
        from: user1WalletPK,
      })
      .signers([burners[0]])
      .rpc();

    const user1WalletData = await program.account.wallet.fetch(user1WalletPK);
    assert.equal(user1WalletData.amount.toNumber(), 0);
    assert.ok(utils.assertWithPrecission(user1WalletData.lastBurned.toNumber(), currentTimeStamp, 5));
    const gpassInfoData = await program.account.gpassInfo.fetch(gpassInfo.publicKey);
    assert.equal(gpassInfoData.totalAmount.toNumber(), 0);
  });

  it("Mint gpass to user1 wallet and try burn in period", async () => {
    const currentTimeStamp = utils.currentTimestamp();
    await program.methods.mintTo(new anchor.BN(user1Amount))
      .accounts({
        authority: minters[0].publicKey,
        gpassInfo: gpassInfo.publicKey,
        to: user1WalletPK,
      })
      .signers([minters[0]])
      .rpc();

    await assert.rejects(program.methods.tryBurnInPeriod()
      .accounts({
        gpassInfo: gpassInfo.publicKey,
        wallet: user1WalletPK,
      })
      .rpc(),
      (e: AnchorError) => {
        assert.ok(e.error !== undefined);
        assert.strictEqual(e.error.errorCode.code, "PeriodNotPassed");
        assert.strictEqual(e.error.errorCode.number, 6010);
        assert.strictEqual(e.error.errorMessage, "Burn period not yet passed");
        return true;
      });

    const user1WalletData = await program.account.wallet.fetch(user1WalletPK);
    assert.equal(user1WalletData.amount.toNumber(), user1Amount);
    assert.ok(utils.assertWithPrecission(user1WalletData.lastBurned.toNumber(), currentTimeStamp, 5));
    const gpassInfoData = await program.account.gpassInfo.fetch(gpassInfo.publicKey);
    assert.equal(gpassInfoData.totalAmount.toNumber(), user1Amount);
  });

  it("Mint gpass to user1 wallet, wait and try burn in period", async () => {
    await program.methods.mintTo(new anchor.BN(user1Amount))
      .accounts({
        authority: minters[0].publicKey,
        gpassInfo: gpassInfo.publicKey,
        to: user1WalletPK,
      })
      .signers([minters[0]])
      .rpc();

    await utils.sleep(burnPeriod);

    const currentTimeStamp = utils.currentTimestamp();
    await program.methods.tryBurnInPeriod()
      .accounts({
        gpassInfo: gpassInfo.publicKey,
        wallet: user1WalletPK,
      })
      .rpc();

    const user1WalletData = await program.account.wallet.fetch(user1WalletPK);
    assert.equal(user1WalletData.amount.toNumber(), 0);
    assert.ok(utils.assertWithPrecission(user1WalletData.lastBurned.toNumber(), currentTimeStamp, 5));
    const gpassInfoData = await program.account.gpassInfo.fetch(gpassInfo.publicKey);
    assert.equal(gpassInfoData.totalAmount.toNumber(), 0);
  });
});
