import { useState, useMemo } from 'react';
import { useSnackbar } from 'notistack';
import { SinglePlayerGame } from '../onitamalib';
import logger from '../logger';
import onEvent from '../events';

const useSingleplayer = (difficulty) => {
  const [state, setState] = useState(null);
  const { enqueueSnackbar } = useSnackbar();
  const handlers = useMemo(() => {
    const worker = new Worker(new URL('../ai.worker.js', import.meta.url));
    const onError = (err) => enqueueSnackbar(err, { variant: 'error', persist: false });
    const requestAiMove = (req) => worker.postMessage(req);
    const game = new SinglePlayerGame(difficulty, setState, onError, requestAiMove, onEvent);
    worker.onmessage = (m) => game.move(m.data, false);
    return {
      playMove: (m) => game.move(m, true),
      reset: (m) => game.reset(m),
      undo: () => game.undo(),
    };
  }, [setState, enqueueSnackbar, difficulty]);
  logger.log(state);
  return { state, ...handlers };
};

export default useSingleplayer;
