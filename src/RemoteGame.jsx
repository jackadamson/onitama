import React, { useCallback, useState } from 'react';
import PropTypes from 'prop-types';
import { useSnackbar } from 'notistack';
import { useParams } from 'react-router';
import Loading from './Loading';
import GameBoard from './GameBoard';
import useMultiplayer from './hooks/useMultiplayer';
import WaitingOverlay from './WaitingOverlay';

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

function RemoteGame({ isAi }) {
  const { roomId = null } = useParams();
  const { enqueueSnackbar } = useSnackbar();
  const { playMove, state, reset, reconnect } = useMultiplayer(roomId, isAi);
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
  // Host always creates game
  const { blueCards, redCards, spare, turn, grid, canMove, winner, player, lastMove, connection } =
    state;
  const isMoveValid = getMoves(src, card, turn);
  return (
    <>
      <WaitingOverlay state={state} reconnect={reconnect} />
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
        player={player}
        move={move}
        discard={discard}
        lastMove={lastMove}
        connectionStatus={connection}
      />
    </>
  );
}
RemoteGame.defaultProps = {
  isAi: false,
};
RemoteGame.propTypes = {
  isAi: PropTypes.bool,
};
export default RemoteGame;
