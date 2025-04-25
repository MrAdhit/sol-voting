import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Voting } from "../target/types/voting";
import bs58 from "bs58";

describe("voting", () => {
  anchor.setProvider(anchor.AnchorProvider.env());

  // const conn = new anchor.web3.Connection("http://127.0.0.1:8899", "confirmed");
  // const kp = anchor.web3.Keypair.generate();
  // const wallet = new anchor.Wallet(kp);
  // anchor.setProvider(new anchor.AnchorProvider(conn, wallet));

  console.log(anchor.getProvider());

  const program = anchor.workspace.voting as Program<Voting>;
  
  console.log(anchor.getProvider().publicKey.toBase58());

  it("Votes", async () => {
    const poll_id = new anchor.BN(Math.floor(Math.random() * 10000000));
    
    const poll_tx = await program.methods.createPoll(
      poll_id,
      "The best element",
      "What is the best element?",
      new anchor.BN(Date.now()),
    ).rpc();
    console.log("Poll transaction signature", poll_tx);
    
    const al_candidate_tx = await program.methods.createCandidate(
      poll_id,
      "Aluminium"
    ).rpc();
    console.log("Aluminium candidate transaction signature", al_candidate_tx);

    const cu_candidate_tx = await program.methods.createCandidate(
      poll_id,
      "Copper"
    ).rpc();
    console.log("Copper candidate transaction signature", cu_candidate_tx);

    const c_candidate_tx = await program.methods.createCandidate(
      poll_id,
      "Carbon"
    ).rpc();
    console.log("Carbon candidate transaction signature", c_candidate_tx);
    
    const fe_candidate_tx = await program.methods.createCandidate(
      poll_id,
      "Iron"
    ).rpc();
    console.log("Iron candidate transaction signature", fe_candidate_tx);

    const au_candidate_tx = await program.methods.createCandidate(
      poll_id,
      "Gold"
    ).rpc();
    console.log("Gold candidate transaction signature", au_candidate_tx);
    
    const [ poll ] = await program.account.pollAccount.all([
      {
        memcmp: {
          offset: 8,
          bytes: bs58.encode(poll_id.toBuffer("le", 8)),
        }
      }
    ]);
    
    console.log({ poll });
    
    const candidates = await program.account.candidateAccount.all([
      {
        memcmp: {
          offset: 8,
          bytes: bs58.encode(poll_id.toBuffer("le", 8)),
        }
      }
    ]);
    
    console.dir({ candidates }, { depth: null });
    
    for (let index = 0; index < 10; index++) {
      const chosen_candidate = candidates[Math.floor(Math.random() * candidates.length)];
      console.log(`Voting candidate ${chosen_candidate.account.candidateName}`);

      const vote_tx = await program.methods.vote().accounts({
        pollAccount: poll.publicKey,
        candidateAccount: chosen_candidate.publicKey,
      }).rpc();

      console.log({ vote_tx });
    }

    const result_candidates = await program.account.candidateAccount.all([
      {
        memcmp: {
          offset: 8,
          bytes: bs58.encode(poll_id.toBuffer("le", 8)),
        }
      }
    ]);

    console.dir({ candidates_result: result_candidates.map(c => ({ name: c.account.candidateName, votes: c.account.candidateVotes.toNumber() })) }, { depth: null });
  });
});
