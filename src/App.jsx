import React, { useCallback, useState } from 'react';
import useOnitama from './useOnitama';
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

const App = () => {
  const { state, playMove, reset } = useOnitama();
  const [card, setCard] = useState(null);
  const [src, setSrc] = useState(null);
  const move = useCallback(
    (dst) => {
      if (!card || !src) {
        return;
      }
      const action = { card: card.card, src, dst, type: 'Move' };
      playMove(action);
      setCard(null);
      setSrc(null);
    },
    [playMove, src, card],
  );
  const discard = useCallback(
    (discardCard) => {
      const action = { card: discardCard, type: 'Discard' };
      playMove(action);
    },
    [playMove],
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

export default App;
