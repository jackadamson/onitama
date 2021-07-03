import React from 'react';
import PropTypes from 'prop-types';
import { Link } from 'react-router-dom';
import {
  makeStyles,
  Dialog,
  DialogActions,
  DialogTitle,
  Button,
  Typography,
} from '@material-ui/core';
import clsx from 'clsx';

const useStyles = makeStyles((theme) => ({
  blue: {
    color: theme.palette.info.main,
  },
  red: {
    color: theme.palette.error.main,
  },
  dialog: {
    padding: theme.spacing(8),
    display: 'flex',
    flexDirection: 'column',
    justifyContent: 'center',
    alignItems: 'center',
  },
}));
const captionsFromStatus = {
  OpponentRematchRequested: 'Opponent requested a rematch',
  RematchRequested: 'Rematch request sent',
};
const GameOver = ({ winner, reset, player, connectionStatus }) => {
  const classes = useStyles();
  const relativeText = player === winner ? 'You Win!' : 'You Lose!';
  const absoluteText = `${winner} wins!`;
  const text = player ? relativeText : absoluteText;
  const caption = captionsFromStatus[connectionStatus];
  return (
    <Dialog open={Boolean(winner)} classes={{ paper: classes.dialog }}>
      <DialogTitle
        className={clsx({
          [classes.red]: winner === 'Red',
          [classes.blue]: winner === 'Blue',
        })}
      >
        {text}
      </DialogTitle>
      <DialogActions>
        <Button variant="contained" onClick={reset} color="primary">
          Rematch
        </Button>
        <Button variant="outlined" color="secondary" component={Link} to="/">
          Main Menu
        </Button>
      </DialogActions>
      <Typography variant="subtitle1">{caption}</Typography>
    </Dialog>
  );
};
GameOver.defaultProps = {
  winner: null,
  player: null,
  connectionStatus: null,
};
GameOver.propTypes = {
  winner: PropTypes.oneOf(['Red', 'Blue', null]),
  player: PropTypes.oneOf(['Red', 'Blue', null]),
  connectionStatus: PropTypes.string,
  reset: PropTypes.func.isRequired,
};

export default GameOver;
