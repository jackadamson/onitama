/* eslint-disable react/no-array-index-key */
import React from 'react';
import PropTypes from 'prop-types';
import { Box, Paper } from '@material-ui/core';
import GameSquare from './GameSquare';

const GameGrid = ({ grid, isMove, src, setSrc, turn, move }) => (
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
            isValid={isMove(x, y)}
            key={`${x}-${y}`}
          />
        ))}
      </Box>
    ))}
  </Paper>
);
GameGrid.defaultProps = {
  src: null,
};
GameGrid.propTypes = {
  grid: PropTypes.arrayOf(PropTypes.arrayOf(PropTypes.string).isRequired).isRequired,
  isMove: PropTypes.func.isRequired,
  src: PropTypes.shape({
    x: PropTypes.number.isRequired,
    y: PropTypes.number.isRequired,
  }),
  setSrc: PropTypes.func.isRequired,
  turn: PropTypes.string.isRequired,
  move: PropTypes.func.isRequired,
};

export default GameGrid;
