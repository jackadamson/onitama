import React, { useCallback, useState } from 'react';
import { useSnackbar } from 'notistack';
import { useParams } from 'react-router';
import useSingleplayer from './hooks/useSingleplayer';
import Loading from './Loading';
import GameBoard from './GameBoard';

const getMoves = (src, card, turn) => {
  if (!src || !card) {
    return () => false;
  }
  const { moves } = card;
  const strMoves =
    turn === 'Red'
      ? moves.map(({ x, y }) => `${src.x + x},${src.y + y}`)
      : moves.map(({ x, y }) => `${src.x - x},${src.y - y}`);
  const dstSet = new Set(strMoves);
  return (x, y) => dstSet.has(`${x},${y}`);
};

function TrainingGame() {
  const { enqueueSnackbar } = useSnackbar();
  const { difficulty } = useParams();
  const { state, playMove, reset, undo, moveRankings } = useSingleplayer(difficulty, true);
  const [card, setCard] = useState(null);
  const [src, setSrc] = useState(null);
  const move = useCallback(
    (dst) => {
      if (!card || !src) {
        return;
      }
      if (!playMove) {
        enqueueSnackbar('Game loading, try again', { variant: 'warning' });
        return;
      }
      const action = { card: card.card, src, dst, type: 'Move' };
      const error = playMove(action);
      if (error) {
        enqueueSnackbar(error, { variant: 'error' });
      } else {
        setCard(null);
        setSrc(null);
      }
    },
    [playMove, src, card, enqueueSnackbar],
  );
  const discard = useCallback(
    (discardCard) => {
      if (!playMove) {
        enqueueSnackbar('Game loading, try again', { variant: 'warning' });
        return;
      }
      const action = { card: discardCard, type: 'Discard' };
      const error = playMove(action);
      if (error) {
        enqueueSnackbar(error, { variant: 'error' });
      } else {
        setCard(null);
        setSrc(null);
      }
    },
    [playMove, enqueueSnackbar],
  );
  if (!state) {
    return <Loading />;
  }
  const { blueCards, redCards, spare, turn, grid, canMove, winner, player, lastMove, canUndo } =
    state;
  const isMoveValid = getMoves(src, card, turn);
  const { max, min, ranksByCardSrc, stale } = moveRankings;
  const dstMoveRankings =
    state && player === turn && ranksByCardSrc && card && src
      ? ranksByCardSrc[`${card.card},${src.x},${src.y}`]
      : null;
  const unweightedScore = player === 'Red' ? max : -min;
  // This formula was not chosen carefully, it just seems maybe good enough 🤷
  const weightedScore =
    Math.abs(unweightedScore) < 1
      ? 0
      : Math.sign(unweightedScore) * Math.min(50, 6 * Math.log(Math.abs(unweightedScore)));
  const normalized = 50 + weightedScore;
  // eslint-disable-next-line no-console
  console.log({ unweightedScore, normalized, ranksByCardSrc });
  return (
    <GameBoard
      src={src}
      setSrc={setSrc}
      card={card}
      setCard={setCard}
      blueCards={blueCards}
      redCards={redCards}
      grid={grid}
      isMoveValid={isMoveValid}
      canMove={canMove}
      reset={reset}
      winner={winner}
      spare={spare}
      turn={turn}
      move={move}
      discard={discard}
      player={player}
      lastMove={lastMove}
      dstMoveRankings={dstMoveRankings}
      undo={undo}
      canUndo={canUndo}
      score={normalized}
      stale={stale}
    />
  );
}

export default TrainingGame;
