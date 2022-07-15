import React from 'react';
import PropTypes from 'prop-types';
import { Box, Typography } from '@material-ui/core';

function Loading({ message }) {
  return (
    <Box display="flex" width="100vw" height="100vh" alignItems="center" justifyContent="center">
      <Typography variant="h4">{message}</Typography>
    </Box>
  );
}
Loading.defaultProps = {
  message: 'Loading ...',
};
Loading.propTypes = {
  message: PropTypes.string,
};
export default Loading;
