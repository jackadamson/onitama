import React, { useEffect, useState } from 'react';
import { Box, Button, Typography, IconButton, Paper } from '@material-ui/core';
import InfoIcon from '@material-ui/icons/Info';
import CloseIcon from '@material-ui/icons/Close';
import { Link } from 'react-router-dom';
import useStyles from './menuStyles';
import GithubRibbon from './GithubRibbon';
import { useAppUpdater } from './updateManager';
import useGameSettings from './hooks/useGameSettings';
import getCardSetDisplayName from './utils/cardSetNames';

function Home() {
  const classes = useStyles();
  useAppUpdater();

  const [gameSettings, setGameSettings] = useGameSettings();
  const [showSettings, setShowSettings] = useState(false); // State to toggle settings visibility

  useEffect(() => {
    const handleStorageChange = () => {
      setGameSettings(JSON.parse(localStorage.getItem('game_settings') || '{}'));
    };
    window.addEventListener('storage', handleStorageChange);

    return () => window.removeEventListener('storage', handleStorageChange);
  }, [setGameSettings]);

  const wayOfTheWindEnabled = !gameSettings.disabledCardSets?.includes('WayOfTheWind');

  const disabledCardSetsDisplayNames = gameSettings.disabledCardSets
    ? gameSettings.disabledCardSets.map((setId) => getCardSetDisplayName(setId))
    : [];

  const lightAndShadowEnabled = gameSettings.enableLightAndShadow;
  const lightAndShadowForced = gameSettings.forceLightAndShadow;
  const lightAndShadowMode = gameSettings.lightAndShadowMode;

  const getLightAndShadowText = () => {
    if (lightAndShadowEnabled) {
      if (!lightAndShadowForced) {
        return 'Light and Shadow Expansion Enabled';
      }
      if (lightAndShadowMode === 'Random') {
        return 'All games will be Light or Shadow games';
      }
      if (lightAndShadowMode === 'Light') {
        return 'All games will be Way of the Light games';
      }
      if (lightAndShadowMode === 'Shadow') {
        return 'All games will be Way of the Shadow games';
      }
    }
    return null;
  };

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
        flexDirection="row"
        alignItems="center"
        justifyContent="center"
        style={{
          width: 'calc(100% - 32px)', // Matches the width of other buttons
          maxWidth: '320px',
          marginTop: 16,
        }}
      >
        <Box
          display="flex"
          flexDirection="row"
          alignItems="center"
          style={{
            width: '100%',
          }}
        >
          <Button
            component={Link}
            to="/settings"
            variant="contained"
            color="secondary"
            style={{
              flexGrow: 1, // Button takes all available space
            }}
          >
            Settings
          </Button>
          <IconButton
            onClick={() => setShowSettings(!showSettings)}
            size="small"
            style={{
              marginLeft: 8, // Space between button and icon
              flexShrink: 0, // Prevent icon from shrinking
              width: 40, // Fixed icon width
              height: 40, // Fixed icon height
            }}
          >
            {showSettings ? <CloseIcon /> : <InfoIcon />}
          </IconButton>
        </Box>
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
          {lightAndShadowEnabled && (
            <Typography variant="body1">{getLightAndShadowText()}</Typography>
          )}
        </Paper>
      )}
    </Box>
  );
}

export default Home;
