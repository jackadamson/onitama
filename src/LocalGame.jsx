import React, { useCallback, useState } from 'react';
import { useSnackbar } from 'notistack';
import useLocalGame from './hooks/useLocalGame';
import Loading from './Loading';
import GameBoard from './GameBoard';
import getMoves from './utils/moveUtils';

function LocalGame() {
  const { enqueueSnackbar } = useSnackbar();
  const { state, playMove, reset } = useLocalGame();
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

  const { blueCards, redCards, spare, turn, grid, canMove, winner, extraMovePending } = state;

  // Determine if a king is selected
  const isKingSelected = src && grid[src.y]?.[src.x]?.includes('King');
  const isWindSpiritSelected = src && grid[src.y]?.[src.x]?.includes('Spirit');

  // Use the centralized getMoves function
  const isMoveValid = getMoves(src, card, turn, isKingSelected, isWindSpiritSelected, extraMovePending );

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
      extraMovePending={extraMovePending}
    />
  );
}

export default LocalGame;
