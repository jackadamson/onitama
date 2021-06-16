import { useEffect, useState } from 'react';
import { Game } from './onitamalib';

const useOnitama = () => {
  const [{ playMove }, setPlayMove] = useState({
    playMove: () => {},
  });
  const [iteration, setIteration] = useState(0);
  const [state, setState] = useState(null);
  useEffect(() => {
    const game = new Game();
    setState(game.getState());
    const newPlayMove = (move) => {
      const result = game.move(move);
      switch (result.status) {
        case 'Playing':
          setState(result);
          break;
        case 'Error':
          return result.message;
        case 'Finished':
          setState((current) => ({ ...current, finished: true, winner: result.winner }));
          break;
        default:
          console.log(`Unhandled Status: ${result.status}`);
          break;
      }
      return null;
    };
    setPlayMove({ playMove: newPlayMove });
  }, [iteration]);
  const reset = () => setIteration((idx) => idx + 1);
  return { state, playMove, reset };
};

export default useOnitama;
