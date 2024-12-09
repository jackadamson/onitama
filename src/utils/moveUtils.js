import KING_MOVE_CARDS from '../constants/SpecialCards';

/**
 * Calculates valid moves for a given card and piece.
 * @param {Object} src - The source position ({ x, y }).
 * @param {Object} card - The active card ({ moves, kingMoves, windMoves, cardSet }).
 * @param {String} turn - The current turn ('Red' or 'Blue').
 * @param {Boolean} isKingSelected - Whether the selected piece is a King.
 * @param {Boolean} isWindSpiritSelected - Whether the selected piece is the Wind Spirit.
 * @param {Boolean} extraMovePending - Whether and extra move is pending.
 * @returns {Function} A function to validate moves for a given destination.
 */

const getMoves = (src, card, turn, isKingSelected = false, isWindSpiritSelected = false, extraMovePending = false) => {

  if (!src || !card || !card.card) {
    return () => false;
  }

  // Check if the card is a "Way of the Wind" card
  const isWayOfTheWindCard = card.cardSet === 'WayOfTheWind';
  // Determine if this is a card that has special moves for a king piece
  const isSpecialCard = KING_MOVE_CARDS.includes(card.card);

  // Get the moves for the card - extraMovePending not implemented
  let moves;

  if (isWindSpiritSelected && isWayOfTheWindCard) {
    if (extraMovePending) {
        moves = card.windMoves || [];
    } else {
        moves = []; // No valid moves if extraMovePending is false
    }
} else if (isKingSelected && isSpecialCard) {
    moves = card.kingMoves || [];
} else {
    moves = card.moves || [];
}

  // Calculate the valid move positions based on the current player's turn
  const strMoves = turn === 'Red'
    ? moves.map(({ x, y }) => `${src.x + x},${src.y + y}`)
    : moves.map(({ x, y }) => `${src.x - x},${src.y - y}`);

  return (x, y) => strMoves.includes(`${x},${y}`);
};

export default getMoves;