import crypto from "crypto";
import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Profile } from "../target/types/profile";
import { use, expect, should } from "chai";
import chaiAsPromised from "chai-as-promised";
import { ProfileLayout } from "./util";

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
  const delegate = anchor.web3.Keypair.generate();

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

  it("adds a delegate to profile", async () => {
    await program.rpc.addDelegate({
      accounts: {
        profile,
        owner: provider.wallet.publicKey,
        delegate: delegate.publicKey,
      },
      signers: [delegate],
    });

    const currentProfile = await program.account.profile.fetch(profile);

    expect(currentProfile.delegate.toBase58()).to.equal(
      delegate.publicKey.toBase58()
    );
  });
});

type Unwrap<T> = T extends Promise<infer U>
  ? U
  : T extends (...args: any) => Promise<infer U>
  ? U
  : T extends (...args: any) => infer U
  ? U
  : T;

type AsyncReturnType<T extends (...args: any) => any> = T extends (
  ...args: any
) => Promise<infer U>
  ? U
  : T extends (...args: any) => infer U
  ? U
  : any;
