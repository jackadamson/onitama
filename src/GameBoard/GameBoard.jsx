import React, { useEffect, useState } from 'react';
import PropTypes from 'prop-types';
import { Link } from 'react-router-dom';
import { Box, Button, useMediaQuery, useTheme } from '@material-ui/core';
import UndoIcon from '@material-ui/icons/Undo';
import GameOver from './GameOver';
import GameCard from './GameCard';
import GameGrid from './GameGrid';
import GameHand from './GameHand';
import GameTurn from './GameTurn';
import { CardPropType, PointPropType } from './props';
import GameScore from './GameScore';
import KING_MOVE_CARDS from '../constants/SpecialCards';
import getMoves from '../utils/moveUtils';

function GameBoard({
  src,
  setSrc,
  grid,
  winner,
  player,
  turn,
  spare,
  blueCards,
  redCards,
  setCard,
  card,
  canMove,
  lastMove,
  dstMoveRankings,
  connectionStatus,
  isMoveValid,
  move,
  discard,
  reset,
  canUndo,
  undo,
  score,
  stale,
}) {
  const theme = useTheme();
  const [minimizedGameOver, setMinimizedGameOver] = useState(false);

  useEffect(() => {
    if (!winner) {
      setMinimizedGameOver(false);
    }
  }, [winner]);

  const hideSideSpare = useMediaQuery(theme.breakpoints.down('sm'));

  // Whether it's the player's turn, always true if local multiplayer
  const playerTurn = player ? player === turn : true;

  // Whether perspective should have red at bottom of screen
  const redOriented = player !== 'Blue';

  // Determine if a king is selected
  const isKingSelected = src && grid[src.y]?.[src.x]?.includes('King');

  // Updated isMoveValid logic using centralized getMoves
  const isValidMove = getMoves(src, card, turn, isKingSelected);

  return (
    <Box height="100vh" display="flex" flexDirection="column">
      <Box display="flex" justifyContent="center">
        <GameTurn player={player} turn={turn} />
      </Box>
      <Box display="flex" flexDirection={redOriented ? 'row' : 'row-reverse'}>
        <Box position="absolute" top="0" left="0">
          <Button component={Link} to="/">
            Home
          </Button>
        </Box>
        <GameOver
          reset={reset}
          winner={winner}
          player={player}
          connectionStatus={connectionStatus}
          minimizedGameOver={minimizedGameOver}
          setMinimizedGameOver={setMinimizedGameOver}
        />
        <Box
          display={hideSideSpare ? 'none' : 'flex'}
          flexDirection="column"
          justifyContent="center"
          alignItems="center"
          flexBasis="33%"
        >
          {turn === 'Blue' && (
            <GameCard
              spare
              inverted={redOriented}
              moves={spare.moves}
              kingMoves={KING_MOVE_CARDS.includes(spare.card) ? spare.kingMoves || [] : []}
              windMoves={spare.cardSet === 'WayOfTheWind' ? spare.windMoves || [] : []}
              cardSet={spare.cardSet}  
              enabled={false}
              setCard={setCard}
              name={spare.card}
              direction={spare.direction}
              selected={false}
            />
          )}
        </Box>
        <Box display="flex" justifyContent="center" alignItems="center" flexGrow={1}>
          <Box display="flex" flexDirection={redOriented ? 'column' : 'column-reverse'}>
            {/* Passing Blue Cards to GameHand */}
            <GameHand
              setCard={setCard}
              selectedCard={card}
              spare={spare}
              discard={discard}
              canMove={canMove}
              cards={blueCards}
              enabled={turn === 'Blue' && playerTurn}
              isPlayerTurn={turn === 'Blue'}
              inverted={redOriented}
            />
            <GameGrid
              isMoveValid={(x, y) => isValidMove(x, y)}
              move={move}
              src={src}
              setSrc={setSrc}
              grid={grid}
              turn={turn}
              lastMove={lastMove}
              dstMoveRankings={dstMoveRankings || {}}
              redOriented={redOriented}
            />
            {/* Passing Red Cards to GameHand */}
            <GameHand
              setCard={setCard}
              selectedCard={card}
              spare={spare}
              discard={discard}
              canMove={canMove}
              cards={redCards}
              enabled={turn === 'Red' && playerTurn}
              isPlayerTurn={turn === 'Red'}
              inverted={!redOriented}
            />
          </Box>
          <GameScore score={score} stale={stale} playerIsRed={player === 'Red'} />
        </Box>
        <Box
          display={hideSideSpare ? 'none' : 'flex'}
          flexDirection="column"
          justifyContent="center"
          alignItems="center"
          flexBasis="33%"
        >
          {turn === 'Red' && (
            <GameCard
              spare
              inverted={!redOriented}
              moves={spare.moves}
              kingMoves={KING_MOVE_CARDS.includes(spare.card) ? spare.kingMoves || [] : []}
              windMoves={spare.cardSet === 'WayOfTheWind' ? spare.windMoves || [] : []}
              cardSet={spare.cardSet}  
              enabled={false}
              setCard={setCard}
              name={spare.card}
              direction={spare.direction}
              selected={false}
            />
          )}
        </Box>
      </Box>
      {undo && (
        <Box width="100%" display="flex" justifyContent="center" py={2}>
          <Box display="flex" flexDirection="column" width="100%" maxWidth="320px">
            <Button variant="contained" disabled={!canUndo} startIcon={<UndoIcon />} onClick={undo}>
              Undo last move
            </Button>
          </Box>
        </Box>
      )}
      {minimizedGameOver && Boolean(winner) && (
        <Box
          p={1}
          display="flex"
          position="sticky"
          justifyContent="center"
          bottom="0px"
          width="100%"
        >
          <Box width="100%" maxWidth="320px" display="flex" flexDirection="column">
            <Button variant="contained" onClick={reset}>
              Rematch
            </Button>
          </Box>
        </Box>
      )}
    </Box>
  );
}

// Re-adding defaultProps for non-required props
GameBoard.defaultProps = {
  src: null,
  winner: null,
  reset: null,
  player: null,
  lastMove: null,
  dstMoveRankings: null,
  connectionStatus: null,
  card: null,
  canUndo: null,
  undo: null,
  score: null,
  stale: true,
};

GameBoard.propTypes = {
  src: PointPropType,
  setSrc: PropTypes.func.isRequired,
  grid: PropTypes.arrayOf(PropTypes.arrayOf(PropTypes.string).isRequired).isRequired,
  winner: PropTypes.oneOf(['Red', 'Blue', null]),
  reset: PropTypes.func,
  turn: PropTypes.oneOf(['Red', 'Blue']).isRequired,
  player: PropTypes.oneOf(['Red', 'Blue', null]),
  spare: CardPropType.isRequired,
  blueCards: PropTypes.arrayOf(CardPropType).isRequired,
  redCards: PropTypes.arrayOf(CardPropType).isRequired,
  lastMove: PropTypes.shape({
    dst: PointPropType.isRequired,
    src: PointPropType.isRequired,
  }),
  dstMoveRankings: PropTypes.objectOf(PropTypes.number),
  connectionStatus: PropTypes.string,
  setCard: PropTypes.func.isRequired,
  card: CardPropType,
  canMove: PropTypes.bool.isRequired,
  isMoveValid: PropTypes.func.isRequired,
  move: PropTypes.func.isRequired,
  discard: PropTypes.func.isRequired,
  canUndo: PropTypes.bool,
  undo: PropTypes.func,
  score: PropTypes.number,
  stale: PropTypes.bool,
};

export default GameBoard;
