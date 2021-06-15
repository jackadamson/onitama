import { useEffect, useState } from 'react';
import { useSnackbar } from 'notistack';

const onitamaLib = import('./onitamalib');

const useOnitama = () => {
  const [{ playMove }, setPlayMove] = useState({
    playMove: () => {},
  });
  const { enqueueSnackbar } = useSnackbar();
  const [iteration, setIteration] = useState(0);
  const [state, setState] = useState(null);
  const [inverted, setInverted] = useState(null);
  useEffect(() => {
    let mounted = true;
    onitamaLib.then(({ Game }) => {
      if (mounted) {
        const game = new Game();
        setState(game.getState());
        setInverted(game.getInvertedState());
        const newPlayMove = (move) => {
          const result = game.move(move);
          switch (result.status) {
            case 'Playing':
              setState(result);
              setInverted(game.getInvertedState());
              break;
            case 'Error':
              enqueueSnackbar(result.message, { variant: 'error' });
              break;
            case 'Finished':
              setState((current) => ({ ...current, finished: true, winner: result.winner }));
              break;
            default:
              console.log(`Unhandled Status: ${result.status}`);
              break;
          }
        };
        setPlayMove({ playMove: newPlayMove });
      }
    });
    return () => {
      mounted = false;
    };
  }, [enqueueSnackbar, iteration]);
  const reset = () => setIteration((idx) => idx + 1);
  return { state, playMove, reset, inverted };
};

export default useOnitama;
