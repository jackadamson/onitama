/* eslint-disable react/no-array-index-key */
import React from 'react';
import PropTypes from 'prop-types';
import { Box, Paper } from '@material-ui/core';
import GameSquare from './GameSquare';
import { PointPropType } from './props';
import LastMove from './LastMove';

function GameGrid({
  grid,
  isMoveValid,
  src,
  setSrc,
  turn,
  move,
  lastMove,
  redOriented,
  dstMoveRankings,
}) {
  return (
    <Paper
      component={Box}
      display="flex"
      flexDirection={redOriented ? 'column' : 'column-reverse'}
      my={2}
    >
      <LastMove lastMove={lastMove} redOriented={redOriented} />
      {grid.map((row, y) => (
        <Box display="flex" flexDirection={redOriented ? 'row' : 'row-reverse'} key={y}>
          {row.map((tile, x) => {
            const isValid = isMoveValid(x, y);

            // Extract type and revealed for Ninja tiles
            const tileType =
              typeof tile === 'object' && tile !== null ? Object.keys(tile)[0] : tile;
            const revealed =
              typeof tile === 'object' && tile !== null ? (tile[tileType]?.revealed ?? true) : true;

            return (
              <GameSquare
                tile={tileType}
                revealed={revealed}
                x={x}
                y={y}
                src={src}
                setSrc={setSrc}
                turn={turn}
                move={move}
                isValid={isValid}
                key={`${x}-${y}`}
                lastMove={lastMove}
                ranking={(dstMoveRankings[`${x},${y}`] || 0) * (redOriented ? 1 : -1)}
              />
            );
          })}
        </Box>
      ))}
    </Paper>
  );
}

GameGrid.defaultProps = {
  src: null,
  turn: null,
  lastMove: null,
};

GameGrid.propTypes = {
  redOriented: PropTypes.bool.isRequired,
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
  lastMove: PropTypes.shape({
    dst: PointPropType.isRequired,
    src: PointPropType.isRequired,
  }),
  dstMoveRankings: PropTypes.objectOf(PropTypes.number).isRequired,
};

export default GameGrid;
