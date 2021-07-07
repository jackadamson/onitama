import { useState, useMemo } from 'react';
import { useSnackbar } from 'notistack';
import { SinglePlayerGame } from '../onitamalib';
import logger from '../logger';

const useSingleplayer = (difficulty) => {
  const [state, setState] = useState(null);
  const { enqueueSnackbar } = useSnackbar();
  const handlers = useMemo(() => {
    const onError = (err) => enqueueSnackbar(err, { variant: 'error', persist: false });
    const game = new SinglePlayerGame(difficulty, setState, onError);
    return {
      playMove: (m) => game.move(m),
      reset: (m) => game.reset(m),
    };
  }, [setState, enqueueSnackbar, difficulty]);
  logger.log(state);
  return { state, ...handlers };
};

export default useSingleplayer;
