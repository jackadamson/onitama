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
  accessible: {
    backgroundColor: theme.palette.secondary.light,
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

const GameMoves = ({ moves, inverted }) => {
  const classes = useStyles({ inverted });
  const moveSet = new Set(moves.map(({ x, y }) => `${x},${y}`));
  const indexes = [-2, -1, 0, 1, 2];
  return (
    <Box className={classes.grid}>
      {indexes.map((y) => (
        <Box className={classes.row} key={y}>
          {indexes.map((x) => {
            const keyed = `${x},${y}`;
            return (
              <Box
                key={keyed}
                className={clsx({
                  [classes.moveCell]: true,
                  [classes.origin]: x === 0 && y === 0,
                  [classes.accessible]: moveSet.has(keyed),
                })}
              />
            );
          })}
        </Box>
      ))}
    </Box>
  );
};
GameMoves.propTypes = {
  inverted: PropTypes.bool.isRequired,
  moves: PropTypes.arrayOf(
    PropTypes.shape({
      x: PropTypes.number.isRequired,
      y: PropTypes.number.isRequired,
    }),
  ).isRequired,
};
export default React.memo(GameMoves, R.equals);
