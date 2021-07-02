import React, { useCallback, useRef } from 'react';
import PropTypes from 'prop-types';
import { Link } from 'react-router-dom';
import {
  Box,
  Button,
  Dialog,
  DialogActions,
  DialogContent,
  DialogTitle,
  makeStyles,
  TextField,
  Typography,
} from '@material-ui/core';

const useStyles = makeStyles((theme) => ({
  dialog: {
    padding: theme.spacing(8, 8, 8, 8),
    display: 'flex',
    flexDirection: 'column',
    justifyContent: 'center',
    alignItems: 'center',
  },
  content: {
    display: 'flex',
    flexDirection: 'column',
    alignItems: 'center',
  },
  invite: {
    fontSize: '12px',
  },
}));
const titleFromStatus = {
  Waiting: 'Waiting for Opponent',
  OpponentDisconnected: 'Opponent disconnected, waiting for them to re-connect',
  Disconnected: 'Disconnected from server',
  Errored: 'Game Error',
};
const WaitingOverlay = ({ state: { connection, roomId, error }, reconnect }) => {
  const classes = useStyles();
  const ref = useRef();
  const title = titleFromStatus[connection] || connection;
  const isWaiting = connection === 'Waiting';
  const isConnecting = connection === 'Connecting';
  const isOpponentDisconnected = connection === 'OpponentDisconnected';
  const isDisconnected = connection === 'Disconnected';
  const isErrored = connection === 'Errored';
  const open = isWaiting || isConnecting || isDisconnected || isOpponentDisconnected || isErrored;
  const showReconnect = isDisconnected || isErrored;
  const copyLink = useCallback(() => {
    ref.current.select();
    ref.current.setSelectionRange(0, 9999);
    document.execCommand('copy');
  }, []);
  return (
    <Dialog open={open} classes={{ paper: classes.dialog }}>
      <DialogTitle>{title}</DialogTitle>
      <DialogContent className={classes.content}>
        {!showReconnect && roomId && (
          <>
            <Box minWidth="280px">
              <Typography variant="subtitle1">Invite Link</Typography>
              <Box display="flex" flexDirection="row">
                <TextField
                  inputRef={ref}
                  className={classes.invite}
                  value={`${window.location.origin}/#/r/${roomId}`}
                  spellCheck={false}
                  variant="outlined"
                />
                <Box m={0.5} />
                <Button variant="contained" color="primary" size="small" onClick={copyLink}>
                  Copy
                </Button>
              </Box>
            </Box>
            <Box pt={2}>
              <Typography variant="h5">Or</Typography>
            </Box>
          </>
        )}
        {error && <Typography variant="body1">{error}</Typography>}
      </DialogContent>
      <DialogActions>
        <Button variant="contained" color="secondary" component={Link} to="/">
          Go Home
        </Button>
        {showReconnect && (
          <Button variant="contained" color="primary" onClick={reconnect}>
            Reconnect
          </Button>
        )}
      </DialogActions>
    </Dialog>
  );
};
WaitingOverlay.propTypes = {
  state: PropTypes.shape({
    connection: PropTypes.string.isRequired,
    roomId: PropTypes.string,
    error: PropTypes.string,
  }).isRequired,
  reconnect: PropTypes.func.isRequired,
};

export default WaitingOverlay;
