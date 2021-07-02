import React from 'react';
import PropTypes from 'prop-types';
import { Box, makeStyles, Typography } from '@material-ui/core';
import logger from '../logger';

const useStyles = makeStyles(() => ({
  Red: {
    color: '#f44336',
  },
  Blue: {
    color: '#2196f3',
  },
}));
const GameTurn = ({ turn, player }) => {
  const classes = useStyles();
  const relativeName = player === turn ? 'Your Turn' : "Opponent's Turn";
  const absoluteName = `${turn}'s Turn`;
  const label = player ? relativeName : absoluteName;
  logger.log({ turn, player });
  return (
    <Box>
      <Typography variant="h4" className={classes[turn]}>
        {label}
      </Typography>
    </Box>
  );
};
GameTurn.defaultProps = {
  player: null,
};
GameTurn.propTypes = {
  turn: PropTypes.oneOf(['Red', 'Blue']).isRequired,
  player: PropTypes.oneOf(['Red', 'Blue', null]),
};

export default GameTurn;
