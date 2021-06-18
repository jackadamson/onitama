import React, { useCallback, useState } from 'react';
import { useSnackbar } from 'notistack';
import { useParams } from 'react-router';
import useOnitama from './useOnitama';
import Loading from './Loading';
import GameBoard from './GameBoard';
import useSocket from './useSocket';

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

const RemoteGame = () => {
  const { roomId = null } = useParams();
  console.log({ roomId });
  const { enqueueSnackbar } = useSnackbar();
  const { state, playMove, reset, importState, exportState } = useOnitama();
  const { playLocalMove, room } = useSocket(roomId, playMove, importState, exportState);
  const [card, setCard] = useState(null);
  const [src, setSrc] = useState(null);
  const move = useCallback(
    (dst) => {
      if (!card || !src) {
        return;
      }
      if (!playLocalMove) {
        enqueueSnackbar('Game loading, try again', { variant: 'warning' });
        return;
      }
      const action = { card: card.card, src, dst, type: 'Move' };
      const error = playLocalMove(action);
      if (error) {
        enqueueSnackbar(error, { variant: 'error' });
      } else {
        setCard(null);
        setSrc(null);
      }
    },
    [playLocalMove, src, card, enqueueSnackbar],
  );
  const discard = useCallback(
    (discardCard) => {
      if (!playLocalMove) {
        enqueueSnackbar('Game loading, try again', { variant: 'warning' });
        return;
      }
      const action = { card: discardCard, type: 'Discard' };
      const error = playLocalMove(action);
      if (error) {
        enqueueSnackbar(error, { variant: 'error' });
      } else {
        setCard(null);
        setSrc(null);
      }
    },
    [playLocalMove, enqueueSnackbar],
  );
  if (!state) {
    return <Loading />;
  }
  const { blueCards, redCards, spare, turn, grid, canMove, winner } = state;
  const isMoveValid = getMoves(src, card, turn);
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
    />
  );
};

export default RemoteGame;
