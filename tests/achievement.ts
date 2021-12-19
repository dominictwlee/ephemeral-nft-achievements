import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { NftAchievement } from "../target/types/nft_achievement";
import { secs } from "./util";
import { expect } from "chai";

describe("Achievement program", async () => {
  anchor.setProvider(anchor.Provider.env());
  const provider = anchor.getProvider();
  const program = anchor.workspace.NftAchievement as Program<NftAchievement>;
  const issuer = provider.wallet.publicKey;
  let mint = anchor.web3.Keypair.generate();
  const [achievement, achievementBump] =
    await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from("achievement"), mint.publicKey.toBuffer()],
      program.programId
    );
  const recipient = anchor.web3.Keypair.generate().publicKey;
  const [tokenHolding, tokenHoldingBump] =
    await anchor.web3.PublicKey.findProgramAddress(
      [
        Buffer.from("achievement_token_holding"),
        mint.publicKey.toBuffer(),
        recipient.toBuffer(),
      ],
      program.programId
    );

  it("creates an achievement", async () => {
    const tx = await program.rpc.createAchievement(
      {
        tier: { major: {} } as never,
        validityLength: new anchor.BN(secs("1y")),
        uri: "www.some-resource-link.com",
        bump: achievementBump,
        maxTransferCount: 1,
      },
      {
        accounts: {
          achievement,
          mint: mint.publicKey,
          issuer,
          recipient,
          issuerAuthority: issuer,
          sysvarRent: anchor.web3.SYSVAR_RENT_PUBKEY,
          sysvarClock: anchor.web3.SYSVAR_CLOCK_PUBKEY,
          tokenProgram: TOKEN_PROGRAM_ID,
          systemProgram: anchor.web3.SystemProgram.programId,
        },
        signers: [mint],
      }
    );
    const currentAchievement = await program.account.achievement.fetch(
      achievement
    );
    expect(currentAchievement.issuer.equals(issuer)).to.be.true;
    expect(currentAchievement.owner.equals(issuer)).to.be.true;
    expect(currentAchievement.mint.equals(mint.publicKey)).to.be.true;
    expect(Object.keys(currentAchievement.tier)[0]).to.be.equal("major");
    expect(currentAchievement.bump).to.equal(achievementBump);
  });

  it("grants an achievement to recipient", async () => {
    const tx = await program.rpc.grantAchievement(
      achievementBump,
      tokenHoldingBump,
      {
        accounts: {
          achievement,
          mint: mint.publicKey,
          tokenHolding,
          issuer,
          recipient,
          issuerAuthority: issuer,
          sysvarRent: anchor.web3.SYSVAR_RENT_PUBKEY,
          sysvarClock: anchor.web3.SYSVAR_CLOCK_PUBKEY,
          tokenProgram: TOKEN_PROGRAM_ID,
          systemProgram: anchor.web3.SystemProgram.programId,
        },
      }
    );

    // expect(currentAchievement.issuer.equals(issuer)).to.be.true;
    // expect(currentAchievement.owner.equals(issuer)).to.be.true;
    // expect(currentAchievement.mint.equals(mint.publicKey)).to.be.true;
    // expect(Object.keys(currentAchievement.tier)[0]).to.be.equal("major");
    // expect(currentAchievement.bump).to.equal(bump);
  });
});
