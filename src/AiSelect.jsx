import React from 'react';
import { Box, Button, Typography } from '@material-ui/core';
import { Link } from 'react-router-dom';
import useStyles from './menuStyles';
import GithubRibbon from './GithubRibbon';

const AiSelect = () => {
  const classes = useStyles();
  return (
    <Box className={classes.outer}>
      <Typography variant="h2">Player vs AI</Typography>
      <Box m={1} />
      <Button
        component={Link}
        to="/ai/easy"
        variant="contained"
        color="primary"
        className={classes.button}
      >
        Easy
      </Button>
      <Button
        component={Link}
        to="/ai/medium"
        variant="contained"
        color="primary"
        className={classes.button}
      >
        Medium
      </Button>
      <Button
        component={Link}
        to="/ai/hard"
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
      <GithubRibbon />
    </Box>
  );
};

export default AiSelect;
