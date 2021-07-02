import { createMuiTheme } from '@material-ui/core/styles';

const theme = createMuiTheme({
  palette: {
    type: 'dark',
    primary: {
      main: '#6579d4',
    },
    secondary: {
      main: '#42938c',
    },
    background: {
      paper: '#282c34',
      default: '#121115',
    },
  },
  props: {
    MuiAppBar: {
      color: 'default',
    },
  },
});

export default theme;
