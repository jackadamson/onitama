import React, { useCallback, useState } from 'react';
import { useSnackbar } from 'notistack';
import { useParams } from 'react-router';
import useSingleplayer from './hooks/useSingleplayer';
import Loading from './Loading';
import GameBoard from './GameBoard';
import getMoves from './utils/moveUtils';

function SinglePlayerGame() {
  const { enqueueSnackbar } = useSnackbar();
  const { difficulty } = useParams();
  const { state, playMove, reset } = useSingleplayer(difficulty);
  const [card, setCard] = useState(null);
  const [src, setSrc] = useState(null);

  const move = useCallback(
    ({ x, y, revealNinja }) => {
      if (!card || !src) {
        return;
      }
      if (!playMove) {
        enqueueSnackbar('Game loading, try again', { variant: 'warning' });
        return;
      }
      const action = {
        card: card.card,
        src,
        dst: { x, y },
        reveal_ninja: revealNinja,
        type: 'Move',
      };
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

  const {
    blueCards,
    redCards,
    spare,
    turn,
    grid,
    canMove,
    winner,
    player,
    lastMove,
    extraMovePending,
  } = state;

  const isMoveValid = getMoves(src, card, grid, turn, extraMovePending);

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
      extraMovePending={extraMovePending}
    />
  );
}

export default SinglePlayerGame;
