import React from 'react';
import PropTypes from 'prop-types';
import { Box, makeStyles } from '@material-ui/core';
import clsx from 'clsx';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import { faChessKing } from '@fortawesome/free-solid-svg-icons';
import * as R from 'ramda';

const useStyles = makeStyles((theme) => ({
  origin: {
    backgroundColor: theme.palette.primary.light,
    position: 'relative',
    display: 'flex',
    alignItems: 'center',
    justifyContent: 'center',
    width: ({ isKingMoves }) => (isKingMoves ? '12px' : '16px'),
    height: ({ isKingMoves }) => (isKingMoves ? '12px' : '16px'),
  },
  moveCell: {
    width: ({ isKingMoves }) => (isKingMoves ? '12px' : '16px'),
    height: ({ isKingMoves }) => (isKingMoves ? '12px' : '16px'),
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
  },
}));

function GameMoves({ moves, inverted, direction, isKingMoves, isSecondGrid }) {
  const classes = useStyles({ inverted, isKingMoves });
  const moveSet = new Set(moves.map(({ x, y }) => `${x},${y}`));
  const indexes = isKingMoves ? [-2, -1, 0] : [-2, -1, 0, 1, 2];
  return (
    <Box className={classes.grid}>
      {indexes.map((y) => (
        <Box className={classes.row} key={y}>
          {[-2, -1, 0, 1, 2].map((x) => {
            const keyed = `${x},${y}`;
            const accessible = moveSet.has(keyed);
            return (
              <Box
                key={keyed}
                className={clsx({
                  [classes.moveCell]: true,
                  [classes.origin]: x === 0 && y === 0 && !(isKingMoves && isSecondGrid),
                  [classes.secondGridOrigin]: x === 0 && y === 0 && isKingMoves && isSecondGrid,
                  [classes.directionBalanced]: accessible && direction === 'Balanced',
                  [classes.directionLeft]: accessible && direction === 'Left',
                  [classes.directionRight]: accessible && direction === 'Right',
                })}
              >
                {isKingMoves && isSecondGrid && x === 0 && y === 0 && (
                  <FontAwesomeIcon
                    icon={faChessKing}
                    style={{
                      color: 'white',
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
  isSecondGrid: PropTypes.bool,
};

export default React.memo(GameMoves, R.equals);
