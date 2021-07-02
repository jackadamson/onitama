import React from 'react';
import { Box, Button, Typography } from '@material-ui/core';
import { Link } from 'react-router-dom';
import useStyles from './menuStyles';

const AiSelect = () => {
  const classes = useStyles();
  return (
    <Box className={classes.outer}>
      <Typography variant="h2">Player vs AI</Typography>
      <Box m={1} />
      <Button variant="contained" color="primary" className={classes.button} disabled>
        WIP: Easy
      </Button>
      <Button variant="contained" color="primary" className={classes.button} disabled>
        WIP: Medium
      </Button>
      <Button
        component={Link}
        to="/r/ai"
        variant="contained"
        color="primary"
        className={classes.button}
      >
        Hard
      </Button>
      <Box m={1} />
      <Button
        component={Link}
        to="/"
        variant="outlined"
        color="secondary"
        className={classes.button}
      >
        Back
      </Button>
    </Box>
  );
};

export default AiSelect;
