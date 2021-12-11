import crypto from "crypto";
import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Profile } from "../target/types/profile";
import { use, expect, should } from "chai";
import chaiAsPromised from "chai-as-promised";

should();
use(chaiAsPromised);

describe("Profile program", async () => {
  anchor.setProvider(anchor.Provider.env());
  const provider = anchor.getProvider();
  const program = anchor.workspace.Profile as Program<Profile>;
  const alias = "dummyuser";
  const detailsUri = "www.some-resource-link.com";
  const [profile, profileBump] = await anchor.web3.PublicKey.findProgramAddress(
    [Buffer.from("profile"), Buffer.from(alias)],
    program.programId
  );

  it("checks character length for uri", async () => {
    const invalidUri =
      "wB2pXL73v9pSOEeUsay0Ud15paqDeBYwiHgB6bNuewbTmPhujPVSt1X4lTQOidgfyUqPzx3K2t2GVCSnss2UOe3QNlfAAno5gbCF9x5fjnG8DjzZBe6uY1ITqiq8HPGxnxBYYrFVAvv7PJFnV4EFLYDxyBMecDnOvDsd50u096cVUAqLkuQuuPKRzOSwKH9n0erMCAhgP";

    await program.rpc
      .create(
        {
          alias,
          detailsUri: invalidUri,
          bump: profileBump,
        },
        {
          accounts: {
            profile,
            owner: provider.wallet.publicKey,
            systemProgram: anchor.web3.SystemProgram.programId,
          },
        }
      )
      .should.eventually.be.rejectedWith("URI character length exceeded");
  });

  it("checks character length for alias", async () => {
    const longAlias = "Z5kRSdM8hbvUmc0Zl1jAZ4prpqi2qWEA";
    const [profile, profileBump] =
      await anchor.web3.PublicKey.findProgramAddress(
        [Buffer.from("profile"), Buffer.from(longAlias)],
        program.programId
      );

    await program.rpc
      .create(
        {
          alias: longAlias,
          detailsUri,
          bump: profileBump,
        },
        {
          accounts: {
            profile,
            owner: provider.wallet.publicKey,
            systemProgram: anchor.web3.SystemProgram.programId,
          },
        }
      )
      .should.eventually.be.rejectedWith("Alias character length exceeded");
  });

  it("creates a profile", async () => {
    await program.rpc.create(
      {
        alias,
        detailsUri,
        bump: profileBump,
      },
      {
        accounts: {
          profile,
          owner: provider.wallet.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        },
      }
    );

    const currentProfile = await program.account.profile.fetch(profile);

    expect(currentProfile.alias).to.be.equal(alias);
    expect(currentProfile.bump).to.be.equal(profileBump);
    expect(currentProfile.detailsUri).to.be.equal(detailsUri);
    expect(currentProfile.owner.equals(provider.wallet.publicKey)).to.be.true;
  });
});
