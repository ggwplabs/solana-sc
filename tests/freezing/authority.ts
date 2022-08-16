import * as anchor from "@project-serum/anchor";
import { Program, AnchorError } from "@project-serum/anchor";
import {
  Keypair, SystemProgram, LAMPORTS_PER_SOL
} from "@solana/web3.js";
import { Freezing } from "../../target/types/freezing";
import { Gpass } from "../../target/types/gpass";
import * as assert from "assert";
import * as utils from "../utils";
import { FreezingTestFixture, prepareFreezingTestFixture } from "./fixture";
import { TOKEN_PROGRAM_ID } from "@project-serum/anchor/dist/cjs/utils/token";

describe("Freezing authority tests", () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  const freezingProgram = anchor.workspace.Freezing as Program<Freezing>;
  const gpassProgram = anchor.workspace.Gpass as Program<Gpass>;

  const rewardPeriod = 100;
  const royalty = 8;
  const unfreezeRoyalty = 15;
  const unfreezeLockPeriod = 500;
  const rewardTable = [
    {
      ggwpAmount: new anchor.BN(10_000_000_000),
      gpassAmount: new anchor.BN(5),
    }
  ];

  let fixture: FreezingTestFixture;
  before(async () => {
    fixture = await prepareFreezingTestFixture(freezingProgram, gpassProgram);
    await freezingProgram.methods.initialize(
      fixture.updateAuth.publicKey,
      new anchor.BN(rewardPeriod),
      royalty,
      unfreezeRoyalty,
      new anchor.BN(unfreezeLockPeriod),
      rewardTable,
    )
      .accounts({
        admin: fixture.admin.publicKey,
        freezingInfo: fixture.freezing.info.publicKey,
        accumulativeFund: fixture.freezing.accumulativeFund,
        gpassInfo: fixture.freezing.gpassInfo.publicKey,
        gpassMintAuth: fixture.freezing.gpassMintAuth,
        treasury: fixture.freezing.treasury,
        treasuryAuth: fixture.freezing.treasuryAuth,
        ggwpToken: fixture.freezing.ggwpToken,
        systemProgram: SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .signers([fixture.admin, fixture.freezing.info])
      .rpc();

    const freezingInfoData = await freezingProgram.account.freezingInfo.fetch(fixture.freezing.info.publicKey);
    assert.ok(freezingInfoData.admin.equals(fixture.admin.publicKey));
    assert.ok(freezingInfoData.updateAuth.equals(fixture.updateAuth.publicKey));
    assert.ok(freezingInfoData.ggwpToken.equals(fixture.freezing.ggwpToken));
    assert.ok(freezingInfoData.gpassInfo.equals(fixture.freezing.gpassInfo.publicKey));
    assert.ok(freezingInfoData.accumulativeFund.equals(fixture.freezing.accumulativeFund));
    assert.ok(freezingInfoData.treasury.equals(fixture.freezing.treasury));
    assert.equal(freezingInfoData.totalFreezed.toNumber(), 0);
    assert.equal(freezingInfoData.rewardPeriod.toNumber(), rewardPeriod);
    assert.equal(freezingInfoData.royalty, royalty);
    assert.equal(freezingInfoData.unfreezeRoyalty, unfreezeRoyalty);
    assert.equal(freezingInfoData.unfreezeLockPeriod.toNumber(), unfreezeLockPeriod);
  });

  const newAdmin = Keypair.generate();
  it("Update admin with invalid admin", async () => {
    const invalidAuthority = Keypair.generate();
    await utils.airdropSol(freezingProgram.provider.connection, invalidAuthority.publicKey, 1 * LAMPORTS_PER_SOL);
    await assert.rejects(freezingProgram.methods
      .updateAdmin(newAdmin.publicKey)
      .accounts({
        authority: invalidAuthority.publicKey,
        freezingInfo:
          fixture.freezing.info.publicKey
      })
      .signers([invalidAuthority])
      .rpc(),
      (e: AnchorError) => {
        assert.ok(e.error !== undefined);
        assert.strictEqual(e.error.errorCode.code, "AccessDenied");
        assert.strictEqual(e.error.errorCode.number, 6000);
        assert.strictEqual(e.error.errorMessage, "Access denied");
        return true;
      });
  });

  it("Update admin", async () => {
    await freezingProgram.methods
      .updateAdmin(newAdmin.publicKey)
      .accounts({
        authority: fixture.admin.publicKey,
        freezingInfo:
          fixture.freezing.info.publicKey
      })
      .signers([fixture.admin])
      .rpc();

    const freezingInfoData = await freezingProgram.account.freezingInfo.fetch(fixture.freezing.info.publicKey);
    assert.ok(freezingInfoData.admin.equals(newAdmin.publicKey));
  });

  const newUpdateAuth = Keypair.generate();
  it("Set update authority with invalid admin", async () => {
    await assert.rejects(freezingProgram.methods
      .setUpdateAuthority(newUpdateAuth.publicKey)
      .accounts({
        authority: fixture.admin.publicKey,
        freezingInfo:
          fixture.freezing.info.publicKey
      })
      .signers([fixture.admin])
      .rpc(),
      (e: AnchorError) => {
        assert.ok(e.error !== undefined);
        assert.strictEqual(e.error.errorCode.code, "AccessDenied");
        assert.strictEqual(e.error.errorCode.number, 6000);
        assert.strictEqual(e.error.errorMessage, "Access denied");
        return true;
      });
  });

  it("Set update authority", async () => {
    await freezingProgram.methods
      .setUpdateAuthority(newUpdateAuth.publicKey)
      .accounts({
        authority: newAdmin.publicKey,
        freezingInfo:
          fixture.freezing.info.publicKey
      })
      .signers([newAdmin])
      .rpc();

    const freezingInfoData = await freezingProgram.account.freezingInfo.fetch(fixture.freezing.info.publicKey);
    assert.ok(freezingInfoData.updateAuth.equals(newUpdateAuth.publicKey));
  });

  it("Update royalty with invalid update auth", async () => {
    await assert.rejects(freezingProgram.methods
      .updateRoyalty(100)
      .accounts({
        authority: fixture.updateAuth.publicKey,
        freezingInfo:
          fixture.freezing.info.publicKey
      })
      .signers([fixture.updateAuth])
      .rpc(),
      (e: AnchorError) => {
        assert.ok(e.error !== undefined);
        assert.strictEqual(e.error.errorCode.code, "AccessDenied");
        assert.strictEqual(e.error.errorCode.number, 6000);
        assert.strictEqual(e.error.errorMessage, "Access denied");
        return true;
      });
  });

  it("Update royalty with invalid value", async () => {
    const invalidRoyalty = 101;
    await assert.rejects(freezingProgram.methods
      .updateRoyalty(invalidRoyalty)
      .accounts({
        authority: newUpdateAuth.publicKey,
        freezingInfo:
          fixture.freezing.info.publicKey
      })
      .signers([newUpdateAuth])
      .rpc(),
      (e: AnchorError) => {
        assert.ok(e.error !== undefined);
        assert.strictEqual(e.error.errorCode.code, "InvalidRoyaltyValue");
        assert.strictEqual(e.error.errorCode.number, 6010);
        assert.strictEqual(e.error.errorMessage, "Invalid royalty value");
        return true;
      });
  });

  it("Update royalty", async () => {
    const newRoyalty = 23;
    await freezingProgram.methods
      .updateRoyalty(newRoyalty)
      .accounts({
        authority: newUpdateAuth.publicKey,
        freezingInfo:
          fixture.freezing.info.publicKey
      })
      .signers([newUpdateAuth])
      .rpc();

    const freezingInfoData = await freezingProgram.account.freezingInfo.fetch(fixture.freezing.info.publicKey);
    assert.equal(freezingInfoData.royalty, newRoyalty);
  });

  it("Update unfreeze royalty with invalid update auth", async () => {
    await assert.rejects(freezingProgram.methods
      .updateUnfreezeRoyalty(100)
      .accounts({
        authority: fixture.updateAuth.publicKey,
        freezingInfo:
          fixture.freezing.info.publicKey
      })
      .signers([fixture.updateAuth])
      .rpc(),
      (e: AnchorError) => {
        assert.ok(e.error !== undefined);
        assert.strictEqual(e.error.errorCode.code, "AccessDenied");
        assert.strictEqual(e.error.errorCode.number, 6000);
        assert.strictEqual(e.error.errorMessage, "Access denied");
        return true;
      });
  });

  it("Update unfreeze royalty with invalid value", async () => {
    const invalidRoyalty = 101;
    await assert.rejects(freezingProgram.methods
      .updateUnfreezeRoyalty(invalidRoyalty)
      .accounts({
        authority: newUpdateAuth.publicKey,
        freezingInfo:
          fixture.freezing.info.publicKey
      })
      .signers([newUpdateAuth])
      .rpc(),
      (e: AnchorError) => {
        assert.ok(e.error !== undefined);
        assert.strictEqual(e.error.errorCode.code, "InvalidUnfreezeRoyaltyValue");
        assert.strictEqual(e.error.errorCode.number, 6011);
        assert.strictEqual(e.error.errorMessage, "Invalid unfreeze royalty value");
        return true;
      });
  });

  it("Update unfreeze royalty", async () => {
    const newRoyalty = 23;
    await freezingProgram.methods
      .updateUnfreezeRoyalty(newRoyalty)
      .accounts({
        authority: newUpdateAuth.publicKey,
        freezingInfo:
          fixture.freezing.info.publicKey
      })
      .signers([newUpdateAuth])
      .rpc();

    const freezingInfoData = await freezingProgram.account.freezingInfo.fetch(fixture.freezing.info.publicKey);
    assert.equal(freezingInfoData.unfreezeRoyalty, newRoyalty);
  });

  it("Update reward period with invalid authority", async () => {
    await assert.rejects(freezingProgram.methods
      .updateRewardPeriod(new anchor.BN(100))
      .accounts({
        authority: fixture.updateAuth.publicKey,
        freezingInfo:
          fixture.freezing.info.publicKey
      })
      .signers([fixture.updateAuth])
      .rpc(),
      (e: AnchorError) => {
        assert.ok(e.error !== undefined);
        assert.strictEqual(e.error.errorCode.code, "AccessDenied");
        assert.strictEqual(e.error.errorCode.number, 6000);
        assert.strictEqual(e.error.errorMessage, "Access denied");
        return true;
      });
  });

  it("Update reward period with invalid value", async () => {
    const rewardPeriod = 0;
    await assert.rejects(freezingProgram.methods
      .updateRewardPeriod(new anchor.BN(rewardPeriod))
      .accounts({
        authority: newUpdateAuth.publicKey,
        freezingInfo:
          fixture.freezing.info.publicKey
      })
      .signers([newUpdateAuth])
      .rpc(),
      (e: AnchorError) => {
        assert.ok(e.error !== undefined);
        assert.strictEqual(e.error.errorCode.code, "InvalidRewardPeriod");
        assert.strictEqual(e.error.errorCode.number, 6014);
        assert.strictEqual(e.error.errorMessage, "Invalid reward period value");
        return true;
      });
  });

  it("Update reward period", async () => {
    const newRewardPeriod = 11223344;
    await freezingProgram.methods
      .updateRewardPeriod(new anchor.BN(newRewardPeriod))
      .accounts({
        authority: newUpdateAuth.publicKey,
        freezingInfo:
          fixture.freezing.info.publicKey
      })
      .signers([newUpdateAuth])
      .rpc();

    const freezingInfoData = await freezingProgram.account.freezingInfo.fetch(fixture.freezing.info.publicKey);
    assert.equal(freezingInfoData.rewardPeriod.toNumber(), newRewardPeriod);
  });

  it("Update reward table with invalid authority", async () => {
    await assert.rejects(freezingProgram.methods
      .updateRewardTable([
        {
          ggwpAmount: new anchor.BN(100),
          gpassAmount: new anchor.BN(1),
        }
      ])
      .accounts({
        authority: fixture.updateAuth.publicKey,
        freezingInfo:
          fixture.freezing.info.publicKey
      })
      .signers([fixture.updateAuth])
      .rpc(),
      (e: AnchorError) => {
        assert.ok(e.error !== undefined);
        assert.strictEqual(e.error.errorCode.code, "AccessDenied");
        assert.strictEqual(e.error.errorCode.number, 6000);
        assert.strictEqual(e.error.errorMessage, "Access denied");
        return true;
      });
  });

  it("Update reward table with empty table", async () => {
    await assert.rejects(freezingProgram.methods
      .updateRewardTable([])
      .accounts({
        authority: fixture.updateAuth.publicKey,
        freezingInfo:
          fixture.freezing.info.publicKey
      })
      .signers([fixture.updateAuth])
      .rpc(),
      (e: AnchorError) => {
        assert.ok(e.error !== undefined);
        assert.strictEqual(e.error.errorCode.code, "InvalidRewardTable");
        assert.strictEqual(e.error.errorCode.number, 6013);
        assert.strictEqual(e.error.errorMessage, "Invalid reward table");
        return true;
      });
  });

  it("Update reward table with max+1 table size", async () => {
    await assert.rejects(freezingProgram.methods
      .updateRewardTable([
        {
          ggwpAmount: new anchor.BN(50),
          gpassAmount: new anchor.BN(1),
        },
        {
          ggwpAmount: new anchor.BN(60),
          gpassAmount: new anchor.BN(2),
        },
        {
          ggwpAmount: new anchor.BN(70),
          gpassAmount: new anchor.BN(3),
        },
        {
          ggwpAmount: new anchor.BN(80),
          gpassAmount: new anchor.BN(4),
        },
        {
          ggwpAmount: new anchor.BN(90),
          gpassAmount: new anchor.BN(5),
        },
        {
          ggwpAmount: new anchor.BN(100),
          gpassAmount: new anchor.BN(6),
        },
      ])
      .accounts({
        authority: fixture.updateAuth.publicKey,
        freezingInfo:
          fixture.freezing.info.publicKey
      })
      .signers([fixture.updateAuth])
      .rpc(),
      (e: AnchorError) => {
        assert.ok(e.error !== undefined);
        assert.strictEqual(e.error.errorCode.code, "InvalidRewardTable");
        assert.strictEqual(e.error.errorCode.number, 6013);
        assert.strictEqual(e.error.errorMessage, "Invalid reward table");
        return true;
      });
  });

  it("Update reward table with invalid table", async () => {
    await assert.rejects(freezingProgram.methods
      .updateRewardTable([
        {
          ggwpAmount: new anchor.BN(100),
          gpassAmount: new anchor.BN(1),
        },
        {
          ggwpAmount: new anchor.BN(50),
          gpassAmount: new anchor.BN(2),
        }
      ])
      .accounts({
        authority: newUpdateAuth.publicKey,
        freezingInfo:
          fixture.freezing.info.publicKey
      })
      .signers([newUpdateAuth])
      .rpc(),
      (e: AnchorError) => {
        assert.ok(e.error !== undefined);
        assert.strictEqual(e.error.errorCode.code, "InvalidRewardTable");
        assert.strictEqual(e.error.errorCode.number, 6013);
        assert.strictEqual(e.error.errorMessage, "Invalid reward table");
        return true;
      });
  });

  it("Update reward table", async () => {
    await freezingProgram.methods
      .updateRewardTable([
        {
          ggwpAmount: new anchor.BN(1000_000_000_000),
          gpassAmount: new anchor.BN(5),
        },
        {
          ggwpAmount: new anchor.BN(2000_000_000_000),
          gpassAmount: new anchor.BN(10),
        },
        {
          ggwpAmount: new anchor.BN(3000_000_000_000),
          gpassAmount: new anchor.BN(15),
        },
        {
          ggwpAmount: new anchor.BN(4000_000_000_000),
          gpassAmount: new anchor.BN(20),
        },
        {
          ggwpAmount: new anchor.BN(5000_000_000_000),
          gpassAmount: new anchor.BN(25),
        },
      ])
      .accounts({
        authority: newUpdateAuth.publicKey,
        freezingInfo:
          fixture.freezing.info.publicKey
      })
      .signers([newUpdateAuth])
      .rpc();

    const freezingInfoData = await freezingProgram.account.freezingInfo.fetch(fixture.freezing.info.publicKey);
    assert.equal(freezingInfoData.rewardTable[0].ggwpAmount.toNumber(), 1000_000_000_000);
    assert.equal(freezingInfoData.rewardTable[0].gpassAmount.toNumber(), 5);
    assert.equal(freezingInfoData.rewardTable[1].ggwpAmount.toNumber(), 2000_000_000_000);
    assert.equal(freezingInfoData.rewardTable[1].gpassAmount.toNumber(), 10);
  });

  it("Update unfreeze lock period with invalid authority", async () => {
    await assert.rejects(freezingProgram.methods
      .updateUnfreezeLockPeriod(new anchor.BN(100))
      .accounts({
        authority: fixture.updateAuth.publicKey,
        freezingInfo:
          fixture.freezing.info.publicKey
      })
      .signers([fixture.updateAuth])
      .rpc(),
      (e: AnchorError) => {
        assert.ok(e.error !== undefined);
        assert.strictEqual(e.error.errorCode.code, "AccessDenied");
        assert.strictEqual(e.error.errorCode.number, 6000);
        assert.strictEqual(e.error.errorMessage, "Access denied");
        return true;
      });
  });

  it("Update unfreeze lock period with invalid value", async () => {
    const unfreezeLockPeriod = 0;
    await assert.rejects(freezingProgram.methods
      .updateUnfreezeLockPeriod(new anchor.BN(unfreezeLockPeriod))
      .accounts({
        authority: newUpdateAuth.publicKey,
        freezingInfo:
          fixture.freezing.info.publicKey
      })
      .signers([newUpdateAuth])
      .rpc(),
      (e: AnchorError) => {
        assert.ok(e.error !== undefined);
        assert.strictEqual(e.error.errorCode.code, "InvalidUnfreezeLockPeriod");
        assert.strictEqual(e.error.errorCode.number, 6012);
        assert.strictEqual(e.error.errorMessage, "Invalid unfreeze lock period");
        return true;
      });
  });

  it("Update unfreeze lock period", async () => {
    const newPeriod = 11223344;
    await freezingProgram.methods
      .updateUnfreezeLockPeriod(new anchor.BN(newPeriod))
      .accounts({
        authority: newUpdateAuth.publicKey,
        freezingInfo:
          fixture.freezing.info.publicKey
      })
      .signers([newUpdateAuth])
      .rpc();

    const freezingInfoData = await freezingProgram.account.freezingInfo.fetch(fixture.freezing.info.publicKey);
    assert.equal(freezingInfoData.unfreezeLockPeriod.toNumber(), newPeriod);
  });
});
