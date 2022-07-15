import React from 'react';
import { Box, Button, Typography } from '@material-ui/core';
import { Link } from 'react-router-dom';
import useStyles from './menuStyles';

function AiSelect() {
  const classes = useStyles();
  return (
    <Box className={classes.outer}>
      <Typography variant="h2" className={classes.centeredHeading}>
        Select an AI to train with
      </Typography>
      <Box m={1} />
      <Button
        component={Link}
        to="/t/easy"
        variant="contained"
        color="primary"
        className={classes.button}
      >
        Easy
      </Button>
      <Button
        component={Link}
        to="/t/medium"
        variant="contained"
        color="primary"
        className={classes.button}
      >
        Medium
      </Button>
      <Button
        component={Link}
        to="/t/hard"
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
}

export default AiSelect;
