import * as anchor from "@project-serum/anchor";
import { AnchorError, Program } from "@project-serum/anchor";
import {
  Keypair, SystemProgram, LAMPORTS_PER_SOL, PublicKey
} from "@solana/web3.js";
import { Gpass } from "../../target/types/gpass";
import * as assert from "assert";
import * as utils from "../utils";

describe("GPASS initialize tests", () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.Gpass as Program<Gpass>;

  const admin = Keypair.generate();
  const updateAuth = Keypair.generate();
  const mintersPK = [new PublicKey("27WSu69fpy9NsJEKMLBVz5m6YG2hph5WiU56tvbizngN")];
  const burnersPK = [
    new PublicKey("27WSu69fpy9NsJEKMLBVz5m6YG2hph5WiU56tvbizngN"),
    new PublicKey("2i6PhfdAByhmEJp4rccGz4kDcZv5NWHxifa9cmJ6yYM4"),
    new PublicKey("2XnYWYqbvyoBAEq6kKWRVPerMW7pUDxJo5AKkP5f6iBv"),
  ];
  const burnPeriod = 100;

  before(async () => {
    await utils.airdropSol(program.provider.connection, admin.publicKey, 100 * LAMPORTS_PER_SOL);
  });

  it("Initialize", async () => {
    const settings = Keypair.generate();
    await program.methods.initialize(
      new anchor.BN(burnPeriod),
      updateAuth.publicKey,
      mintersPK,
      burnersPK
    )
      .accounts({
        admin: admin.publicKey,
        settings: settings.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .signers([admin, settings])
      .rpc();

    const settingsData = await program.account.gpassSettings.fetch(settings.publicKey);
    assert.ok(settingsData.admin.equals(admin.publicKey));
    assert.ok(settingsData.updateAuth.equals(updateAuth.publicKey));
    assert.equal(settingsData.burnPeriod.toNumber(), burnPeriod);
    assert.deepStrictEqual(settingsData.minters, mintersPK);
    assert.deepStrictEqual(settingsData.burners, burnersPK);
  });

  it("Initialize with invalid minters list", async () => {
    const settings = Keypair.generate();
    // Invalid minters list len
    const invalidMintersPK = [
      new PublicKey("27WSu69fpy9NsJEKMLBVz5m6YG2hph5WiU56tvbizngN"),
      new PublicKey("2i6PhfdAByhmEJp4rccGz4kDcZv5NWHxifa9cmJ6yYM4"),
    ];
    await assert.rejects(program.methods.initialize(
      new anchor.BN(burnPeriod),
      updateAuth.publicKey,
      invalidMintersPK,
      burnersPK
    )
      .accounts({
        admin: admin.publicKey,
        settings: settings.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .signers([admin, settings])
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

  it("Initialize with invalid burners list", async () => {
    const settings = Keypair.generate();
    // Invalid burners list len
    const invalidBurnersPK = [
      new PublicKey("27WSu69fpy9NsJEKMLBVz5m6YG2hph5WiU56tvbizngN"),
      new PublicKey("2i6PhfdAByhmEJp4rccGz4kDcZv5NWHxifa9cmJ6yYM4"),
      new PublicKey("3HGeJRu8geesShzadEusQttjz3PxgphRRYf7X69aDjYF"),
      new PublicKey("3iz28W56yCZJXgo3Ug1acgBSCJResakqz1jRCUhmFH2R"),
      new PublicKey("8QLJytxCtGEUCtckcaLFVV2dxrwAUdEyaDFKqQiAE6SB"),
      new PublicKey("9sMLugrL5FVw59fqHq17zmZEnDbrXSKUn2gBcy3V5t9b"),
    ];
    await assert.rejects(program.methods.initialize(
      new anchor.BN(burnPeriod),
      updateAuth.publicKey,
      mintersPK,
      invalidBurnersPK
    )
      .accounts({
        admin: admin.publicKey,
        settings: settings.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .signers([admin, settings])
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

  it("Initialize with invalid burn period", async () => {
    const settings = Keypair.generate();
    const invalidBurnPeriod = 0;
    await assert.rejects(program.methods.initialize(
      new anchor.BN(invalidBurnPeriod),
      updateAuth.publicKey,
      mintersPK,
      burnersPK,
    )
      .accounts({
        admin: admin.publicKey,
        settings: settings.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .signers([admin, settings])
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
});
