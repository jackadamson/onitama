import { useState, useMemo } from 'react';
import { useSnackbar } from 'notistack';
import { LocalGame } from '../onitamalib';
import onEvent from '../events';
import getMeta from '../meta';

const useLocalGame = () => {
  const [state, setState] = useState(null);
  const { enqueueSnackbar } = useSnackbar();
  const handlers = useMemo(() => {
    const onError = (err) => enqueueSnackbar(err, { variant: 'error', persist: false });

    const disabledCardSetsRaw = localStorage.getItem('disabled_card_sets');
    const disabledCardSets = disabledCardSetsRaw ? JSON.parse(disabledCardSetsRaw) : [];
    const game = new LocalGame(getMeta(), disabledCardSets, setState, onError, onEvent);
    return {
      playMove: (m) => game.move(m),
      reset: (m) => game.reset(m),
    };
  }, [setState, enqueueSnackbar]);
  return { state, ...handlers };
};

export default useLocalGame;
