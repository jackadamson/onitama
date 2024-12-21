import { makeStyles } from '@material-ui/core';

const useStyles = makeStyles((theme) => ({
  outer: {
    width: '100vw',
    height: '100vh',
    display: 'flex',
    alignItems: 'center',
    flexDirection: 'column',
    paddingTop: theme.spacing(4),
    gap: theme.spacing(2),
  },
  title: {
    width: '100%',
    maxWidth: '300px',
    margin: '0 auto',
    textAlign: 'center',
    fontSize: '2.5rem',
    fontWeight: 'bold',
    whiteSpace: 'normal',
    wordWrap: 'break-word',
    lineHeight: 1.2,
    overflow: 'visible',
    paddingTop: '50px',
    [theme.breakpoints.down('sm')]: {
      fontSize: '2rem',
      paddingTop: '70px',
    },
  },
  button: {
    width: 'calc(100% - 32px)',
    maxWidth: '320px',
  },
  centeredHeading: {
    textAlign: 'center',
  },
}));

export default useStyles;
