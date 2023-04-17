import React from 'react';
import PropTypes from 'prop-types';
import { Box, makeStyles } from '@material-ui/core';
import clsx from 'clsx';
import * as R from 'ramda';

const useStyles = makeStyles((theme) => ({
  origin: {
    backgroundColor: theme.palette.primary.light,
  },
  moveCell: {
    width: '16px',
    height: '16px',
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
}));

function GameMoves({ moves, inverted, direction }) {
  const classes = useStyles({ inverted });
  const moveSet = new Set(moves.map(({ x, y }) => `${x},${y}`));
  const indexes = [-2, -1, 0, 1, 2];
  return (
    <Box className={classes.grid}>
      {indexes.map((y) => (
        <Box className={classes.row} key={y}>
          {indexes.map((x) => {
            const keyed = `${x},${y}`;
            const accessible = moveSet.has(keyed);
            return (
              <Box
                key={keyed}
                className={clsx({
                  [classes.moveCell]: true,
                  [classes.origin]: x === 0 && y === 0,
                  [classes.directionBalanced]: accessible && direction === 'Balanced',
                  [classes.directionLeft]: accessible && direction === 'Left',
                  [classes.directionRight]: accessible && direction === 'Right',
                })}
              />
            );
          })}
        </Box>
      ))}
    </Box>
  );
}
GameMoves.propTypes = {
  inverted: PropTypes.bool.isRequired,
  moves: PropTypes.arrayOf(
    PropTypes.shape({
      x: PropTypes.number.isRequired,
      y: PropTypes.number.isRequired,
    }),
  ).isRequired,
  direction: PropTypes.string.isRequired,
};
export default React.memo(GameMoves, R.equals);
