import { useState, useMemo } from 'react';
import { useSnackbar } from 'notistack';
import { SinglePlayerGame } from '../onitamalib';
import logger from '../logger';
import onEvent from '../events';

const useSingleplayer = (difficulty, trainingMode) => {
  const [state, setState] = useState(null);
  const [moveRankings, setMoveRankings] = useState({});
  const { enqueueSnackbar } = useSnackbar();
  const handlers = useMemo(() => {
    const worker = new Worker(new URL('../ai.worker.js', import.meta.url));
    const trainer = trainingMode && new Worker(new URL('../trainer.worker.js', import.meta.url));
    const onError = (err) => enqueueSnackbar(err, { variant: 'error', persist: false });
    const requestAiMove = (req) => worker.postMessage(req);
    const requestMoveRanking = (req) => {
      if (trainer) {
        trainer.postMessage(req);
      }
    };
    const onSetState = (newState) => {
      setMoveRankings({});
      setState(newState);
    };
    const game = new SinglePlayerGame(
      difficulty,
      trainingMode || false,
      onSetState,
      onError,
      requestAiMove,
      requestMoveRanking,
      onEvent,
    );
    worker.onmessage = (m) => game.move(m.data, false);
    trainer.onmessage = (m) => {
      const newMoveRankings = {};
      m.data.forEach(([{ src, dst, card }, ranking]) => {
        const cardSrc = `${card},${src.x},${src.y}`;
        if (!newMoveRankings[cardSrc]) {
          newMoveRankings[cardSrc] = {};
        }
        newMoveRankings[cardSrc][`${dst.x},${dst.y}`] = ranking;
      });
      setMoveRankings(newMoveRankings);
    };
    return {
      playMove: (m) => game.move(m, true),
      reset: (m) => game.reset(m),
      undo: () => game.undo(),
    };
  }, [setState, enqueueSnackbar, difficulty, trainingMode]);
  logger.log(state);
  return { state, moveRankings, ...handlers };
};

export default useSingleplayer;
