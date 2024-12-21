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
  const [originalGameSettings] = useGameSettings();

  // Override enableLightAndShadow and related settings
  const gameSettings = useMemo(() => {
    if (originalGameSettings.enableLightAndShadow) {
      enqueueSnackbar('Light and Shadow expansion not currently supported in Local Multiplayer', {
        variant: 'warning',
      });
    }

    return {
      ...originalGameSettings,
      enableLightAndShadow: false,
      forceLightAndShadow: false,
      lightAndShadowMode: null,
    };
  }, [originalGameSettings, enqueueSnackbar]);

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
