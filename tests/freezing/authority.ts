import * as anchor from "@project-serum/anchor";
import { Program, AnchorError } from "@project-serum/anchor";
import {
  Keypair, SystemProgram, PublicKey
} from "@solana/web3.js";
import { Freezing } from "../../target/types/freezing";
import { Gpass } from "../../target/types/gpass";
import * as assert from "assert";
import * as utils from "../utils";
import { prepareFreezingTestFixture } from "./fixture";
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
  before(async () => {
    let fixture = await prepareFreezingTestFixture(freezingProgram, gpassProgram);
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
        freezingParams: fixture.freezing.params.publicKey,
        accumulativeFund: fixture.freezing.accumulativeFund,
        gpassSettings: fixture.freezing.gpassSettings.publicKey,
        gpassMintAuth: fixture.freezing.gpassMintAuth,
        treasury: fixture.freezing.treasury,
        ggwpToken: fixture.freezing.ggwpToken,
        systemProgram: SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .signers([fixture.admin, fixture.freezing.params])
      .rpc();

    const paramsData = await freezingProgram.account.freezingParams.fetch(fixture.freezing.params.publicKey);
    assert.ok(paramsData.admin.equals(fixture.admin.publicKey));
    assert.ok(paramsData.updateAuth.equals(fixture.updateAuth.publicKey));
    assert.ok(paramsData.ggwpToken.equals(fixture.freezing.ggwpToken));
    assert.ok(paramsData.gpassSettings.equals(fixture.freezing.gpassSettings.publicKey));
    assert.ok(paramsData.accumulativeFund.equals(fixture.freezing.accumulativeFund));
    assert.ok(paramsData.treasury.equals(fixture.freezing.treasury));
    assert.equal(paramsData.totalFreezed.toNumber(), 0);
    assert.equal(paramsData.rewardPeriod.toNumber(), rewardPeriod);
    assert.equal(paramsData.royalty, royalty);
    assert.equal(paramsData.unfreezeRoyalty, unfreezeRoyalty);
    assert.equal(paramsData.unfreezeLockPeriod.toNumber(), unfreezeLockPeriod);
  });

  // TODO:
  const newAdmin = Keypair.generate();
  it("Update admin with invalid admin", async () => {
  });

  it("Update admin", async () => {
  });

  it("Set update authority with invalid admin", async () => {
  });

  it("Set update authority", async () => {
  });

  it("Update royalty with invalid update auth", async () => {
  });

  it("Update royalty with invalid value", async () => {
  });

  it("Update royalty", async () => {
  });

  it("Update unfreeze royalty with invalid update auth", async () => {
  });

  it("Update unfreeze royalty with invalid value", async () => {
  });

  it("Update unfreeze royalty", async () => {
  });

  it("Update reward period with invalid authority", async () => {
  });

  it("Update reward period with invalid value", async () => {
  });

  it("Update reward period", async () => {
  });

  it("Update reward table with invalid authority", async () => {
  });

  it("Update reward table with invalid table", async () => {
  });

  it("Update reward table", async () => {
  });
});
