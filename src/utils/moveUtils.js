import KING_MOVE_CARDS from '../constants/SpecialCards';

/**
 * Calculates valid moves for a given card and piece.
 * @param {Object} src - The source position ({ x, y }).
 * @param {Object} card - The active card ({ moves, kingMoves }).
 * @param {String} turn - The current turn ('Red' or 'Blue').
 * @param {Boolean} isKingSelected - Whether the selected piece is a King.
 * @returns {Function} A function to validate moves for a given destination.
 */

const getMoves = (src, card, turn, isKingSelected = false) => {
  if (!src || !card || !card.card) {
    console.error('Invalid src or card in getMoves:', { src, card });
    return () => false;
  }

  // Determine if this is a card that has special moves for a king piece
  const isSpecialCard = KING_MOVE_CARDS.includes(card.card);

  // Get the moves for the card - ignore windMoves for now
  const moves = isKingSelected && isSpecialCard
    ? card.kingMoves || []
    : card.moves || [];

  // Calculate the valid move positions based on the current player's turn
  const strMoves = turn === 'Red'
    ? moves.map(({ x, y }) => `${src.x + x},${src.y + y}`)
    : moves.map(({ x, y }) => `${src.x - x},${src.y - y}`);

  return (x, y) => strMoves.includes(`${x},${y}`);
};

export default getMoves;
