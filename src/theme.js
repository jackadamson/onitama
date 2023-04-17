import { createTheme } from '@material-ui/core/styles';

const theme = createTheme({
  palette: {
    type: 'dark',
    primary: {
      main: '#6579d4',
      light: '#d7d7d7',
    },
    secondary: {
      main: '#42938c',
    },
    direction: {
      balanced: '#cbc1a4',
      left: '#9daee3',
      right: '#c4746b',
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
