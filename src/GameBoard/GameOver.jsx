import React from 'react';
import PropTypes from 'prop-types';
import { makeStyles, Dialog, DialogActions, DialogTitle, Button } from '@material-ui/core';
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
const GameOver = ({ winner, reset }) => {
  const classes = useStyles();
  return (
    <Dialog open={Boolean(winner)} classes={{ paper: classes.dialog }}>
      <DialogTitle
        className={clsx({
          [classes.red]: winner === 'Red',
          [classes.blue]: winner === 'Blue',
        })}
      >{`${winner} wins!`}</DialogTitle>
      <DialogActions>
        <Button variant="contained" onClick={reset} color="primary">
          Play Again
        </Button>
      </DialogActions>
    </Dialog>
  );
};
GameOver.defaultProps = {
  winner: null,
};
GameOver.propTypes = {
  winner: PropTypes.oneOf(['Red', 'Blue', null]),
  reset: PropTypes.func.isRequired,
};

export default GameOver;
