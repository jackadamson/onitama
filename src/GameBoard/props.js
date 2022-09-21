import PropTypes from 'prop-types';

export const PointPropType = PropTypes.shape({
  x: PropTypes.number.isRequired,
  y: PropTypes.number.isRequired,
});

export const CardPropType = PropTypes.shape({
  card: PropTypes.string.isRequired,
  moves: PropTypes.arrayOf(PointPropType).isRequired,
  direction: PropTypes.string.isRequired,
});
