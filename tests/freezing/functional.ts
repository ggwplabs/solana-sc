import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import {
  Keypair, SystemProgram, LAMPORTS_PER_SOL
} from "@solana/web3.js";
import { Freezing } from "../../target/types/freezing";
import { Gpass } from "../../target/types/gpass";
import * as assert from "assert";
import * as utils from "../utils";
import { FreezingTestFixture, prepareFreezingTestFixture } from "./fixture";
import { TOKEN_PROGRAM_ID } from "@project-serum/anchor/dist/cjs/utils/token";

describe("Freezing functional tests", () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  const freezingProgram = anchor.workspace.Freezing as Program<Freezing>;
  const gpassProgram = anchor.workspace.Gpass as Program<Gpass>;

  const rewardPeriod = 20;
  const royalty = 8;
  const unfreezeRoyalty = 15;
  const unfreezeLockPeriod = 10;
  const rewardTable = [
    {
      ggwpAmount: new anchor.BN(10_000_000_000),
      gpassAmount: new anchor.BN(5),
    },
    {
      ggwpAmount: new anchor.BN(20_000_000_000),
      gpassAmount: new anchor.BN(10),
    },
    {
      ggwpAmount: new anchor.BN(30_000_000_000),
      gpassAmount: new anchor.BN(15),
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
  });

  const userFreezeAmount = 10_870_000_000; // 10 GGWP + royalty percent
  it("User freeze amount of GGWP", async () => {
    await freezingProgram.methods.freeze(new anchor.BN(userFreezeAmount))
      .accounts({
        user: fixture.user.kp.publicKey,
        userInfo: fixture.user.info,
        userGgwpWallet: fixture.user.ggwpWallet,
        userGpassWallet: fixture.user.gpassWallet,
        freezingParams: fixture.freezing.params.publicKey,
        gpassSettings: fixture.freezing.gpassSettings.publicKey,
        gpassMintAuth: fixture.freezing.gpassMintAuth,
        accumulativeFund: fixture.freezing.accumulativeFund,
        treasury: fixture.freezing.treasury,
        systemProgram: SystemProgram.programId,
        gpassProgram: gpassProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .signers([fixture.user.kp])
      .rpc();

    const userWalletData = await gpassProgram.account.wallet.fetch(fixture.user.gpassWallet);
    assert.equal(userWalletData.amount.toNumber(), 5);
    assert.ok(utils.assertWithPrecission(await utils.getTokenBalance(fixture.freezing.accumulativeFund), utils.calcRoyaltyAmount(userFreezeAmount, royalty), 1));
    assert.ok(utils.assertWithPrecission(await utils.getTokenBalance(fixture.freezing.treasury), userFreezeAmount - utils.calcRoyaltyAmount(userFreezeAmount, royalty), 1));
    const freezingParamsData = await freezingProgram.account.freezingParams.fetch(fixture.freezing.params.publicKey);
    assert.ok(utils.assertWithPrecission(freezingParamsData.totalFreezed.toNumber(), userFreezeAmount - utils.calcRoyaltyAmount(userFreezeAmount, royalty), 1));
  });
});
