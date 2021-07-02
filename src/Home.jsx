import React from 'react';
import { Box, Button, makeStyles, Typography } from '@material-ui/core';
import { Link } from 'react-router-dom';

const useStyles = makeStyles((theme) => ({
  outer: {
    width: '100vw',
    height: '100vh',
    display: 'flex',
    alignItems: 'center',
    flexDirection: 'column',
    paddingTop: theme.spacing(4),
    gap: theme.spacing(2),
  },
  button: {
    width: 'calc(100% - 32px)',
    maxWidth: '320px',
  },
}));
const Home = () => {
  const classes = useStyles();
  return (
    <Box className={classes.outer}>
      <Typography variant="h2">Onitama App</Typography>
      <Box m={1} />
      <Button variant="contained" color="secondary" className={classes.button} disabled>
        WIP: How to Play
      </Button>
      <Box m={1} />
      <Button
        component={Link}
        to="/r/ai"
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
    </Box>
  );
};

export default Home;
