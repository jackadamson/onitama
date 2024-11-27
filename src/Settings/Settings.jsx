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

import { Link } from 'react-router-dom';
import Marquee from 'react-fast-marquee';
import { listCardSets } from '../onitamalib';
import GameCard from '../GameBoard/GameCard';
import KING_MOVE_CARDS from '../constants/SpecialCards';

const useStyles = makeStyles((theme) => ({
  card: {
    backgroundColor: theme.palette.background.default,
  },
}));

function Settings() {
  const theme = useTheme();
  const largeScreen = useMediaQuery(theme.breakpoints.up('md'));
  const styles = useStyles();
  const cardSets = useMemo(() => listCardSets(), []);
  const validSetIds = useMemo(() => cardSets.map(({ id }) => id), [cardSets]);

  const storedDisabledCardSetIds = useMemo(() => {
    const rawCardSetIds = localStorage.getItem('disabled_card_sets');
    if (rawCardSetIds) {
      return JSON.parse(rawCardSetIds);
    }
    return ['WayOfTheWind'];
  }, []);
  const [disabledCardSetIds, setDisabledCardSetIds] = useState(storedDisabledCardSetIds);

  const toggleCardSet = (toggledId) => {
    const addingCard = disabledCardSetIds.includes(toggledId);
    const newDisabledIds = addingCard
      ? disabledCardSetIds.filter((id) => id !== toggledId)
      : [...disabledCardSetIds, toggledId];
    setDisabledCardSetIds(newDisabledIds);

    if (newDisabledIds.length === 0 || newDisabledIds.length === validSetIds.length) {
      localStorage.removeItem('disabled_card_sets');
    } else {
      localStorage.setItem('disabled_card_sets', JSON.stringify(newDisabledIds));
    }
  };

  const storedNumberOfWowCards = localStorage.getItem('number_of_wow_cards');
  let initialNumberOfWowCards;
  if (storedNumberOfWowCards === 'Random') {
    initialNumberOfWowCards = 'Random';
  } else if (storedNumberOfWowCards) {
    initialNumberOfWowCards = parseInt(storedNumberOfWowCards, 10);
  } else {
    initialNumberOfWowCards = 2;
  }
  const [numberOfWowCards, setNumberOfWowCards] = useState(initialNumberOfWowCards);

  const storedForceWindSpiritInclusion = localStorage.getItem('force_wind_spirit_inclusion');
  const initialForceWindSpiritInclusion =
    storedForceWindSpiritInclusion !== null ? storedForceWindSpiritInclusion === 'true' : false;

  const [forceWindSpiritInclusion, setForceWindSpiritInclusion] = useState(
    initialForceWindSpiritInclusion,
  );

  useEffect(() => {
    localStorage.setItem('number_of_wow_cards', numberOfWowCards);
  }, [numberOfWowCards]);

  useEffect(() => {
    localStorage.setItem('force_wind_spirit_inclusion', forceWindSpiritInclusion);
  }, [forceWindSpiritInclusion]);

  const enabledCardSetIds = useMemo(
    () => validSetIds.filter((id) => !disabledCardSetIds.includes(id)),
    [validSetIds, disabledCardSetIds],
  );

  const wayOfTheWindEnabled = enabledCardSetIds.includes('WayOfTheWind');
  const otherEnabledCardSets = cardSets.filter(
    ({ id }) => !disabledCardSetIds.includes(id) && id !== 'WayOfTheWind',
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
  } else if (wayOfTheWindEnabled && otherEnabledCardSets.length === 0 && numberOfWowCards !== 5) {
    errorMessage = (
      <Alert severity="error">
        <AlertTitle>Not Enough Cards Selected</AlertTitle>
        At least one other set is required to play Way of the Wind with less than 5 cards
      </Alert>
    );
  }

  return (
    <Box m={2}>
      <Box display="flex" alignItems="center" justifyContent="center">
        <Box maxWidth="720px" width="100%">
          <Typography variant="h4">Settings</Typography>
          <Box my={2} />
          <Typography variant="h5">Card Sets</Typography>
          <Typography variant="body1">
            Turn sets of cards on or off (currently only works for Single Player and Local Multiplayer)
          </Typography>
          {errorMessage}
          {cardSets.map(({ id, name, cards }) => (
            <Box my={1} key={id}>
              <Card variant="outlined" className={styles.card}>
                <CardHeader
                  title={name}
                  action={
                    <IconButton
                      aria-label={disabledCardSetIds.includes(id) ? 'Enable set' : 'Disable set'}
                      onClick={() => toggleCardSet(id)}
                    >
                      {disabledCardSetIds.includes(id) ? <DisabledIcon /> : <EnabledIcon />}
                    </IconButton>
                  }
                />
                <CardContent>
                  <Marquee speed={25} play={cards.length > 4 || !largeScreen}>
                    {cards.map((card) => (
                      <Box mx={1} key={card.card}>
                        <GameCard
                          moves={card.moves}
                          kingMoves={KING_MOVE_CARDS.includes(card.card) ? card.king_moves || [] : []}
                          windMoves={id === 'WayOfTheWind' ? card.wind_moves || [] : []}
                          name={card.card}
                          setCard={() => {}}
                          direction={card.direction}
                          enabled
                          spare
                          cardSet={id === 'WayOfTheWind' ? 'WayOfTheWind' : ''}
                          isKingMoves={KING_MOVE_CARDS.includes(card.card)}
                          isWindMoves={!!(id === 'WayOfTheWind' && card.wind_moves && card.wind_moves.length > 0)}
                        />
                      </Box>
                    ))}
                  </Marquee>
                </CardContent>
              </Card>
            </Box>
          ))}
          {wayOfTheWindEnabled && (
            <Box my={2}>
              <FormControl variant="outlined" fullWidth>
                <InputLabel id="number-of-wow-cards-label">
                  Number of Way of the Wind Cards
                </InputLabel>
                <Select
                  labelId="number-of-wow-cards-label"
                  value={numberOfWowCards}
                  onChange={(event) => setNumberOfWowCards(event.target.value)}
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
              <Box mt={2}>
                <FormControlLabel
                  control={
                    <Switch
                      checked={forceWindSpiritInclusion}
                      onChange={(event) => setForceWindSpiritInclusion(event.target.checked)}
                      color="primary"
                    />
                  }
                  label="Force Wind Spirit Inclusion"
                />
              </Box>
            </Box>
          )}
          <Box display="flex" mt={3}>
            <Button variant="outlined" color="secondary" component={Link} to="/">
              Back to Menu
            </Button>
          </Box>
        </Box>
      </Box>
    </Box>
  );
}

export default Settings;
