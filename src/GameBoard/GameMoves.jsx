import React from 'react';
import PropTypes from 'prop-types';
import { Box, makeStyles } from '@material-ui/core';
import clsx from 'clsx';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import { faChessKing, faChessQueen } from '@fortawesome/free-solid-svg-icons';
import * as R from 'ramda';

const useStyles = makeStyles((theme) => ({
  origin: {
    backgroundColor: theme.palette.primary.light,
    position: 'relative',
    display: 'flex',
    alignItems: 'center',
    justifyContent: 'center',
    width: ({ isKingMoves, isWindMoves }) => (isKingMoves || isWindMoves ? '12px' : '16px'),
    height: ({ isKingMoves, isWindMoves }) => (isKingMoves || isWindMoves ? '12px' : '16px'),
  },
  moveCell: {
    width: ({ isKingMoves, isWindMoves }) => (isKingMoves || isWindMoves ? '12px' : '16px'),
    height: ({ isKingMoves, isWindMoves }) => (isKingMoves || isWindMoves ? '12px' : '16px'),
    borderStyle: 'solid',
    borderWidth: '1px',
    borderColor: theme.palette.grey['600'],
  },
  directionBalanced: {
    backgroundColor: theme.palette.direction.balanced,
  },
  directionLeft: {
    backgroundColor: theme.palette.direction.left,
  },
  directionRight: {
    backgroundColor: theme.palette.direction.right,
  },
  grid: ({ inverted }) => ({
    display: 'flex',
    flexDirection: inverted ? 'column-reverse' : 'column',
  }),
  row: ({ inverted }) => ({
    display: 'flex',
    flexDirection: inverted ? 'row-reverse' : 'row',
  }),
  secondGridOrigin: {
    position: 'relative',
    display: 'flex',
    alignItems: 'center',
    justifyContent: 'center',
    backgroundColor: 'transparent', // Remove background for second grid origin
  },
}));

function GameMoves({ moves, inverted, direction, isKingMoves, isWindMoves, isSecondGrid }) {
  const classes = useStyles({ inverted, isKingMoves, isWindMoves });
  const moveSet = new Set(moves.map(({ x, y }) => `${x},${y}`));

  // Determine the dynamic range for two-grid cards (WindMove or KingMove)
  const isSpecialGrid = isKingMoves || isWindMoves;
  const shouldUseAlternateRangeY = moves.some(({ y }) => y > 0 || y < -2);

  const indexesX = [-2, -1, 0, 1, 2];
  let indexesY;
  if (isSpecialGrid) {
    if (shouldUseAlternateRangeY) {
      indexesY = [-1, 0, 1];
    } else {
      indexesY = [-2, -1, 0];
    }
  } else {
    indexesY = [-2, -1, 0, 1, 2];
  }

  return (
    <Box className={classes.grid}>
      {indexesY.map((y) => (
        <Box className={classes.row} key={y}>
          {indexesX.map((x) => {
            const keyed = `${x},${y}`;
            const accessible = moveSet.has(keyed);
            return (
              <Box
                key={keyed}
                className={clsx({
                  [classes.moveCell]: true,
                  [classes.origin]:
                    x === 0 &&
                    y === 0 &&
                    !(isKingMoves && isSecondGrid) &&
                    !(isWindMoves && isSecondGrid),
                  [classes.secondGridOrigin]:
                    x === 0 && y === 0 && (isKingMoves || isWindMoves) && isSecondGrid,
                  [classes.directionBalanced]:
                    accessible && (direction === 'Balanced' || (isWindMoves && isSecondGrid)),
                  [classes.directionLeft]:
                    accessible && direction === 'Left' && !(isWindMoves && isSecondGrid),
                  [classes.directionRight]:
                    accessible && direction === 'Right' && !(isWindMoves && isSecondGrid),
                })}
              >
                {isSecondGrid && (isKingMoves || isWindMoves) && x === 0 && y === 0 && (
                  <FontAwesomeIcon
                    icon={isKingMoves ? faChessKing : faChessQueen}
                    style={{
                      color: isKingMoves ? 'white' : '#C2C2C2',
                      fontSize: '10px',
                      position: 'relative',
                      margin: 'auto',
                      transform: inverted ? 'rotate(180deg)' : 'none',
                    }}
                  />
                )}
              </Box>
            );
          })}
        </Box>
      ))}
    </Box>
  );
}

GameMoves.defaultProps = {
  isKingMoves: false,
  isWindMoves: false,
  isSecondGrid: false,
};

GameMoves.propTypes = {
  inverted: PropTypes.bool.isRequired,
  moves: PropTypes.arrayOf(
    PropTypes.shape({
      x: PropTypes.number.isRequired,
      y: PropTypes.number.isRequired,
    }),
  ).isRequired,
  direction: PropTypes.string.isRequired,
  isKingMoves: PropTypes.bool,
  isWindMoves: PropTypes.bool,
  isSecondGrid: PropTypes.bool,
};

export default React.memo(GameMoves, R.equals);
