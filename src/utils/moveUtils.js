import KING_MOVE_CARDS from '../constants/SpecialCards';

/**
 * Calculates valid moves for a given card and piece.
 * @param {Object} src - The source position ({ x, y }).
 * @param {Object} card - The active card ({ moves, kingMoves }).
 * @param {String} turn - The current turn ('Red' or 'Blue').
 * @param {Boolean} isKingSelected - Whether the selected piece is a King.
 * @param {Boolean} isWindSpiritSelected - Whether the selected piece is the Wind Spirit.
 * @returns {Function} A function to validate moves for a given destination.
 */

const getMoves = (src, card, turn, isKingSelected = false, isWindSpiritSelected = false) => {
  console.log("getMoves called with:", { src, card, turn, isKingSelected, isWindSpiritSelected });

  if (!src || !card || !card.card) {
    return () => false;
  }

  // Check if the card is a "Way of the Wind" card
  const isWayOfTheWindCard = card.cardSet === 'WayOfTheWind';
    // If this is the Wind Spirit and it's a "Way of the Wind" card, return a function that does nothing
  if (isWindSpiritSelected && isWayOfTheWindCard) {
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