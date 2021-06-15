import React, { useCallback, useState } from 'react';
import { Box, Grid, Typography } from '@material-ui/core';
import useOnitama from './useOnitama';
import GameCard from './GameCard';
import GameGrid from './GameGrid';


const getMoves = (src, card, turn) => {
  if (!src || !card) {
    return () => false;
  }
  const { moves } = card;
  const strMoves = turn === 'Red'
    ? moves.map(({ x, y }) => `${src.x + x},${src.y + y}`)
    : moves.map(({ x, y }) => `${src.x - x},${src.y - y}`)
  const dstSet = new Set(strMoves);
  return (x, y) => dstSet.has(`${x},${y}`);
};

const App = () => {
  const { state, playMove } = useOnitama();
  console.log(state);
  const [card, setCard] = useState(null);
  const [src, setSrc] = useState(null);
  const move = useCallback((dst) => {
    console.log({ src, dst, card});
    if (!card || !src) {
      return;
    }
    const action = { card: card.card, src, dst, type: 'Move' };
    playMove(action);
    setCard(null);
    setSrc(null);
  }, [playMove, src, card]);

  if (!state) {
    return <Typography variant="h2">Loading...</Typography>;
  }
  const { blueCards, redCards, spare, turn, grid } = state;
  const isMove = getMoves(src, card, turn);
  console.log({ card, src });
  return (
    <Box height="100vh" display="flex">
      <Box display="flex" flexDirection="column" justifyContent="center" alignItems="center" flexBasis="33%">
        { turn ==='Blue' && <GameCard spare moves={spare.moves} enabled={false} setCard={setCard} name={spare.card} selected={false} /> }
      </Box>
      <Grid item xs={4} component={Box} display="flex" justifyContent="center" alignItems="center" flexGrow={1}>
        <Box display="flex" flexDirection="column">
          <Box display="flex" flexDirection="row" style={{ gap: '8px' }}>
            {blueCards.map(({ card: name, moves }) => (
              <GameCard
                setCard={setCard}
                name={name}
                selected={card?.card === name}
                key={name}
                moves={moves}
                enabled={turn === 'Blue'}
              />
          ))}
          </Box>
          <GameGrid isMove={isMove} move={move} src={src} setSrc={setSrc} grid={grid} turn={turn} />
          <Box display="flex" flexDirection="row" style={{ gap: '8px' }}>
            {redCards.map(({ card: name, moves }) => (
              <GameCard
                setCard={setCard}
                name={name}
                selected={card?.card === name}
                key={name}
                moves={moves}
                enabled={turn === 'Red'}
              />
            ))}
          </Box>
        </Box>
      </Grid>
      <Box display="flex" flexDirection="column" justifyContent="center" alignItems="center" flexBasis="33%">
        { turn ==='Red' && <GameCard spare moves={spare.moves} enabled={false} setCard={setCard} name={spare.card} selected={false} /> }
      </Box>
    </Box>
  );
};

export default App;
