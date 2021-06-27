import React from 'react';
import PropTypes from 'prop-types';
import { Box, Button } from '@material-ui/core';
import GameOver from './GameOver';
import GameCard from './GameCard';
import GameGrid from './GameGrid';

const GameBoard = ({
  src,
  setSrc,
  grid,
  winner,
  player,
  turn,
  spare,
  blueCards,
  redCards,
  setCard,
  card,
  canMove,
  isMoveValid,
  move,
  discard,
  reset,
}) => {
  // Whether it's the player's turn, always true if local multiplayer
  const playerTurn = player ? player === turn : true;
  return (
    <Box height="100vh" display="flex">
      {reset && (
        <Box position="fixed" x={0} y={0}>
          <Button onClick={reset}>Reset</Button>
        </Box>
      )}
      <GameOver reset={reset} winner={winner} />
      <Box
        display="flex"
        flexDirection="column"
        justifyContent="center"
        alignItems="center"
        flexBasis="33%"
      >
        {turn === 'Blue' && (
          <GameCard
            spare
            inverted
            moves={spare.moves}
            enabled={false}
            setCard={setCard}
            name={spare.card}
            selected={false}
          />
        )}
      </Box>
      <Box display="flex" justifyContent="center" alignItems="center" flexGrow={1}>
        <Box display="flex" flexDirection="column">
          <Box display="flex" flexDirection="row" style={{ gap: '8px' }}>
            {blueCards.map(({ card: name, moves }) => (
              <GameCard
                inverted
                setCard={setCard}
                name={name}
                selected={card?.card === name}
                key={name}
                moves={moves}
                enabled={turn === 'Blue' && playerTurn}
                canMove={canMove}
                discard={discard}
              />
            ))}
          </Box>
          <GameGrid
            isMoveValid={isMoveValid}
            move={move}
            src={src}
            setSrc={setSrc}
            grid={grid}
            turn={turn}
          />
          <Box display="flex" flexDirection="row" style={{ gap: '8px' }}>
            {redCards.map(({ card: name, moves }) => (
              <GameCard
                setCard={setCard}
                name={name}
                selected={card?.card === name}
                key={name}
                moves={moves}
                enabled={turn === 'Red' && playerTurn}
                canMove={canMove}
                discard={discard}
              />
            ))}
          </Box>
        </Box>
      </Box>
      <Box
        display="flex"
        flexDirection="column"
        justifyContent="center"
        alignItems="center"
        flexBasis="33%"
      >
        {turn === 'Red' && (
          <GameCard
            spare
            moves={spare.moves}
            enabled={false}
            setCard={setCard}
            name={spare.card}
            selected={false}
          />
        )}
      </Box>
    </Box>
  );
};
const PointPropType = PropTypes.shape({
  x: PropTypes.number.isRequired,
  y: PropTypes.number.isRequired,
});
const CardPropType = PropTypes.shape({
  card: PropTypes.string.isRequired,
  moves: PropTypes.arrayOf(PointPropType).isRequired,
});
GameBoard.defaultProps = {
  card: null,
  src: null,
  winner: null,
  reset: null,
  player: null,
};
GameBoard.propTypes = {
  src: PointPropType,
  setSrc: PropTypes.func.isRequired,
  grid: PropTypes.arrayOf(PropTypes.arrayOf(PropTypes.string).isRequired).isRequired,
  winner: PropTypes.oneOf(['Red', 'Blue', null]),
  reset: PropTypes.func,
  turn: PropTypes.oneOf(['Red', 'Blue']).isRequired,
  player: PropTypes.oneOf(['Red', 'Blue', null]),
  spare: CardPropType.isRequired,
  blueCards: PropTypes.arrayOf(CardPropType).isRequired,
  redCards: PropTypes.arrayOf(CardPropType).isRequired,
  setCard: PropTypes.func.isRequired,
  card: CardPropType,
  canMove: PropTypes.bool.isRequired,
  isMoveValid: PropTypes.func.isRequired,
  move: PropTypes.func.isRequired,
  discard: PropTypes.func.isRequired,
};

export default GameBoard;
