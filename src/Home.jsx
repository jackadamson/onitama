import React, { useEffect, useState } from 'react';
import { Box, Button, Typography } from '@material-ui/core';
import { Link } from 'react-router-dom';
import useStyles from './menuStyles';
import GithubRibbon from './GithubRibbon';
import { useAppUpdater } from './updateManager';
import getCardSetDisplayName from './utils/cardSetNames';

const getGameSettings = () => {
  const storedSettings = localStorage.getItem('game_settings');
  return storedSettings ? JSON.parse(storedSettings) : {};
};

function Home() {
  const classes = useStyles();
  useAppUpdater();

  const [gameSettings, setGameSettings] = useState(getGameSettings());

  useEffect(() => {
    const handleStorageChange = () => {
      setGameSettings(getGameSettings());
    };
    window.addEventListener('storage', handleStorageChange);

    return () => {
      window.removeEventListener('storage', handleStorageChange);
    };
  }, []);

  const wayOfTheWindEnabled = !gameSettings.disabledCardSets?.includes('WayOfTheWind');

  const disabledCardSetsDisplayNames = gameSettings.disabledCardSets
    ? gameSettings.disabledCardSets.map((setId) => getCardSetDisplayName(setId))
    : [];

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

      {/* Display Current Game Settings */}
      <Box mt={1}>
        <Typography variant="body1">
          Disabled Card Sets: {disabledCardSetsDisplayNames.join(', ') || 'None'}
        </Typography>

        {wayOfTheWindEnabled && (
          <>
            <Typography variant="body1">
              Number of Wind Cards: {gameSettings.numberOfWindCards ?? 'Not Set'}
            </Typography>
            <Typography variant="body1">
              Force Wind Spirit: {gameSettings.forceWindSpiritInclusion ? 'Yes' : 'No'}
            </Typography>
          </>
        )}
      </Box>
    </Box>
  );
}

export default Home;
