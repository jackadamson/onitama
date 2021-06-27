import { useState, useMemo } from 'react';
import { useSnackbar } from 'notistack';
import { LocalGame } from '../onitamalib';

const useLocalGame = () => {
  const [state, setState] = useState(null);
  const { enqueueSnackbar } = useSnackbar();
  const handlers = useMemo(() => {
    const onError = (err) => enqueueSnackbar(err, { variant: 'error', persist: true });
    const game = new LocalGame(setState, onError);
    return {
      playMove: (m) => game.move(m),
      reset: (m) => game.reset(m),
    };
  }, [setState, enqueueSnackbar]);
  return { state, ...handlers };
};

export default useLocalGame;
