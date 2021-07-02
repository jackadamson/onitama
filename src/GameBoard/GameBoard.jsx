import React from 'react';
import PropTypes from 'prop-types';
import { Link } from 'react-router-dom';
import { Box, Button, useMediaQuery, useTheme } from '@material-ui/core';
import GameOver from './GameOver';
import GameCard from './GameCard';
import GameGrid from './GameGrid';
import GameHand from './GameHand';
import GameTurn from './GameTurn';

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
  const theme = useTheme();
  const hideSideSpare = useMediaQuery(theme.breakpoints.down('sm'));
  // Whether it's the player's turn, always true if local multiplayer
  const playerTurn = player ? player === turn : true;
  return (
    <Box height="100vh" display="flex" flexDirection="column">
      <Box display="flex" justifyContent="center">
        <GameTurn player={player} turn={turn} />
      </Box>
      <Box display="flex" flexGrow={1}>
        <Box position="absolute" top="0" left="0">
          <Button component={Link} to="/">
            Home
          </Button>
        </Box>
        <GameOver reset={reset} winner={winner} player={player} />
        <Box
          display={hideSideSpare ? 'none' : 'flex'}
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
            <GameHand
              setCard={setCard}
              selectedCard={card}
              spare={spare}
              discard={discard}
              canMove={canMove}
              cards={blueCards}
              enabled={turn === 'Blue' && playerTurn}
              isPlayerTurn={turn === 'Blue'}
              inverted
            />
            <GameGrid
              isMoveValid={isMoveValid}
              move={move}
              src={src}
              setSrc={setSrc}
              grid={grid}
              turn={turn}
            />
            <GameHand
              setCard={setCard}
              selectedCard={card}
              spare={spare}
              discard={discard}
              canMove={canMove}
              cards={redCards}
              enabled={turn === 'Red' && playerTurn}
              isPlayerTurn={turn === 'Red'}
            />
          </Box>
        </Box>
        <Box
          display={hideSideSpare ? 'none' : 'flex'}
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
