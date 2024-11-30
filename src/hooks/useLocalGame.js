import { useState, useMemo } from 'react';
import { useSnackbar } from 'notistack';
import { LocalGame } from '../onitamalib';
import onEvent from '../events';
import getMeta from '../meta';
import useGameSettings from './useGameSettings'; 

const useLocalGame = () => {
  const [state, setState] = useState(null);
  const { enqueueSnackbar } = useSnackbar();
  
  // Use the custom game settings hook
  const [gameSettings] = useGameSettings();

  const handlers = useMemo(() => {
    const onError = (err) => enqueueSnackbar(err, { variant: 'error', persist: false });

    // Create a new LocalGame instance
    const game = new LocalGame(getMeta(), gameSettings, setState, onError, onEvent);

    return {
      playMove: (m) => game.move(m),
      reset: (m) => game.reset(m),
    };
  }, [setState, enqueueSnackbar, gameSettings]); // Adding gameSettings as a dependency

  return { state, ...handlers };
};

export default useLocalGame;
