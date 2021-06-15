import React from 'react';
import PropTypes from 'prop-types';
import { Box, makeStyles, Paper, Typography } from '@material-ui/core';
import clsx from 'clsx';

const useStyles = makeStyles((theme) => ({
  card: ({ selected, enabled, spare }) => ({
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
    borderColor: enabled && selected ? theme.palette.primary.main : 'rgba(0, 0, 0, 0)',
  }),
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
  origin: {
    backgroundColor: theme.palette.primary.light,
  },
  accessible: {
    backgroundColor: theme.palette.secondary.light,
  },
}));

const Moves = ({ moves }) => {
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
Moves.propTypes = {
  moves: PropTypes.arrayOf(
    PropTypes.shape({
      x: PropTypes.number.isRequired,
      y: PropTypes.number.isRequired,
    }),
  ).isRequired,
};
const GameCard = ({ name, setCard, selected, enabled, moves, spare }) => {
  const classes = useStyles({ selected, enabled, spare });
  const handler = () => {
    console.log({ enabled, name, moves });
    if (enabled) {
      setCard({ card: name, moves });
    }
  };
  return (
    <Paper className={clsx(classes.card, spare && classes.spare)} onClick={handler}>
      <Typography variant="subtitle1">{name}</Typography>
      <Moves moves={moves} />
    </Paper>
  );
};
GameCard.defaultProps = {
  spare: false,
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
};

export default GameCard;
