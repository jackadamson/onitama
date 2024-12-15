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

// Determine the controller of WindSpirit based on the current player
const getWindSpiritController = (currentPlayer) => (currentPlayer === 'Blue' ? 'Blue' : 'Red');

const useStyles = makeStyles((theme) => ({
  selected: {
    borderColor: theme.palette.primary.main,
  },
  valid: {
    borderColor: theme.palette.secondary.dark,
  },
  selectable: {
    '&:hover': {
      borderColor: theme.palette.primary.main,
    },
    cursor: 'pointer',
  },
  lastMove: {
    // Styling for the squares that were in the last move
    // borderColor: `${theme.palette.success.dark}a0`,
  },
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
}));

// tilePlayer is now a function that takes the currentPlayer as an argument
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
  player, // Add player prop
  move,
  isValid,
  lastMove,
  ranking,
}) {
  const classes = useStyles();

  // Ownership based on the player viewing the board
    const tileOwner = (() => {
        if (tile.includes('Blue')) return 'Blue';
        if (tile.includes('Red')) return 'Red';
        return null;
    })();
  const isViewerPiece = tileOwner === player; // Check if the tile belongs to the viewing player

  // Render Ninja only if it's the viewer's Ninja or revealed
  const shouldRenderNinja = tile.includes('Ninja') && (isViewerPiece || revealed);

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
          setSrc(null); // Deselect if already selected
        } else if (src) {
          move({ x, y }); // Attempt to move to the clicked square
        } else if (tileOwner === turn) {
          setSrc({ x, y, type: tile.includes('King') ? 'King' : 'Pawn' }); // Select the piece
        }
      }}
    >
      {ranking < -1000000 && <Skull />}
      {ranking > 1000000 && <Star />}
      {tile.includes('Ninja') && shouldRenderNinja && icons[tile]}{' '}
      {/* Conditionally render Ninja */}
      {!tile.includes('Ninja') && icons[tile]} {/* Render other pieces normally */}
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
