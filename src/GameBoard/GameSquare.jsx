import React from 'react';
import PropTypes from 'prop-types';
import clsx from 'clsx';
import { Box, makeStyles, Paper } from '@material-ui/core';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import { faChessPawn, faChessKing } from '@fortawesome/free-solid-svg-icons';
import Color from 'color';
import { PointPropType } from './props';

const icons = {
  Empty: null,
  BluePawn: <FontAwesomeIcon icon={faChessPawn} color="#2196f3" size="3x" />,
  BlueKing: <FontAwesomeIcon icon={faChessKing} color="#2196f3" size="3x" />,
  RedPawn: <FontAwesomeIcon icon={faChessPawn} color="#f44336" size="3x" />,
  RedKing: <FontAwesomeIcon icon={faChessKing} color="#f44336" size="3x" />,
};
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
    // Styling for the squares the were in the last move
    // borderColor: `${theme.palette.success.dark}a0`,
  },
  redBase: {
    backgroundColor: Color(theme.palette.background.paper).mix(Color('#f44336'), 0.1).hex(),
  },
  blueBase: {
    backgroundColor: Color(theme.palette.background.paper).mix(Color('#2196f3'), 0.1).hex(),
  },
}));
const tilePlayer = {
  BluePawn: 'Blue',
  BlueKing: 'Blue',
  RedPawn: 'Red',
  RedKing: 'Red',
};
const GameSquare = ({ tile, x, y, src, setSrc, turn, move, isValid, lastMove }) => {
  const classes = useStyles();
  const player = tilePlayer[tile];
  const activePlayer = turn === player;
  const selected = x === src?.x && y === src?.y;
  const selectable = !selected && Boolean(activePlayer || src);
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
        if (activePlayer) {
          setSrc({ x, y });
        } else {
          move({ x, y });
        }
      }}
    >
      {icons[tile]}
    </Paper>
  );
};
GameSquare.defaultProps = {
  src: null,
  turn: null,
  lastMove: null,
};
GameSquare.propTypes = {
  tile: PropTypes.oneOf(['Empty', 'BluePawn', 'BlueKing', 'RedPawn', 'RedKing']).isRequired,
  x: PropTypes.number.isRequired,
  y: PropTypes.number.isRequired,
  src: PropTypes.shape({
    x: PropTypes.number.isRequired,
    y: PropTypes.number.isRequired,
  }),
  setSrc: PropTypes.func.isRequired,
  turn: PropTypes.oneOf(['Red', 'Blue', null]),
  move: PropTypes.func.isRequired,
  isValid: PropTypes.bool.isRequired,
  lastMove: PropTypes.shape({
    dst: PointPropType.isRequired,
    src: PointPropType.isRequired,
  }),
};

export default GameSquare;
