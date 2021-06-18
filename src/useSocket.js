import { useEffect, useState } from 'react';
import { useSnackbar } from 'notistack';
import { WEBSOCKET_BASE } from './config';

const useSocket = (roomId, playMove, importState, exportState) => {
  // If hosting, roomId is null so will need to acquire from server
  const [room, setRoom] = useState(null);
  const [connected, setConnected] = useState(false);
  const [[playLocalMove], setPlayLocalMove] = useState([null]);
  const { enqueueSnackbar } = useSnackbar();
  useEffect(() => {
    if (!playMove) {
      console.log('Waiting for game to init');
      return () => {};
    }
    const roomUrl = `${WEBSOCKET_BASE}${roomId || ''}`;
    const sock = new WebSocket(roomUrl);
    // Guest's will have a room ID so don't need to track being connected
    let receivedRoom = false;
    let initialized = false;
    const handleMessage = (event) => {
      console.log('MSG RECV: ', event.data);
      if (!receivedRoom) {
        receivedRoom = true;
        console.log('Received room!');
        setRoom(event.data);
        // If guest, send game state
        if (roomId) {
          initialized = true;
          const state = exportState();
          console.log(state);
          const msg = { state };
          sock.send(JSON.stringify(msg));
        }
        return;
      }
      const msg = JSON.parse(event.data);
      if (!initialized) {
        initialized = true;
        const { state } = msg;
        if (state) {
          importState(state);
        }
        return;
      }
      // Message is a move
      const { move } = msg;
      const result = playMove(move);
      if (result) {
        enqueueSnackbar('Opponent Played Illegal Move', { variant: 'error', persist: true });
      }
    };
    sock.addEventListener('message', handleMessage);
    setPlayLocalMove([
      (move) => {
        const result = playMove(move);
        if (!result) {
          const msg = { move };
          sock.send(JSON.stringify(msg));
        }
        return result;
      },
    ]);
    return () => {
      sock.removeEventListener('message', handleMessage);
      sock.close();
    };
  }, [roomId, playMove, enqueueSnackbar, setRoom, setConnected, importState, exportState]);
  return { playLocalMove, room: roomId || room, connected };
};

export default useSocket;
