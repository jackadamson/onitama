import { useState, useEffect, useCallback } from 'react';
import { useSnackbar } from 'notistack';
import { useHistory } from 'react-router';
import { WEBSOCKET_BASE } from '../config';
import { MultiplayerGame } from '../onitamalib';
import logger from '../logger';

const useMultiplayer = (roomId) => {
  const [state, setState] = useState(null);
  const [handlers, setHandlers] = useState({});
  const { enqueueSnackbar } = useSnackbar();
  // Toggle this to force a reconnect
  const [reconnectVal, setReconnectVal] = useState(false);
  const history = useHistory();
  const reconnect = useCallback(() => {
    const currentRoomId = state?.roomId || roomId;
    history.replace(`/r/${currentRoomId}`);
    setReconnectVal((current) => !current);
  }, [setReconnectVal, state?.roomId, history]);
  useEffect(() => {
    const onError = (err) => enqueueSnackbar(err, { variant: 'error', persist: false });
    const roomUrl = `${WEBSOCKET_BASE}${roomId || ''}`;
    const sock = new WebSocket(roomUrl);
    sock.binaryType = 'arraybuffer';
    const onSend = (data) => {
      sock.send(data);
    };
    const game = new MultiplayerGame(setState, onError, onSend);
    const onMessage = (e) => {
      if (typeof e.data === 'string') {
        logger.log('Received string', e.data);
      } else {
        game.handleMsg(e);
      }
    };
    setHandlers({
      playMove: (m) => game.move(m),
      reset: () => game.reset(),
    });
    const onClose = () => {
      logger.log('Disconnected');
      setState((current) => ({ ...current, connection: 'Disconnected' }));
    };
    sock.addEventListener('close', onClose);
    sock.addEventListener('message', onMessage);
    return () => {
      sock.close(1000);
    };
  }, [enqueueSnackbar, roomId, history, reconnectVal]);
  logger.log(state);
  return { state, reconnect, ...handlers };
};

export default useMultiplayer;
