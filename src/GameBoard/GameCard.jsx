import React from 'react';
import PropTypes from 'prop-types';
import { makeStyles, Paper, Typography } from '@material-ui/core';
import clsx from 'clsx';
import { CardPropType } from './props';
import GameMoves from './GameMoves';

const useStyles = makeStyles((theme) => ({
  card: ({ enabled, spare, inverted }) => ({
    display: 'flex',
    flexDirection: inverted ? 'column-reverse' : 'column',
    flexBasis: spare ? null : '50%',
    maxWidth: spare ? '100%' : '50%',
    height: '142px',
    alignItems: 'center',
    padding: theme.spacing(0.5, 0),
    cursor: enabled ? 'pointer' : 'default',
    color: enabled ? theme.palette.common.white : theme.palette.action.disabled,
    backgroundColor: enabled ? theme.palette.background.paper : '#1a1d21',
    borderStyle: 'solid',
    borderWidth: '1px',
    [inverted ? 'borderBottomColor' : 'borderTopColor']: theme.palette.primary.main,
  }),
  selected: {
    borderColor: theme.palette.primary.main,
  },
  spare: {
    flexBasis: null,
    width: '156px',
    height: '142px',
  },
  noMoves: {
    borderColor: theme.palette.error.main,
  },
  hasMoves: {
    borderColor: theme.palette.grey['600'],
  },
  error: {
    color: theme.palette.error.main,
  },
  played: {
    borderColor: theme.palette.secondary.dark,
  },
  label: {
    margin: '2px 0',
    fontSize: '0.7rem',
    fontWeight: 'bold',
    color: theme.palette.text.secondary,
    transform: ({ inverted }) => (inverted ? 'rotate(180deg)' : 'none'),
    transition: 'transform 0.3s ease',
  },
  movesContainer: {
    display: 'flex',
    flexDirection: 'column',
    alignItems: 'center',
    gap: '4px',
  },
}));

function GameCard({
  name,
  direction,
  setCard,
  selected,
  enabled,
  moves,
  kingMoves,
  windMoves,
  cardSet,
  spare,
  canMove,
  discard,
  inverted,
  showPlayed,
  windMovePending,
  ninjaMovePending,
  windMoveCard,
  ninjaMoveCard,
}) {
  const classes = useStyles({ enabled, spare, inverted });
  const isKingMoveCard = kingMoves && kingMoves.length > 0;
  const isWindMoveCard = cardSet === 'WayOfTheWind' && windMoves && windMoves.length > 0;

  const handler = () => {
    if (enabled && !canMove) {
      discard(name);
    } else if (enabled) {
      if (ninjaMovePending) {
        setCard(ninjaMoveCard); // Force select ninjaMoveCard
      } else if (windMovePending) {
        setCard(windMoveCard); // Force select windMoveCard
      } else {
        setCard({ card: name, moves, direction, kingMoves, windMoves, cardSet });
      }
    }
  };

  return (
    <Paper
      className={clsx({
        [classes.card]: true,
        [classes.spare]: spare,
        [classes.noMoves]: enabled && !canMove,
        [classes.hasMoves]: enabled && canMove && !selected,
        [classes.selected]: selected,
        [classes.played]: showPlayed,
      })}
      onClick={handler}
    >
      <Typography
        variant="subtitle1"
        style={{ fontSize: isKingMoveCard || isWindMoveCard ? '0.75rem' : '1rem' }}
      >
        {name}
      </Typography>
      {isKingMoveCard || isWindMoveCard ? (
        <div className={classes.movesContainer}>
          {inverted ? (
            <>
              {/* Second Grid for WindMove or KingMove cards when inverted */}
              <GameMoves
                moves={isWindMoveCard ? windMoves : kingMoves}
                direction={direction}
                inverted={inverted}
                isKingMoves={isKingMoveCard}
                isWindMoves={isWindMoveCard}
                isSecondGrid
                icon={isWindMoveCard ? 'queen' : 'king'}
              />
              <Typography className={classes.label}>{isWindMoveCard ? 'THEN' : 'OR'}</Typography>
              {/* Normal Moves Grid for WindMove or KingMove cards when inverted */}
              <GameMoves
                moves={moves}
                direction={direction}
                inverted={inverted}
                isKingMoves={isKingMoveCard}
                isWindMoves={isWindMoveCard}
                icon={isWindMoveCard ? 'queen' : 'king'}
              />
            </>
          ) : (
            <>
              {/* Normal Moves Grid for WindMove or KingMove cards */}
              <GameMoves
                moves={moves}
                direction={direction}
                inverted={inverted}
                isKingMoves={isKingMoveCard}
                isWindMoves={isWindMoveCard}
                icon={isWindMoveCard ? 'queen' : 'king'}
              />
              <Typography className={classes.label}>{isWindMoveCard ? 'THEN' : 'OR'}</Typography>
              {/* Second Grid for WindMove or KingMove cards */}
              <GameMoves
                moves={isWindMoveCard ? windMoves : kingMoves}
                direction={direction}
                inverted={inverted}
                isKingMoves={isKingMoveCard}
                isWindMoves={isWindMoveCard}
                isSecondGrid
                icon={isWindMoveCard ? 'queen' : 'king'}
              />
            </>
          )}
        </div>
      ) : (
        // Regular Cards Grid
        <GameMoves moves={moves} direction={direction} inverted={inverted} />
      )}
      {!canMove && enabled && (
        <Typography className={classes.error} variant="caption">
          Discard
        </Typography>
      )}
      {showPlayed && <Typography variant="caption">(played)</Typography>}
    </Paper>
  );
}

GameCard.defaultProps = {
  enabled: false,
  selected: false,
  spare: false,
  inverted: false,
  canMove: true,
  discard: () => {},
  showPlayed: false,
  kingMoves: [],
  windMoves: [],
  windMoveCard: null,
  ninjaMoveCard: null,
  ninjaMovePending: false,
  windMovePending: false,
};

GameCard.propTypes = {
  enabled: PropTypes.bool,
  selected: PropTypes.bool,
  name: PropTypes.string.isRequired,
  direction: PropTypes.string.isRequired,
  setCard: PropTypes.func.isRequired,
  moves: PropTypes.arrayOf(
    PropTypes.shape({
      x: PropTypes.number.isRequired,
      y: PropTypes.number.isRequired,
    }),
  ).isRequired,
  kingMoves: PropTypes.arrayOf(
    PropTypes.shape({
      x: PropTypes.number.isRequired,
      y: PropTypes.number.isRequired,
    }),
  ),
  windMoves: PropTypes.arrayOf(
    PropTypes.shape({
      x: PropTypes.number.isRequired,
      y: PropTypes.number.isRequired,
    }),
  ),
  cardSet: PropTypes.string.isRequired,
  spare: PropTypes.bool,
  canMove: PropTypes.bool,
  discard: PropTypes.func,
  inverted: PropTypes.bool,
  showPlayed: PropTypes.bool,
  windMovePending: PropTypes.bool,
  ninjaMovePending: PropTypes.bool,
  windMoveCard: CardPropType,
  ninjaMoveCard: CardPropType,
};

export default GameCard;
