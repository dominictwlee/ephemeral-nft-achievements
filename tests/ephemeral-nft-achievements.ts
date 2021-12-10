import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { TOKEN_PROGRAM_ID, Token, AuthorityType } from "@solana/spl-token";
import { EphemeralNftAchievements } from "../target/types/ephemeral_nft_achievements";
import { secs } from "./util";
import { expect } from "chai";

describe("ephemeral-nft-achievements", () => {
  anchor.setProvider(anchor.Provider.env());
  const provider = anchor.getProvider();
  const program = anchor.workspace
    .EphemeralNftAchievements as Program<EphemeralNftAchievements>;
  const granter = provider.wallet.publicKey;
  const nodeWallet = anchor.Wallet.local().payer;
  let mint: Token;
  let achievement: anchor.web3.PublicKey;
  let bump: number;
  const recipient = anchor.web3.Keypair.generate().publicKey;

  before(async () => {
    mint = await Token.createMint(
      provider.connection,
      nodeWallet,
      nodeWallet.publicKey,
      null,
      0,
      TOKEN_PROGRAM_ID
    );

    const achievementPDA = await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from("achievement"), mint.publicKey.toBuffer()],
      program.programId
    );
    achievement = achievementPDA[0];
    bump = achievementPDA[1];

    await mint.setAuthority(
      mint.publicKey,
      achievement,
      "MintTokens",
      granter,
      []
    );
  });

  it("creates an achievement", async () => {
    console.log("public", mint.publicKey);

    const tx = await program.rpc.createAchievement(
      {
        tier: { major: {} } as never,
        validityLength: new anchor.BN(secs("1y")),
        uri: "www.some-resource-link.com",
        bump,
        maxTransferCount: 1,
      },
      {
        accounts: {
          achievement,
          mint: mint.publicKey,
          granter,
          recipient,
          granterAuthority: granter,
          sysvarRent: anchor.web3.SYSVAR_RENT_PUBKEY,
          sysvarClock: anchor.web3.SYSVAR_CLOCK_PUBKEY,
          tokenProgram: TOKEN_PROGRAM_ID,
          systemProgram: anchor.web3.SystemProgram.programId,
        },
      }
    );
    const currentAchievement = await program.account.achievement.fetch(
      achievement
    );
    expect(currentAchievement.granter.equals(granter)).to.be.true;
    expect(currentAchievement.recipient.equals(recipient)).to.be.true;
    expect(currentAchievement.currentOwner.equals(recipient)).to.be.true;
    expect(currentAchievement.mint.equals(mint.publicKey)).to.be.true;
    expect(Object.keys(currentAchievement.tier)[0]).to.be.equal("major");
    expect(currentAchievement.bump).to.equal(bump);
  });
});
