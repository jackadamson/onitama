import React, { useMemo, useState, useEffect } from 'react';
import {
  Box,
  Button,
  Card,
  CardContent,
  CardHeader,
  IconButton,
  makeStyles,
  Typography,
  useMediaQuery,
  useTheme,
  Select,
  MenuItem,
  FormControl,
  InputLabel,
  Switch,
  FormControlLabel,
} from '@material-ui/core';
import { Alert, AlertTitle } from '@material-ui/lab';
import EnabledIcon from '@material-ui/icons/Visibility';
import DisabledIcon from '@material-ui/icons/VisibilityOff';

import { useHistory } from 'react-router-dom';
import Marquee from 'react-fast-marquee';
import { listCardSets } from '../onitamalib';
import GameCard from '../GameBoard/GameCard';
import KING_MOVE_CARDS from '../constants/SpecialCards';
import useGameSettings, { DEFAULT_GAME_SETTINGS } from '../hooks/useGameSettings';

const useStyles = makeStyles((theme) => ({
  card: {
    backgroundColor: theme.palette.background.default,
  },
  marqueeContainer: {
    marginTop: theme.spacing(1),
  },
}));

function Settings() {
  const theme = useTheme();
  const largeScreen = useMediaQuery(theme.breakpoints.up('md'));
  const styles = useStyles();
  const history = useHistory();
  const cardSets = useMemo(() => listCardSets(), []);
  const validSetIds = useMemo(() => cardSets.map(({ id }) => id), [cardSets]);

  // Use the custom hook
  const [gameSettings, updateGameSettings] = useGameSettings();

  // Create local state copies for settings
  const [localDisabledCardSets, setLocalDisabledCardSets] = useState(gameSettings.disabledCardSets);
  const [localNumberOfWindCards, setLocalNumberOfWindCards] = useState(
    gameSettings.numberOfWindCards,
  );
  const [localForceWindSpiritInclusion, setLocalForceWindSpiritInclusion] = useState(
    gameSettings.forceWindSpiritInclusion,
  );
  const [localEnableLightAndShadow, setLocalEnableLightAndShadow] = useState(
    gameSettings.enableLightAndShadow,
  );
  const [localForceLightAndShadow, setLocalForceLightAndShadow] = useState(
    gameSettings.forceLightAndShadow,
  );
  const [localLightAndShadowMode, setLocalLightAndShadowMode] = useState(
    gameSettings.lightAndShadowMode || 'Random',
  );
  const [marqueeStates, setMarqueeStates] = useState(
    cardSets.reduce((acc, { id }) => ({ ...acc, [id]: false }), {}),
  );

  // Effect to handle when Way of the Wind cards are set to 5
  useEffect(() => {
    if (localNumberOfWindCards === 5) {
      setLocalDisabledCardSets(validSetIds.filter((id) => id !== 'WayOfTheWind'));
      setLocalForceWindSpiritInclusion(true);
    }
  }, [localNumberOfWindCards, validSetIds]);

  // Check if current settings match defaults
  const settingsAreDefault = useMemo(
    () =>
      JSON.stringify(localDisabledCardSets) ===
        JSON.stringify(DEFAULT_GAME_SETTINGS.disabledCardSets) &&
      localNumberOfWindCards === DEFAULT_GAME_SETTINGS.numberOfWindCards &&
      localForceWindSpiritInclusion === DEFAULT_GAME_SETTINGS.forceWindSpiritInclusion &&
      localEnableLightAndShadow === DEFAULT_GAME_SETTINGS.enableLightAndShadow &&
      localForceLightAndShadow === DEFAULT_GAME_SETTINGS.forceLightAndShadow &&
      localLightAndShadowMode === DEFAULT_GAME_SETTINGS.lightAndShadowMode,
    [
      localDisabledCardSets,
      localNumberOfWindCards,
      localForceWindSpiritInclusion,
      localEnableLightAndShadow,
      localForceLightAndShadow,
      localLightAndShadowMode,
    ],
  );

  // Toggle card set in local state
  const toggleCardSet = (toggledId) => {
    const addingCard = localDisabledCardSets.includes(toggledId);
    const newDisabledIds = addingCard
      ? localDisabledCardSets.filter((id) => id !== toggledId)
      : [...localDisabledCardSets, toggledId];
    setLocalDisabledCardSets(newDisabledIds);
  };

  const resetSettings = () => {
    setLocalDisabledCardSets(DEFAULT_GAME_SETTINGS.disabledCardSets);
    setLocalNumberOfWindCards(DEFAULT_GAME_SETTINGS.numberOfWindCards);
    setLocalForceWindSpiritInclusion(DEFAULT_GAME_SETTINGS.forceWindSpiritInclusion);
    setLocalEnableLightAndShadow(DEFAULT_GAME_SETTINGS.enableLightAndShadow);
    setLocalForceLightAndShadow(DEFAULT_GAME_SETTINGS.forceLightAndShadow);
    setLocalLightAndShadowMode(DEFAULT_GAME_SETTINGS.lightAndShadowMode);
  };

  const handleBackToMenu = () => {
    // Save to global settings when navigating back
    updateGameSettings({
      disabledCardSets: localDisabledCardSets,
      numberOfWindCards: localNumberOfWindCards,
      forceWindSpiritInclusion: localForceWindSpiritInclusion,
      enableLightAndShadow: localEnableLightAndShadow,
      forceLightAndShadow: localForceLightAndShadow,
      lightAndShadowMode: localLightAndShadowMode,
    });

    // Delay navigation slightly to ensure state is updated
    setTimeout(() => {
      history.push('/');
    }, 200);
  };

  const enabledCardSetIds = useMemo(
    () => validSetIds.filter((id) => !localDisabledCardSets.includes(id)),
    [validSetIds, localDisabledCardSets],
  );

  const wayOfTheWindEnabled = enabledCardSetIds.includes('WayOfTheWind');
  const otherEnabledCardSets = cardSets.filter(
    ({ id }) => !localDisabledCardSets.includes(id) && id !== 'WayOfTheWind',
  );

  const totalEnabledCardCount = enabledCardSetIds.reduce((accumulator, id) => {
    const cardSet = cardSets.find((setItem) => setItem.id === id);
    return accumulator + (cardSet ? cardSet.cards.length : 0);
  }, 0);

  let errorMessage = null;
  if (totalEnabledCardCount < 5) {
    errorMessage = (
      <Alert severity="error">
        <AlertTitle>Not Enough Cards Selected</AlertTitle>
        At least 5 cards are required for a game
      </Alert>
    );
  } else if (
    wayOfTheWindEnabled &&
    otherEnabledCardSets.length === 0 &&
    localNumberOfWindCards !== 5
  ) {
    errorMessage = (
      <Alert severity="error">
        <AlertTitle>Not Enough Cards Selected</AlertTitle>
        At least one other set is required to play Way of the Wind with less than 5 cards
      </Alert>
    );
  }

  const toggleMarquee = (id) => {
    setMarqueeStates((prevState) => ({ ...prevState, [id]: !prevState[id] }));
  };

  const getLightAndShadowText = () => {
    if (localForceLightAndShadow) {
      if (localLightAndShadowMode === 'Light') {
        return 'All games will be Way of the Light games.';
      }
      if (localLightAndShadowMode === 'Shadow') {
        return 'All games will be Way of the Shadow games.';
      }
      return 'All games will be either Light or Shadow games.';
    }
    return 'Games will be Light or Shadow 5% of the time.';
  };

  return (
    <Box m={2}>
      <Box display="flex" alignItems="center" justifyContent="center">
        <Box maxWidth="720px" width="100%">
          <Typography variant="h4">Settings</Typography>
          <Box my={2} />
          <Typography variant="h5">Card Sets</Typography>
          <Typography variant="body1">
            Turn sets of cards on or off (Single Player and Local Multiplayer)
          </Typography>
          {errorMessage}
          {cardSets
            .filter(({ id }) => id !== 'WayOfTheWind')
            .map(({ id, name, cards }) => (
              <Box my={1} key={id}>
                <Card variant="outlined" className={styles.card}>
                  <CardHeader
                    title={
                      <Box onClick={() => toggleMarquee(id)} style={{ cursor: 'pointer' }}>
                        {name}
                        {!marqueeStates[id] && (
                          <Typography variant="subtitle2" color="textSecondary">
                            Click to see cards.
                          </Typography>
                        )}
                      </Box>
                    }
                    action={
                      <IconButton
                        aria-label={
                          localDisabledCardSets.includes(id) ? 'Enable set' : 'Disable set'
                        }
                        onClick={() => toggleCardSet(id)}
                        disabled={localNumberOfWindCards === 5 && wayOfTheWindEnabled}
                      >
                        {localDisabledCardSets.includes(id) ? <DisabledIcon /> : <EnabledIcon />}
                      </IconButton>
                    }
                  />
                  {marqueeStates[id] && (
                    <CardContent className={styles.marqueeContainer}>
                      <Box style={{ maxHeight: '150px', overflowY: 'auto', cursor: 'pointer' }}>
                        <Marquee speed={25} play={cards.length > 4 || !largeScreen} pauseOnClick>
                          {cards.map((card) => (
                            <Box mx={1} key={card.card}>
                              <GameCard
                                moves={card.moves}
                                kingMoves={
                                  KING_MOVE_CARDS.includes(card.card) ? card.king_moves || [] : []
                                }
                                windMoves={[]}
                                name={card.card}
                                setCard={() => {}}
                                direction={card.direction}
                                enabled
                                spare
                                cardSet=""
                                isKingMoves={KING_MOVE_CARDS.includes(card.card)}
                                isWindMoves={false}
                              />
                            </Box>
                          ))}
                        </Marquee>
                      </Box>
                    </CardContent>
                  )}
                </Card>
              </Box>
            ))}

          <Typography variant="h5">Expansions</Typography>
          <Typography variant="body1">
            Turn expansions on or off (Single Player and Local Multiplayer)
          </Typography>

          {/* Way of the Wind settings */}
          <Box my={2}>
            <Card variant="outlined" className={styles.card}>
              <CardHeader
                title={
                  <Box onClick={() => toggleMarquee('WayOfTheWind')} style={{ cursor: 'pointer' }}>
                    Way of the Wind
                    {!marqueeStates.WayOfTheWind && (
                      <Typography variant="subtitle2" color="textSecondary">
                        Click to see cards.
                      </Typography>
                    )}
                  </Box>
                }
                action={
                  <IconButton
                    aria-label={
                      localDisabledCardSets.includes('WayOfTheWind') ? 'Enable set' : 'Disable set'
                    }
                    onClick={() => toggleCardSet('WayOfTheWind')}
                  >
                    {localDisabledCardSets.includes('WayOfTheWind') ? (
                      <DisabledIcon />
                    ) : (
                      <EnabledIcon />
                    )}
                  </IconButton>
                }
              />
              {marqueeStates.WayOfTheWind && (
                <CardContent className={styles.marqueeContainer}>
                  <Box style={{ maxHeight: '150px', overflowY: 'auto', cursor: 'pointer' }}>
                    <Marquee speed={25} play pauseOnClick>
                      {cardSets
                        .find((set) => set.id === 'WayOfTheWind')
                        .cards.map((card) => (
                          <Box mx={1} key={card.card}>
                            <GameCard
                              moves={card.moves}
                              kingMoves={
                                KING_MOVE_CARDS.includes(card.card) ? card.king_moves || [] : []
                              }
                              windMoves={card.wind_moves || []}
                              name={card.card}
                              setCard={() => {}}
                              direction={card.direction}
                              enabled
                              spare
                              cardSet="WayOfTheWind"
                              isKingMoves={KING_MOVE_CARDS.includes(card.card)}
                              isWindMoves={!!card.wind_moves?.length}
                            />
                          </Box>
                        ))}
                    </Marquee>
                  </Box>
                </CardContent>
              )}
              {wayOfTheWindEnabled && (
                <CardContent>
                  <FormControl variant="outlined" fullWidth>
                    <InputLabel id="number-of-wow-cards-label">
                      Number of Way of the Wind Cards
                    </InputLabel>
                    <Select
                      labelId="number-of-wow-cards-label"
                      value={localNumberOfWindCards === null ? 'Random' : localNumberOfWindCards}
                      onChange={(event) =>
                        setLocalNumberOfWindCards(
                          event.target.value === 'Random' ? null : event.target.value,
                        )
                      }
                      label="Number of Way of the Wind Cards"
                    >
                      <MenuItem value="Random">Random</MenuItem>
                      {[0, 1, 2, 3, 4, 5].map((num) => (
                        <MenuItem key={num} value={num}>
                          {num}
                        </MenuItem>
                      ))}
                    </Select>
                  </FormControl>
                  <Typography variant="subtitle2" color="textSecondary" className={styles.subtitle}>
                    {localNumberOfWindCards === 5 || localForceWindSpiritInclusion
                      ? 'The Wind Spirit will appear in all games!'
                      : 'The Wind Spirit will appear in 25% of games.'}
                  </Typography>
                  {localNumberOfWindCards !== 5 && (
                    <Box mt={2}>
                      <FormControlLabel
                        control={
                          <Switch
                            checked={localForceWindSpiritInclusion}
                            onChange={() =>
                              setLocalForceWindSpiritInclusion(!localForceWindSpiritInclusion)
                            }
                            color="primary"
                          />
                        }
                        label="Force Wind Spirit Inclusion"
                      />
                    </Box>
                  )}
                </CardContent>
              )}
            </Card>
          </Box>

          {/* Light and Shadow settings */}
          <Box my={2}>
            <Card variant="outlined" className={styles.card}>
              <CardHeader
                title={
                  <Box onClick={() => setLocalEnableLightAndShadow(!localEnableLightAndShadow)}>
                    Light and Shadow
                  </Box>
                }
                action={
                  <IconButton
                    aria-label={
                      localEnableLightAndShadow
                        ? 'Disable Light and Shadow'
                        : 'Enable Light and Shadow'
                    }
                    onClick={() => setLocalEnableLightAndShadow(!localEnableLightAndShadow)}
                  >
                    {localEnableLightAndShadow ? <EnabledIcon /> : <DisabledIcon />}
                  </IconButton>
                }
              />
              {localEnableLightAndShadow && (
                <CardContent>
                  <Typography variant="subtitle2" color="textSecondary" className={styles.subtitle}>
                    {getLightAndShadowText()}
                  </Typography>
                  <Box mt={2}>
                    <FormControlLabel
                      control={
                        <Switch
                          checked={localForceLightAndShadow}
                          onChange={() => setLocalForceLightAndShadow(!localForceLightAndShadow)}
                          color="primary"
                        />
                      }
                      label="Force Light and Shadow"
                    />
                  </Box>
                  {localForceLightAndShadow && (
                    <Box mt={2}>
                      <FormControl variant="outlined" fullWidth>
                        <InputLabel id="light-and-shadow-mode-label">
                          Light and Shadow Mode
                        </InputLabel>
                        <Select
                          labelId="light-and-shadow-mode-label"
                          value={localLightAndShadowMode || 'Random'}
                          onChange={(event) =>
                            setLocalLightAndShadowMode(
                              event.target.value === 'Random' ? null : event.target.value,
                            )
                          }
                          label="Light and Shadow Mode"
                        >
                          <MenuItem value="Random">Random</MenuItem>
                          <MenuItem value="Shadow">Shadow</MenuItem>
                          <MenuItem value="Light">Light</MenuItem>
                        </Select>
                      </FormControl>
                    </Box>
                  )}
                </CardContent>
              )}
            </Card>
          </Box>

          {/* Save and Reset Buttons */}
          <Box mt={3} display="flex" justifyContent="space-between">
            <Button
              variant="contained"
              color="secondary"
              onClick={handleBackToMenu}
              disabled={!!errorMessage} // Disable if there's an active error message
            >
              Back to Menu
            </Button>
            {!settingsAreDefault && (
              <Button variant="contained" color="primary" onClick={resetSettings}>
                Reset to Defaults
              </Button>
            )}
          </Box>
        </Box>
      </Box>
    </Box>
  );
}

export default Settings;
