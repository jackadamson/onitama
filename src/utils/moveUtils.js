import KING_MOVE_CARDS from '../constants/SpecialCards';

/**
 * Calculates valid moves for a given card and piece.
 * @param {Object} src - The source position ({ x, y }).
 * @param {Object} card - The active card ({ moves, kingMoves, windMoves, cardSet }).
 * @param {Array} grid - The game board grid to determine piece types and ownership.
 * @param {String} turn - The current turn ('Red' or 'Blue').
 * @param {Boolean} windMovePending - Whether an wind move is pending.
 * @returns {Function} A function to validate moves for a given destination.
 */

const getMoves = (src, card, grid, turn, windMovePending = false) => {
  if (!src || !card || !card.card || !grid[src.y]?.[src.x]) {
    return () => false;
  }

  // Get the selected piece from the grid
  let piece = grid[src.y][src.x];
  if (typeof piece === 'object') {
    [piece] = Object.keys(piece);
  }

  // Determine piece attributes
  const isKing = piece.includes('King');
  const isWindSpirit = piece.includes('Spirit');
  const isOpponentKing = isKing && !piece.includes(turn);

  const isSpecialCard = KING_MOVE_CARDS.includes(card.card);

  // Get the moves for the card
  let moves;

  if (isWindSpirit && card.cardSet === 'WayOfTheWind') {
    moves = windMovePending ? card.windMoves || [] : [];
  } else if (isOpponentKing && card.card === 'Kame') {
    // Hard-coded move: Opponent's King always moves 1 square towards user
    moves = [{ x: 0, y: -1 }]; // Universal move; direction flipped by `strMoves`
  } else if (isKing && isSpecialCard) {
    moves = card.kingMoves || []; // Default King moves
  } else {
    moves = card.moves || []; // Default moves for other pieces
  }

  // Calculate the valid move positions based on the current player's turn
  const strMoves = moves.map(
    ({ x, y }) =>
      turn === 'Red'
        ? `${src.x + x},${src.y + y}` // Forward for Red
        : `${src.x - x},${src.y - y}`, // Forward for Blue
  );

  return (x, y) => strMoves.includes(`${x},${y}`);
};

export default getMoves;
