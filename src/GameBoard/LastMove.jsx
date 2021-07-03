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
const LastMove = ({ lastMove }) => {
  const classes = useStyles();
  if (!lastMove) {
    return null;
  }
  const { src, dst } = lastMove;
  const deltaX = dst.x - src.x;
  const deltaY = dst.y - src.y;
  const angle = Math.atan2(deltaY, deltaX) * radToDeg;
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
  return (
    <svg className={classes.svg}>
      <g transform={`translate(${src.x * 64 + 32} ${src.y * 64 + 32}) rotate(${angle})`}>
        <polygon
          points={points}
          stroke="rgba(80, 255, 80, 0.15)"
          fill="transparent"
          strokeWidth="3"
        />
      </g>
    </svg>
  );
};
LastMove.defaultProps = {
  lastMove: null,
};
LastMove.propTypes = {
  lastMove: PropTypes.shape({
    dst: PointPropType.isRequired,
    src: PointPropType.isRequired,
  }),
};

export default LastMove;
