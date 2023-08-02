import React, { useMemo, useState } from 'react';
import {
  Box,
  Button,
  Card,
  CardContent,
  CardHeader,
  IconButton,
  makeStyles,
  Typography, useMediaQuery, useTheme,
} from '@material-ui/core';
import { Alert, AlertTitle } from '@material-ui/lab';
import EnabledIcon from '@material-ui/icons/Visibility';
import DisabledIcon from '@material-ui/icons/VisibilityOff';

import { Link } from 'react-router-dom';
import Marquee from 'react-fast-marquee';
import { listCardSets } from '../onitamalib';
import GameCard from '../GameBoard/GameCard';

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
  const storedDisabledCardSetIds = useMemo(() => {
    const rawCardSetIds = localStorage.getItem('disabled_card_sets');
    return rawCardSetIds ? JSON.parse(rawCardSetIds) : [];
  }, []);
  const [disabledCardSetIds, setDisabledCardSetIds] = useState(storedDisabledCardSetIds);
  const toggleCardSet = (toggledId) => {
    // as well as toggling, remove any invalid set ids and ensure they are in order
    const validSetIds = cardSets.map(({ id }) => id);
    const addingCard = !disabledCardSetIds.includes(toggledId);
    const newDisabledIds = addingCard
      ? validSetIds.filter((id) => disabledCardSetIds.includes(id) || id === toggledId)
      : validSetIds.filter((id) => disabledCardSetIds.includes(id) && id !== toggledId);
    setDisabledCardSetIds(newDisabledIds);
    if (newDisabledIds.length === validSetIds.length || newDisabledIds.length === 0) {
      localStorage.removeItem('disabled_card_sets');
    } else {
      localStorage.setItem('disabled_card_sets', JSON.stringify(newDisabledIds));
    }
  };
  const enabledCardCount = useMemo(() => {
    const enabledSets = cardSets.filter(({ id }) => !disabledCardSetIds.includes(id));
    return enabledSets.reduce((accumulator, set) => accumulator + set.cards.length, 0);
  }, [disabledCardSetIds, cardSets]);
  return (
    <Box m={2}>
      <Box display="flex" alignItems="center" justifyContent="center">
        <Box maxWidth="720px" width="100%">
          <Typography variant="h4">Settings</Typography>
          <Box my={2} />
          <Typography variant="h5">Card Sets</Typography>
          <Typography variant="body1">
            Turn sets of cards on or off (currently only works for Single Player and Local
            Multiplayer)
          </Typography>
          {enabledCardCount < 5 && (
            <Alert severity="error">
              <AlertTitle>Not Enough Cards Selected</AlertTitle>
              At least 5 cards are required for a game
            </Alert>
          )}
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
                          name={card.card}
                          setCard={() => {}}
                          direction={card.direction}
                          enabled
                          spare
                        />
                      </Box>
                    ))}
                  </Marquee>
                </CardContent>
              </Card>
            </Box>
          ))}
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
