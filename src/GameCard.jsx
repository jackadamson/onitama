import React from 'react';
import PropTypes from 'prop-types';
import { Box, makeStyles, Paper, Typography } from '@material-ui/core';
import clsx from 'clsx';
import * as R from 'ramda';

const useStyles = makeStyles((theme) => ({
  card: ({ enabled, spare }) => ({
    display: 'flex',
    flexDirection: 'column',
    flexBasis: spare ? null : '50%',
    maxWidth: spare ? '100%' : '50%',
    alignItems: 'center',
    padding: theme.spacing(1, 0, 3, 0),
    cursor: enabled ? 'pointer' : 'default',
    color: enabled ? theme.palette.common.white : theme.palette.action.disabled,
    backgroundColor: enabled ? theme.palette.background.paper : '#1a1d21',
    borderStyle: 'solid',
    borderWidth: '1px',
  }),
  selected: {
    borderColor: theme.palette.primary.main,
  },
  spare: {
    flexBasis: null,
    width: '156px',
    height: '142px',
  },
  moveCell: {
    width: '16px',
    height: '16px',
    borderStyle: 'solid',
    borderWidth: '1px',
    borderColor: theme.palette.grey['600'],
  },
  noMoves: {
    borderColor: theme.palette.error.main,
  },
  hasMoves: {
    borderColor: theme.palette.grey['600'],
  },
  origin: {
    backgroundColor: theme.palette.primary.light,
  },
  accessible: {
    backgroundColor: theme.palette.secondary.light,
  },
  error: {
    color: theme.palette.error.main,
  },
}));

const WrappedMoves = ({ moves }) => {
  console.log({ moves });
  const classes = useStyles({});
  const moveSet = new Set(moves.map(({ x, y }) => `${x},${y}`));
  const indexes = [-2, -1, 0, 1, 2];
  return (
    <Box display="flex" flexDirection="column">
      {indexes.map((y) => (
        <Box display="flex" flexDirection="row" key={y}>
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
WrappedMoves.propTypes = {
  moves: PropTypes.arrayOf(
    PropTypes.shape({
      x: PropTypes.number.isRequired,
      y: PropTypes.number.isRequired,
    }),
  ).isRequired,
};
const Moves = React.memo(WrappedMoves, R.equals);

const GameCard = ({ name, setCard, selected, enabled, moves, spare, canMove, discard }) => {
  const classes = useStyles({ enabled, spare });
  const handler = () => {
    if (enabled && !canMove) {
      discard(name);
    } else if (enabled) {
      setCard({ card: name, moves });
    }
  };
  return (
    <Paper
      className={clsx({
        [classes.card]: true,
        [classes.spare]: spare,
        [classes.noMoves]: enabled && !canMove,
        [classes.hasMoves]: enabled && canMove && !selected,
        [classes.selected]: selected,
      })}
      onClick={handler}
    >
      <Typography variant="subtitle1">{name}</Typography>
      <Moves moves={moves} />
      {!canMove && enabled && (
        <Typography className={classes.error} variant="caption">
          Discard
        </Typography>
      )}
    </Paper>
  );
};
GameCard.defaultProps = {
  spare: false,
  canMove: true,
  discard: () => {},
};
GameCard.propTypes = {
  selected: PropTypes.bool.isRequired,
  enabled: PropTypes.bool.isRequired,
  name: PropTypes.string.isRequired,
  setCard: PropTypes.func.isRequired,
  moves: PropTypes.arrayOf(
    PropTypes.shape({
      x: PropTypes.number.isRequired,
      y: PropTypes.number.isRequired,
    }),
  ).isRequired,
  spare: PropTypes.bool,
  canMove: PropTypes.bool,
  discard: PropTypes.func,
};

export default GameCard;
