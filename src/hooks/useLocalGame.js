import { useState, useMemo } from 'react';
import { useSnackbar } from 'notistack';
import { LocalGame } from '../onitamalib';
import onEvent from '../events';

const useLocalGame = () => {
  const [state, setState] = useState(null);
  const { enqueueSnackbar } = useSnackbar();
  const handlers = useMemo(() => {
    const onError = (err) => enqueueSnackbar(err, { variant: 'error', persist: false });

    const cardSetsRaw = localStorage.getItem('card_sets');
    const cardSets = cardSetsRaw ? JSON.parse(cardSetsRaw) : [];
    const game = new LocalGame(cardSets, setState, onError, onEvent);
    return {
      playMove: (m) => game.move(m),
      reset: (m) => game.reset(m),
    };
  }, [setState, enqueueSnackbar]);
  return { state, ...handlers };
};

export default useLocalGame;
