import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { assert } from "chai";
import { Tictactoe } from "../target/types/tictactoe";
import { fundWallet } from "./utils";

describe("tictactoe", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Tictactoe as Program<Tictactoe>;

  // global state
  const playerX = anchor.web3.Keypair.generate();
  const playerO = anchor.web3.Keypair.generate();
  let game: anchor.web3.Keypair;

  before("fund wallet", async () => {
    await fundWallet(program.provider.connection, playerX.publicKey, 2);
    await fundWallet(program.provider.connection, playerO.publicKey, 2);
  });

  it("create game", async () => {
    game = anchor.web3.Keypair.generate();

    await program.methods
      .createGame()
      .accounts({
        game: game.publicKey,
        player: playerX.publicKey,
      })
      .signers([playerX, game])
      .rpc();

    const gameAccount = await program.account.game.fetch(game.publicKey);

    assert.equal(gameAccount.playerX.toBase58(), playerX.publicKey.toBase58());
  });

  it("player joins", async () => {
    await program.methods
      .playerJoins()
      .accounts({
        game: game.publicKey,
        player: playerO.publicKey,
      })
      .signers([playerO])
      .rpc();

    const gameAccount = await program.account.game.fetch(game.publicKey);

    assert.equal(gameAccount.playerX.toBase58(), playerX.publicKey.toBase58());
    assert.equal(gameAccount.playerO.toBase58(), playerO.publicKey.toBase58());
    // yea enums generated by anchor 🙄
    assert.deepEqual(gameAccount.state, { started: {} });
  });

  it("player X make a move", async () => {
    await program.methods
      .playerMoves(0, 0)
      .accounts({
        game: game.publicKey,
        player: playerX.publicKey,
      })
      .signers([playerX])
      .rpc();

    const gameAccount = await program.account.game.fetch(game.publicKey);

    assert.deepEqual(gameAccount.board[0], { x: {} });
    assert.deepEqual(gameAccount.turn, { playerO: {} });
  });
});
