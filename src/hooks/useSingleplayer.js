import { useState, useMemo } from 'react';
import { useSnackbar } from 'notistack';
import { SingleplayerGame } from '../onitamalib';

const useSingleplayer = () => {
  const [state, setState] = useState(null);
  const { enqueueSnackbar } = useSnackbar();
  const handlers = useMemo(() => {
    const onError = (err) => enqueueSnackbar(err, { variant: 'error', persist: true });
    const game = new SingleplayerGame(setState, onError);
    return {
      playMove: (m) => game.move(m),
      reset: (m) => game.reset(m),
    };
  }, [setState, enqueueSnackbar]);
  return { state, ...handlers };
};

export default useSingleplayer;
