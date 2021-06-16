import React from 'react';
import PropTypes from 'prop-types';
import { makeStyles, Paper, Typography } from '@material-ui/core';
import clsx from 'clsx';
import GameMove from './GameMoves';

const useStyles = makeStyles((theme) => ({
  card: ({ enabled, spare, inverted }) => ({
    display: 'flex',
    flexDirection: inverted ? 'column-reverse' : 'column',
    flexBasis: spare ? null : '50%',
    maxWidth: spare ? '100%' : '50%',
    height: '142px',
    alignItems: 'center',
    padding: theme.spacing(1, 0),
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
  noMoves: {
    borderColor: theme.palette.error.main,
  },
  hasMoves: {
    borderColor: theme.palette.grey['600'],
  },
  error: {
    color: theme.palette.error.main,
  },
}));

const GameCard = ({
  name,
  setCard,
  selected,
  enabled,
  moves,
  spare,
  canMove,
  discard,
  inverted,
}) => {
  const classes = useStyles({ enabled, spare, inverted });
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
      <GameMove moves={moves} inverted={inverted} />
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
  inverted: false,
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
  inverted: PropTypes.bool,
};

export default GameCard;
