/* eslint-disable react/no-array-index-key */
import React from 'react';
import PropTypes from 'prop-types';
import { Box, Paper } from '@material-ui/core';
import GameSquare from './GameSquare';

const GameGrid = ({ grid, isMoveValid, src, setSrc, turn, move }) => (
  <Paper component={Box} display="flex" flexDirection="column" my={2}>
    {grid.map((row, y) => (
      <Box display="flex" flexDirection="row" key={y}>
        {row.map((tile, x) => (
          <GameSquare
            tile={tile}
            x={x}
            y={y}
            src={src}
            setSrc={setSrc}
            turn={turn}
            move={move}
            isValid={isMoveValid(x, y)}
            key={`${x}-${y}`}
          />
        ))}
      </Box>
    ))}
  </Paper>
);
GameGrid.defaultProps = {
  src: null,
  turn: null,
};
GameGrid.propTypes = {
  grid: PropTypes.arrayOf(PropTypes.arrayOf(PropTypes.string).isRequired).isRequired,
  isMoveValid: PropTypes.func.isRequired,
  src: PropTypes.shape({
    x: PropTypes.number.isRequired,
    y: PropTypes.number.isRequired,
  }),
  setSrc: PropTypes.func.isRequired,
  // Null turn means opponents turn in multiplayer
  turn: PropTypes.oneOf(['Red', 'Blue', null]),
  move: PropTypes.func.isRequired,
};

export default GameGrid;
