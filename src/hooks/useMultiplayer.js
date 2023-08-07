import { useState, useEffect, useCallback } from 'react';
import { useSnackbar } from 'notistack';
import { useHistory } from 'react-router';
import { WEBSOCKET_BASE } from '../config';
import { MultiplayerGame } from '../onitamalib';
import logger from '../logger';
import onEvent from '../events';
import getMeta from '../meta';

const useMultiplayer = (roomId, isAi) => {
  const [state, setState] = useState(null);
  const [handlers, setHandlers] = useState({});
  const { enqueueSnackbar } = useSnackbar();
  // Toggle this to force a reconnect
  const [reconnectVal, setReconnectVal] = useState(false);
  const history = useHistory();
  const reconnect = useCallback(() => {
    setReconnectVal((current) => !current);
  }, [setReconnectVal, state?.roomId, history]);
  useEffect(() => {
    let mounted = true;
    const setStateMounted = (val) => {
      if (mounted) setState(val);
    };
    const onError = (err) => enqueueSnackbar(err, { variant: 'error', persist: false });
    const roomUrl = `${WEBSOCKET_BASE}${isAi ? 'ai/' : ''}${roomId || ''}`;
    const sock = new WebSocket(roomUrl);
    const keepAlive = setInterval(() => {
      sock.send('ping');
    }, 30000);
    sock.binaryType = 'arraybuffer';
    const onSend = (data) => {
      sock.send(data);
    };
    const game = new MultiplayerGame(getMeta(), setStateMounted, onError, onSend, onEvent);
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
      setStateMounted((current) => ({ ...current, connection: 'Disconnected' }));
    };
    sock.addEventListener('close', onClose);
    sock.addEventListener('message', onMessage);
    return () => {
      clearInterval(keepAlive);
      mounted = false;
      sock.close(1000);
    };
  }, [enqueueSnackbar, roomId, history, reconnectVal, isAi]);
  const stateRoomId = state?.roomId;
  useEffect(() => {
    if (stateRoomId && !roomId) {
      history.replace(`/r/${stateRoomId}`);
    }
  }, [history, roomId, stateRoomId]);
  logger.log(state);
  return { state, reconnect, ...handlers };
};

export default useMultiplayer;
