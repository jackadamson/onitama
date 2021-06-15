import { useEffect, useState } from 'react';

const onitamaLib = import('./onitamalib');

const useOnitama = () => {
  const [{ playMove }, setPlayMove] = useState({
    playMove: () => {},
  });
  const [state, setState] = useState(null);
  useEffect(() => {
    let mounted = true;
    onitamaLib.then(({ Game }) => {
      if (mounted) {
        const game = new Game();
        setState(game.getState());
        const newPlayMove = (move) => {
          const result = game.move(move);
          // TODO: Handle game finish
          if (result.status === 'Playing') {
            console.log({ result });
            setState(result);
          }
        };
        setPlayMove({ playMove: newPlayMove });
      }
    });
    return () => {
      mounted = false;
    };
  }, []);
  return { state, playMove };
};

export default useOnitama;
