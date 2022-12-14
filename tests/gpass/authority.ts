import * as anchor from "@project-serum/anchor";
import { Program, AnchorError } from "@project-serum/anchor";
import {
  Keypair, SystemProgram, LAMPORTS_PER_SOL, PublicKey
} from "@solana/web3.js";
import { Gpass } from "../../target/types/gpass";
import * as assert from "assert";
import * as utils from "../utils";

describe("GPASS authority tests", () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.Gpass as Program<Gpass>;

  const admin = Keypair.generate();
  const updateAuth = Keypair.generate();
  const mintersPK = [Keypair.generate().publicKey];
  const burnersPK = [
    Keypair.generate().publicKey,
    Keypair.generate().publicKey,
  ];
  const burnPeriod = 100;

  const gpassInfo = Keypair.generate();
  before(async () => {
    await utils.airdropSol(program.provider.connection, admin.publicKey, 100 * LAMPORTS_PER_SOL);
    await utils.airdropSol(program.provider.connection, updateAuth.publicKey, 100 * LAMPORTS_PER_SOL);
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
  });

  const newAdmin = Keypair.generate();
  it("update admin with invalid admin", async () => {
    const invalidAdmin = Keypair.generate();
    await utils.airdropSol(program.provider.connection, invalidAdmin.publicKey, 1 * LAMPORTS_PER_SOL);
    await assert.rejects(program.methods.updateAdmin(newAdmin.publicKey)
      .accounts({
        authority: invalidAdmin.publicKey,
        gpassInfo: gpassInfo.publicKey,
      })
      .signers([invalidAdmin])
      .rpc(),
      (e: AnchorError) => {
        assert.ok(e.error !== undefined);
        assert.strictEqual(e.error.errorCode.code, "AccessDenied");
        assert.strictEqual(e.error.errorCode.number, 6000);
        assert.strictEqual(e.error.errorMessage, "Access denied");
        return true;
      }
    );
  });

  it("update admin", async () => {
    await program.methods.updateAdmin(newAdmin.publicKey)
      .accounts({
        authority: admin.publicKey,
        gpassInfo: gpassInfo.publicKey,
      })
      .signers([admin])
      .rpc();

    const gpassInfoData = await program.account.gpassInfo.fetch(gpassInfo.publicKey);
    assert.ok(gpassInfoData.admin.equals(newAdmin.publicKey));
  });

  const newUpdAuthority = Keypair.generate();
  it("set update authority with invalid admin", async () => {
    const invalidUpdateAuth = Keypair.generate();
    await utils.airdropSol(program.provider.connection, invalidUpdateAuth.publicKey, 1 * LAMPORTS_PER_SOL);
    await assert.rejects(program.methods.setUpdateAuthority(newUpdAuthority.publicKey)
      .accounts({
        authority: invalidUpdateAuth.publicKey,
        gpassInfo: gpassInfo.publicKey,
      })
      .signers([invalidUpdateAuth])
      .rpc(),
      (e: AnchorError) => {
        assert.ok(e.error !== undefined);
        assert.strictEqual(e.error.errorCode.code, "AccessDenied");
        assert.strictEqual(e.error.errorCode.number, 6000);
        assert.strictEqual(e.error.errorMessage, "Access denied");
        return true;
      }
    );
  });

  it("set update authority", async () => {
    await program.methods.setUpdateAuthority(newUpdAuthority.publicKey)
      .accounts({
        authority: newAdmin.publicKey,
        gpassInfo: gpassInfo.publicKey,
      })
      .signers([newAdmin])
      .rpc();

    const gpassInfoData = await program.account.gpassInfo.fetch(gpassInfo.publicKey);
    assert.ok(gpassInfoData.updateAuth.equals(newUpdAuthority.publicKey));
  });

  it("update burn period with invalid authority", async () => {
    const invalidUpdateAuth = Keypair.generate();
    const newBurnPeriod = 200;
    await utils.airdropSol(program.provider.connection, invalidUpdateAuth.publicKey, 1 * LAMPORTS_PER_SOL);
    await assert.rejects(program.methods.updateBurnPeriod(new anchor.BN(newBurnPeriod))
      .accounts({
        authority: invalidUpdateAuth.publicKey,
        gpassInfo: gpassInfo.publicKey,
      })
      .signers([invalidUpdateAuth])
      .rpc(),
      (e: AnchorError) => {
        assert.ok(e.error !== undefined);
        assert.strictEqual(e.error.errorCode.code, "AccessDenied");
        assert.strictEqual(e.error.errorCode.number, 6000);
        assert.strictEqual(e.error.errorMessage, "Access denied");
        return true;
      }
    );
  });

  it("update burn period with invalid burn period value", async () => {
    const invalidBurnPeriod = 0;
    await assert.rejects(program.methods.updateBurnPeriod(new anchor.BN(invalidBurnPeriod))
      .accounts({
        authority: newUpdAuthority.publicKey,
        gpassInfo: gpassInfo.publicKey,
      })
      .signers([newUpdAuthority])
      .rpc(),
      (e: AnchorError) => {
        assert.ok(e.error !== undefined);
        assert.strictEqual(e.error.errorCode.code, "InvalidBurnPeriodValue");
        assert.strictEqual(e.error.errorCode.number, 6004);
        assert.strictEqual(e.error.errorMessage, "Invalid burn period value");
        return true;
      }
    );
  });

  it("update burn period", async () => {
    const newBurnPeriod = 200;
    await program.methods.updateBurnPeriod(new anchor.BN(newBurnPeriod))
      .accounts({
        authority: newUpdAuthority.publicKey,
        gpassInfo: gpassInfo.publicKey,
      })
      .signers([newUpdAuthority])
      .rpc();

    const gpassInfoData = await program.account.gpassInfo.fetch(gpassInfo.publicKey);
    assert.equal(gpassInfoData.burnPeriod.toNumber(), newBurnPeriod);
  });

  it("update minters with invalid authority", async () => {
    const invalidUpdateAuth = Keypair.generate();
    const newMinters = [new PublicKey("27WSu69fpy9NsJEKMLBVz5m6YG2hph5WiU56tvbizngN")];
    await utils.airdropSol(program.provider.connection, invalidUpdateAuth.publicKey, 1 * LAMPORTS_PER_SOL);
    await assert.rejects(program.methods.updateMinters(newMinters)
      .accounts({
        authority: invalidUpdateAuth.publicKey,
        gpassInfo: gpassInfo.publicKey,
      })
      .signers([invalidUpdateAuth])
      .rpc(),
      (e: AnchorError) => {
        assert.ok(e.error !== undefined);
        assert.strictEqual(e.error.errorCode.code, "AccessDenied");
        assert.strictEqual(e.error.errorCode.number, 6000);
        assert.strictEqual(e.error.errorMessage, "Access denied");
        return true;
      }
    );
  });

  it("update minters with invalid minters list size", async () => {
    const newMinters = [
      new PublicKey("27WSu69fpy9NsJEKMLBVz5m6YG2hph5WiU56tvbizngN"),
      new PublicKey("2i6PhfdAByhmEJp4rccGz4kDcZv5NWHxifa9cmJ6yYM4"),
    ];
    await assert.rejects(program.methods.updateMinters(newMinters)
      .accounts({
        authority: newUpdAuthority.publicKey,
        gpassInfo: gpassInfo.publicKey,
      })
      .signers([newUpdAuthority])
      .rpc(),
      (e: AnchorError) => {
        assert.ok(e.error !== undefined);
        assert.strictEqual(e.error.errorCode.code, "MaxMintersSizeExceeded");
        assert.strictEqual(e.error.errorCode.number, 6002);
        assert.strictEqual(e.error.errorMessage, "Max minters size exceeded");
        return true;
      }
    );
  });

  it("update minters", async () => {
    const newMinters = [
      new PublicKey("27WSu69fpy9NsJEKMLBVz5m6YG2hph5WiU56tvbizngN"),
    ];
    await program.methods.updateMinters(newMinters)
      .accounts({
        authority: newUpdAuthority.publicKey,
        gpassInfo: gpassInfo.publicKey,
      })
      .signers([newUpdAuthority])
      .rpc();

    const gpassInfoData = await program.account.gpassInfo.fetch(gpassInfo.publicKey);
    assert.deepStrictEqual(gpassInfoData.minters, newMinters);
  });

  it("update burners with invalid authority", async () => {
    const invalidUpdateAuth = Keypair.generate();
    const newBurners = [new PublicKey("27WSu69fpy9NsJEKMLBVz5m6YG2hph5WiU56tvbizngN")];
    await utils.airdropSol(program.provider.connection, invalidUpdateAuth.publicKey, 1 * LAMPORTS_PER_SOL);
    await assert.rejects(program.methods.updateBurners(newBurners)
      .accounts({
        authority: invalidUpdateAuth.publicKey,
        gpassInfo: gpassInfo.publicKey,
      })
      .signers([invalidUpdateAuth])
      .rpc(),
      (e: AnchorError) => {
        assert.ok(e.error !== undefined);
        assert.strictEqual(e.error.errorCode.code, "AccessDenied");
        assert.strictEqual(e.error.errorCode.number, 6000);
        assert.strictEqual(e.error.errorMessage, "Access denied");
        return true;
      }
    );
  });

  it("update burners with invalid burners list size", async () => {
    const newBurners = [
      new PublicKey("27WSu69fpy9NsJEKMLBVz5m6YG2hph5WiU56tvbizngN"),
      new PublicKey("2i6PhfdAByhmEJp4rccGz4kDcZv5NWHxifa9cmJ6yYM4"),
      new PublicKey("8QLJytxCtGEUCtckcaLFVV2dxrwAUdEyaDFKqQiAE6SB"),
      new PublicKey("9sMLugrL5FVw59fqHq17zmZEnDbrXSKUn2gBcy3V5t9b"),
    ];
    await assert.rejects(program.methods.updateBurners(newBurners)
      .accounts({
        authority: newUpdAuthority.publicKey,
        gpassInfo: gpassInfo.publicKey,
      })
      .signers([newUpdAuthority])
      .rpc(),
      (e: AnchorError) => {
        assert.ok(e.error !== undefined);
        assert.strictEqual(e.error.errorCode.code, "MaxBurnersSizeExceeded");
        assert.strictEqual(e.error.errorCode.number, 6003);
        assert.strictEqual(e.error.errorMessage, "Max burners size exceeded");
        return true;
      }
    );
  });

  it("update burners", async () => {
    const newBurners = [
      new PublicKey("27WSu69fpy9NsJEKMLBVz5m6YG2hph5WiU56tvbizngN"),
    ];
    await program.methods.updateBurners(newBurners)
      .accounts({
        authority: newUpdAuthority.publicKey,
        gpassInfo: gpassInfo.publicKey,
      })
      .signers([newUpdAuthority])
      .rpc();

    const gpassInfoData = await program.account.gpassInfo.fetch(gpassInfo.publicKey);
    assert.deepStrictEqual(gpassInfoData.minters, newBurners);
  });
});
