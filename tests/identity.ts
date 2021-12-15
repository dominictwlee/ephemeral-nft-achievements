import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { ClusterType, register, resolve } from "@identity.com/sol-did-client";
import { Identity } from "../target/types/identity";
import { use, expect, should } from "chai";
import chaiAsPromised from "chai-as-promised";
import { SOL_DID_PUBLIC_KEY } from "./util";

should();
use(chaiAsPromised);

describe("Identity program", async () => {
  anchor.setProvider(anchor.Provider.env());
  const provider = anchor.getProvider();
  const program = anchor.workspace.Identity as Program<Identity>;
  const name = "dummyuser";
  const detailsUri = "www.some-resource-link.com";
  const didOwner = anchor.web3.Keypair.generate();
  const [didAccount, didAccountBump] =
    await anchor.web3.PublicKey.findProgramAddress(
      [didOwner.publicKey.toBuffer(), Buffer.from("sol", "utf8")],
      SOL_DID_PUBLIC_KEY
    );
  const [profile, profileBump] = await anchor.web3.PublicKey.findProgramAddress(
    [Buffer.from("profile"), didAccount.toBuffer()],
    program.programId
  );

  it("checks character length for details uri", async () => {
    const invalidUri =
      "wB2pXL73v9pSOEeUsay0Ud15paqDeBYwiHgB6bNuewbTmPhujPVSt1X4lTQOidgfyUqPzx3K2t2GVCSnss2UOe3QNlfAAno5gbCF9x5fjnG8DjzZBe6uY1ITqiq8HPGxnxBYYrFVAvv7PJFnV4EFLYDxyBMecDnOvDsd50u096cVUAqLkuQuuPKRzOSwKH9n0erMCAhgP";

    await program.rpc
      .createProfile(
        {
          name,
          detailsUri: invalidUri,
          bump: profileBump,
        },
        {
          accounts: {
            profile,
            solDid: didAccount,
            owner: didOwner.publicKey,
            payer: provider.wallet.publicKey,
            systemProgram: anchor.web3.SystemProgram.programId,
          },
          signers: [didOwner],
        }
      )
      .should.eventually.be.rejectedWith("URI character length exceeded");
  });

  it("checks character length for profile name", async () => {
    const invalidName = "Z5kRSdM8hbvUmc0Zl1jAZ4prpqi2qWEA";

    await program.rpc
      .createProfile(
        {
          name: invalidName,
          detailsUri,
          bump: profileBump,
        },
        {
          accounts: {
            profile,
            solDid: didAccount,
            payer: provider.wallet.publicKey,
            owner: didOwner.publicKey,
            systemProgram: anchor.web3.SystemProgram.programId,
          },
          signers: [didOwner],
        }
      )
      .should.eventually.be.rejectedWith(
        "Profile name character length exceeded"
      );
  });

  it("creates a profile", async () => {
    const identifier = await register({
      payer: anchor.Wallet.local().payer.secretKey,
      owner: didOwner.publicKey.toBase58(),
      cluster: ClusterType.development(),
      connection: provider.connection,
    });

    await program.rpc.createProfile(
      {
        name,
        detailsUri,
        bump: profileBump,
      },
      {
        accounts: {
          profile,
          payer: provider.wallet.publicKey,
          owner: didOwner.publicKey,
          solDid: didAccount,
          systemProgram: anchor.web3.SystemProgram.programId,
        },
        signers: [didOwner],
      }
    );

    const currentProfile = await program.account.profile.fetch(profile);

    expect(currentProfile.name).to.be.equal(name);
    expect(currentProfile.bump).to.be.equal(profileBump);
    expect(currentProfile.detailsUri).to.be.equal(detailsUri);
    expect(currentProfile.owner.equals(didAccount)).to.be.true;
  });

  // it("adds a delegate to profile", async () => {
  //   await program.rpc.addDelegate({
  //     accounts: {
  //       profile,
  //       owner: provider.wallet.publicKey,
  //       delegate: delegate.publicKey,
  //     },
  //     signers: [delegate],
  //   });

  //   const currentProfile = await program.account.profile.fetch(profile);

  //   expect(currentProfile.delegates[0].toBase58()).to.equal(
  //     delegate.publicKey.toBase58()
  //   );
  // });
});
