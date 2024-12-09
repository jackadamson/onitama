import React, { useEffect, useState } from 'react';
import { Box, Button, Typography, IconButton, Paper } from '@material-ui/core';
import InfoIcon from '@material-ui/icons/Info';
import CloseIcon from '@material-ui/icons/Close';
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
  const [showSettings, setShowSettings] = useState(false); // State to toggle settings visibility

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
      <Typography variant="h2" className={classes.title}>
        Onitama App
      </Typography>
      <Box m={0.5} />
      <Button
        component={Link}
        to="/rules/base-game"
        variant="contained"
        color="secondary"
        className={classes.button}
      >
        How to Play
      </Button>
      <Box m={0.5} />
      <Button
        component={Link}
        to="/ai"
        variant="contained"
        color="primary"
        className={classes.button}
      >
        Single Player
      </Button>
      <Box m={0.5} />
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

      {/* Settings Button + Info Icon */}
      <Box
        display="flex"
        flexDirection="column"
        alignItems="center"
        width="100%"
        style={{ marginTop: 16 }}
      >
        <Box
          className={classes.button}
          display="flex"
          alignItems="center"
          justifyContent="space-between"
          style={{
            width: '100%',
            maxWidth: '320px',
          }}
        >
          <Button
            component={Link}
            to="/settings"
            variant="contained"
            color="secondary"
            style={{
              flex: 1,
            }}
          >
            Settings
          </Button>
          <IconButton
            onClick={() => setShowSettings(!showSettings)}
            style={{
              marginLeft: 8,
            }}
          >
            {showSettings ? <CloseIcon /> : <InfoIcon />}
          </IconButton>
        </Box>

        {/* Conditionally Display Current Game Settings */}
        {showSettings && (
          <Paper
            elevation={3}
            style={{
              marginTop: 8,
              padding: 12,
              width: 'calc(100% - 32px)',
              maxWidth: '320px',
            }}
          >
            <Typography variant="body1">
              Disabled Card Sets: {disabledCardSetsDisplayNames.join(', ') || 'None'}
            </Typography>
            {wayOfTheWindEnabled && (
              <>
                <Typography variant="body1">
                  Number of Wind Cards: {gameSettings.numberOfWindCards ?? 'Random'}
                </Typography>
                <Typography variant="body1">
                  Force Wind Spirit: {gameSettings.forceWindSpiritInclusion ? 'Yes' : 'No'}
                </Typography>
              </>
            )}
          </Paper>
        )}
      </Box>
    </Box>
  );
}

export default Home;
