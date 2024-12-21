import React from 'react';
import PropTypes from 'prop-types';
import clsx from 'clsx';
import { Box, makeStyles, Paper, Tooltip } from '@material-ui/core';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import {
  faChessPawn,
  faChessKnight,
  faChessKing,
  faChessQueen,
  faSkull,
  faStar,
} from '@fortawesome/free-solid-svg-icons';
import Color from 'color';
import { PointPropType } from './props';

const icons = {
  Empty: null,
  WindSpirit: <FontAwesomeIcon icon={faChessQueen} color="#B5B4B4" size="3x" />,
  BluePawn: <FontAwesomeIcon icon={faChessPawn} color="#2196f3" size="3x" />,
  BlueNinja: <FontAwesomeIcon icon={faChessKnight} color="#2196f3" size="3x" />,
  BlueKing: <FontAwesomeIcon icon={faChessKing} color="#2196f3" size="3x" />,
  RedPawn: <FontAwesomeIcon icon={faChessPawn} color="#f44336" size="3x" />,
  RedNinja: <FontAwesomeIcon icon={faChessKnight} color="#f44336" size="3x" />,
  RedKing: <FontAwesomeIcon icon={faChessKing} color="#f44336" size="3x" />,
};

const getWindSpiritController = (currentPlayer) => (currentPlayer === 'Blue' ? 'Blue' : 'Red');

const useStyles = makeStyles((theme) => ({
  selected: { borderColor: theme.palette.primary.main },
  valid: { borderColor: theme.palette.secondary.dark },
  selectable: {
    '&:hover': { borderColor: theme.palette.primary.main },
    cursor: 'pointer',
  },
  lastMove: {},
  redBase: {
    backgroundColor: Color(theme.palette.background.paper).mix(Color('#f44336'), 0.1).hex(),
  },
  blueBase: {
    backgroundColor: Color(theme.palette.background.paper).mix(Color('#2196f3'), 0.1).hex(),
  },
  rankMarker: {
    position: 'absolute',
    transform: 'translate(-20px, -20px)',
  },
  hiddenNinja: {
    opacity: 0.5, // Apply transparency
  },
}));

const tilePlayer = (currentPlayer) => ({
  BluePawn: 'Blue',
  BlueKing: 'Blue',
  BlueNinja: 'Blue',
  RedPawn: 'Red',
  RedNinja: 'Red',
  RedKing: 'Red',
  WindSpirit: getWindSpiritController(currentPlayer),
});

function Skull() {
  const classes = useStyles();
  return (
    <Tooltip title="likely lose">
      <Box className={classes.rankMarker}>
        <FontAwesomeIcon icon={faSkull} color="#ffffff" size="xs" />
      </Box>
    </Tooltip>
  );
}

function Star() {
  const classes = useStyles();
  return (
    <Tooltip title="guaranteed win">
      <Box className={classes.rankMarker}>
        <FontAwesomeIcon icon={faStar} color="#ffff00" size="xs" bounce />
      </Box>
    </Tooltip>
  );
}

function GameSquare({
  tile,
  revealed,
  x,
  y,
  src,
  setSrc,
  turn,
  player,
  move,
  isValid,
  lastMove,
  ranking,
}) {
  const classes = useStyles();

  const currentTilePlayer = tilePlayer(turn);
  const tileOwner = currentTilePlayer[tile];
  const isPlayerPiece = tileOwner === player;

  const shouldRenderNinja =
    tile.includes('Ninja') && (isPlayerPiece || (!isPlayerPiece && revealed));

  const selected = x === src?.x && y === src?.y;
  const selectable = !selected && Boolean(tileOwner === turn || src);
  const lastSrc = x === lastMove?.src?.x && y === lastMove?.src?.y;
  const lastDst = x === lastMove?.dst?.x && y === lastMove?.dst?.y;
  const moved = lastSrc || lastDst;

  return (
    <Paper
      variant="outlined"
      component={Box}
      width="64px"
      height="64px"
      display="flex"
      justifyContent="center"
      alignItems="center"
      className={clsx({
        [classes.selected]: selected,
        [classes.selectable]: selectable || isValid,
        [classes.valid]: isValid,
        [classes.redBase]: x === 2 && y === 4,
        [classes.blueBase]: x === 2 && y === 0,
        [classes.lastMove]: moved && !isValid,
      })}
      onClick={() => {
        if (selected) {
          setSrc(null); // Deselect the piece
        } else if (src) {
          // If src is selected, handle the movement for the selected piece
          if (tile.includes('WindSpirit') && tileOwner === turn) {
            // Allow the Wind Spirit to move onto any square (including one with the player's piece)
            move({ x, y, revealNinja: false });
          } else {
            // Default movement behavior for other pieces
            move({ x, y, revealNinja: false });
          }
        } else if (tileOwner === turn) {
          // If no piece is selected, select the current piece
          setSrc({ x, y, tile, revealed });
        }
      }}
      onContextMenu={(e) => {
        e.preventDefault();

        // Ensure src is set and represents a valid starting square
        if (src) {
          // Cross-reference src (starting square) instead of the destination (current square)
          if (src.tile.includes('Ninja') && !src.revealed) {
            move({ x, y, revealNinja: true }); // Trigger reveal move to the destination square
          }
        }
      }}
    >
      {ranking < -1000000 && <Skull />}
      {ranking > 1000000 && <Star />}
      {tile.includes('Ninja') && shouldRenderNinja && (
        <Box
          className={clsx({
            [classes.hiddenNinja]: !revealed && isPlayerPiece,
          })}
        >
          {icons[tile]}
        </Box>
      )}
      {!tile.includes('Ninja') && icons[tile]}
    </Paper>
  );
}

GameSquare.defaultProps = {
  src: null,
  turn: null,
  lastMove: null,
  player: null,
};

GameSquare.propTypes = {
  tile: PropTypes.oneOf([
    'Empty',
    'BluePawn',
    'BlueNinja',
    'BlueKing',
    'RedPawn',
    'RedNinja',
    'RedKing',
    'WindSpirit',
  ]).isRequired,
  revealed: PropTypes.bool.isRequired,
  x: PropTypes.number.isRequired,
  y: PropTypes.number.isRequired,
  src: PropTypes.shape({
    x: PropTypes.number.isRequired,
    y: PropTypes.number.isRequired,
    tile: PropTypes.oneOf([
      'Empty',
      'BluePawn',
      'BlueNinja',
      'BlueKing',
      'RedPawn',
      'RedNinja',
      'RedKing',
      'WindSpirit',
    ]).isRequired,
    revealed: PropTypes.bool.isRequired,
  }),
  setSrc: PropTypes.func.isRequired,
  turn: PropTypes.oneOf(['Red', 'Blue', null]),
  player: PropTypes.oneOf(['Red', 'Blue', null]),
  move: PropTypes.func.isRequired,
  isValid: PropTypes.bool.isRequired,
  lastMove: PropTypes.shape({
    dst: PointPropType.isRequired,
    src: PointPropType.isRequired,
  }),
  ranking: PropTypes.number.isRequired,
};

export default GameSquare;
