import React from 'react';
import { Box, Button, Typography } from '@material-ui/core';
import { Link } from 'react-router-dom';
import useStyles from './menuStyles';
import GithubRibbon from './GithubRibbon';
import { useAppUpdater } from './updateManager';

function Home() {
  const classes = useStyles();
  useAppUpdater();
  return (
    <Box className={classes.outer}>
      <GithubRibbon />
      <Typography variant="h2">Onitama App</Typography>
      <Box m={1} />
      <Button
        component={Link}
        to="/help"
        variant="contained"
        color="secondary"
        className={classes.button}
      >
        How to Play
      </Button>
      <Box m={1} />
      <Button
        component={Link}
        to="/ai"
        variant="contained"
        color="primary"
        className={classes.button}
      >
        Single Player
      </Button>
      <Box m={1} />
      <Button
        component={Link}
        to="/l/"
        variant="contained"
        color="primary"
        className={classes.button}
      >
        Local Multiplayer
      </Button>
      <Button
        component={Link}
        to="/r/"
        variant="contained"
        color="primary"
        className={classes.button}
      >
        Online Multiplayer
      </Button>

      {process.env.REACT_APP_LOCAL_AI && (
        <>
          <Box m={1} />
          <Button
            component={Link}
            to="/t"
            variant="contained"
            color="primary"
            className={classes.button}
          >
            Training Mode
          </Button>
        </>
      )}
      <Box m={1} />
      <Button
        component={Link}
        to="/settings"
        variant="contained"
        color="secondary"
        className={classes.button}
      >
        Settings (NEW)
      </Button>
    </Box>
  );
}

export default Home;
