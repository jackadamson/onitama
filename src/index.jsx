import React, { Suspense } from 'react';
import ReactDOM from 'react-dom';
import { CssBaseline } from '@material-ui/core';
import { ThemeProvider } from '@material-ui/core/styles';
import { SnackbarProvider } from 'notistack';
import theme from './theme';
import Loading from './Loading';

const App = React.lazy(() => import('./App'));

ReactDOM.render(
  <ThemeProvider theme={theme}>
    <CssBaseline />
    <SnackbarProvider maxSnack={2}>
      <Suspense fallback={<Loading />}>
        <App />
      </Suspense>
    </SnackbarProvider>
  </ThemeProvider>,
  document.querySelector('#root'),
);
