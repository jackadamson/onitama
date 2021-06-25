import { useState, useEffect } from 'react';
import { useSnackbar } from 'notistack';
import { WEBSOCKET_BASE } from '../config';
import { MultiplayerGame } from '../onitamalib';

const useMultiplayer = (roomId) => {
  const [state, setState] = useState(null);
  const [room, setRoom] = useState(null);
  const [handlers, setHandlers] = useState({});
  const { enqueueSnackbar } = useSnackbar();
  useEffect(() => {
    const onError = (err) => enqueueSnackbar(err, { variant: 'error', persist: true });
    const roomUrl = `${WEBSOCKET_BASE}${roomId || ''}`;
    const sock = new WebSocket(roomUrl);
    sock.binaryType = 'arraybuffer';
    const onSend = (data) => {
      sock.send(data);
    };
    // Host goes first
    const isHost = !roomId;
    const isRed = isHost || roomId === 'ai';
    const game = new MultiplayerGame(isRed, isHost, setState, onError, onSend);
    const onMessage = (e) => {
      if (typeof e.data === 'string') {
        setRoom(e.data);
        game.connected();
      } else {
        game.handleMsg(e);
      }
    };
    setHandlers({
      playMove: (m) => game.move(m),
      reset: () => game.reset(),
    });
    sock.addEventListener('message', onMessage);
    return () => {
      sock.removeEventListener('message', onMessage);
    };
  }, [enqueueSnackbar, roomId]);
  return { state, room: roomId || room, ...handlers };
};

export default useMultiplayer;
