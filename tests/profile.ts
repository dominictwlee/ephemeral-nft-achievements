import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Profile } from "../target/types/profile";
import { expect } from "chai";

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

  it("creates a profile", async () => {
    const tx = await program.rpc.create(
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
