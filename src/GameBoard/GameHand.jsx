import React from 'react';
import PropTypes from 'prop-types';
import { Box, useMediaQuery, useTheme } from '@material-ui/core';
import GameCard from './GameCard';
import KING_MOVE_CARDS from '../constants/SpecialCards';

function GameHand({
  cards,
  setCard,
  selectedCard,
  canMove,
  enabled,
  discard,
  spare,
  isPlayerTurn,
  inverted,
}) {
  const theme = useTheme();
  const showSpare = useMediaQuery(theme.breakpoints.down('sm')) && !isPlayerTurn;

  return (
    <Box display="flex" flexDirection={inverted ? 'row-reverse' : 'row'} style={{ gap: '8px' }}>
      {showSpare && (
        <GameCard
          setCard={setCard}
          name={spare.card}
          direction={spare.direction}
          selected={false}
          moves={spare.moves}
          kingMoves={KING_MOVE_CARDS.includes(spare.card) ? spare.kingMoves || [] : []} // Handle kingMoves for spare
          enabled={enabled}
          canMove={canMove}
          discard={discard}
          showPlayed
          inverted={!inverted}
        />
      )}
      {cards.map(({ card: name, moves, direction, kingMoves }) => (
        <GameCard
          setCard={setCard}
          name={name}
          direction={direction}
          selected={selectedCard?.card === name}
          key={name}
          moves={moves}
          kingMoves={KING_MOVE_CARDS.includes(name) ? kingMoves || [] : []} // Handle kingMoves for cards
          enabled={enabled}
          canMove={canMove}
          discard={discard}
          inverted={inverted}
        />
      ))}
    </Box>
  );
}

const CardPropType = PropTypes.shape({
  card: PropTypes.string.isRequired,
  moves: PropTypes.arrayOf(
    PropTypes.shape({
      x: PropTypes.number.isRequired,
      y: PropTypes.number.isRequired,
    }),
  ).isRequired,
  direction: PropTypes.string.isRequired,
  kingMoves: PropTypes.arrayOf( // Optional kingMoves property
    PropTypes.shape({
      x: PropTypes.number.isRequired,
      y: PropTypes.number.isRequired,
    }),
  ),
});

GameHand.defaultProps = {
  selectedCard: null,
  inverted: false,
};

GameHand.propTypes = {
  setCard: PropTypes.func.isRequired,
  selectedCard: PropTypes.shape({
    card: PropTypes.string.isRequired,
  }),
  canMove: PropTypes.bool.isRequired,
  discard: PropTypes.func.isRequired,
  cards: PropTypes.arrayOf(CardPropType).isRequired,
  spare: CardPropType.isRequired,
  enabled: PropTypes.bool.isRequired,
  isPlayerTurn: PropTypes.bool.isRequired,
  inverted: PropTypes.bool,
};

export default GameHand;
