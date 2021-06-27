import React, { useCallback, useRef } from 'react';
import PropTypes from 'prop-types';
import {
  Box,
  Button,
  Dialog,
  DialogContent,
  DialogTitle,
  makeStyles,
  TextField,
  Typography,
} from '@material-ui/core';

const useStyles = makeStyles((theme) => ({
  dialog: {
    padding: theme.spacing(8),
    display: 'flex',
    flexDirection: 'column',
    justifyContent: 'center',
    alignItems: 'center',
  },
  invite: {
    fontSize: '12px',
  },
}));
const WaitingOverlay = ({ state: { connection, roomId } }) => {
  const classes = useStyles();
  const ref = useRef();
  const isWaiting = connection === 'Waiting';
  const isConnecting = connection === 'Connecting';
  const open = isWaiting || isConnecting;
  const copyLink = useCallback(() => {
    ref.current.select();
    ref.current.setSelectionRange(0, 9999);
    document.execCommand('copy');
  }, []);
  return (
    <Dialog open={open} classes={{ paper: classes.dialog }}>
      <DialogTitle>{isWaiting ? 'Waiting for Opponent' : 'Connecting'}</DialogTitle>
      <DialogContent>
        {roomId && (
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
        )}
      </DialogContent>
    </Dialog>
  );
};

WaitingOverlay.propTypes = {
  state: PropTypes.shape({
    connection: PropTypes.string.isRequired,
    roomId: PropTypes.string,
  }).isRequired,
};

export default WaitingOverlay;
