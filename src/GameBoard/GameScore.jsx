import React from 'react';
import PropTypes from 'prop-types';
import { Box } from '@material-ui/core';

function GameScore({ score, stale, playerIsRed }) {
  if (!score) {
    return null;
  }
  const red = '#f44336';
  const blue = '#2196f3';
  const outer = playerIsRed ? blue : red;
  const inner = playerIsRed ? red : blue;
  return (
    <Box maxWidth="0px" height="100%">
      <Box height="100%" width="8px" bgcolor={outer} overflow="hidden" borderRadius="4px">
        <Box
          height="100%"
          width="8px"
          bgcolor={inner}
          style={{ transform: `translateY(${(100 - score).toFixed(0)}%)` }}
        />
      </Box>
    </Box>
  );
}
GameScore.defaultProps = {
  stale: true,
  score: null,
};
GameScore.propTypes = {
  playerIsRed: PropTypes.bool.isRequired,
  stale: PropTypes.bool,
  score: PropTypes.number,
};

export default GameScore;
