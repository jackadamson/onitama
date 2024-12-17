import React from 'react';
import PropTypes from 'prop-types';
import { makeStyles } from '@material-ui/core';
import { PointPropType } from './props';

const useStyles = makeStyles(() => ({
  svg: {
    position: 'absolute',
    width: '320px',
    height: '320px',
    pointerEvents: 'none',
  },
}));
const radToDeg = 180 / Math.PI;
function LastMove({ lastMove, redOriented, grid, player }) {
  const classes = useStyles();
  if (!lastMove) {
    return null;
  }

  const { src, dst } = lastMove;

  const isHiddenNinja = (currentPlayer) => {
    const tile = grid[dst.y]?.[dst.x];
    if (!tile) return false;

    if (typeof tile === 'string') {
      return tile.includes('Ninja') && !tile.includes(currentPlayer);
    }

    const tileType = Object.keys(tile)[0];
    const revealed = tile[tileType]?.revealed ?? true;
    const owner = tile[tileType]?.owner || ''; // Assume owner exists in tile
    return tileType.includes('Ninja') && !revealed && owner !== currentPlayer;
  };

  // Suppress rendering if hidden Ninja is present
  if (isHiddenNinja(player)) {
    return null;
  }

  const deltaX = dst.x - src.x;
  const deltaY = dst.y - src.y;
  const angle = Math.atan2(deltaY, deltaX) * radToDeg;
  const angleAdjusted = redOriented ? angle : angle + 180;
  const scale = Math.sqrt(deltaX ** 2 + deltaY ** 2);
  const length = 64 * scale;
  const points = [
    [0, 0],
    [length - 24, -3],
    [length - 24, -12],
    [length - 12, 0],
    [length - 24, 12],
    [length - 24, 3],
  ]
    .flatMap((x) => x.toString())
    .join(' ');
  const x = redOriented ? src.x : 4 - src.x;
  const y = redOriented ? src.y : 4 - src.y;
  return (
    <svg className={classes.svg}>
      <g transform={`translate(${x * 64 + 32} ${y * 64 + 32}) rotate(${angleAdjusted})`}>
        <polygon
          points={points}
          stroke="rgba(80, 255, 80, 0.15)"
          fill="transparent"
          strokeWidth="3"
        />
      </g>
    </svg>
  );
}
LastMove.defaultProps = {
  lastMove: null,
};
LastMove.propTypes = {
  redOriented: PropTypes.bool.isRequired,
  grid: PropTypes.arrayOf(
    PropTypes.arrayOf(PropTypes.oneOfType([PropTypes.string, PropTypes.object])),
  ).isRequired,
  lastMove: PropTypes.shape({
    dst: PointPropType.isRequired,
    src: PointPropType.isRequired,
  }),
  player: PropTypes.string.isRequired,
};

export default LastMove;
