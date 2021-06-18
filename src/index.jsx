import React, { Suspense } from 'react';
import ReactDOM from 'react-dom';
import { CssBaseline } from '@material-ui/core';
import { ThemeProvider } from '@material-ui/core/styles';
import { SnackbarProvider } from 'notistack';
import { HashRouter } from 'react-router-dom';
import { Route, Switch } from 'react-router';
import theme from './theme';
import Loading from './Loading';

const LocalGame = React.lazy(() => import('./LocalGame'));
const RemoteGame = React.lazy(() => import('./RemoteGame'));

ReactDOM.render(
  <ThemeProvider theme={theme}>
    <HashRouter>
      <CssBaseline />
      <SnackbarProvider maxSnack={2}>
        <Switch>
          <Route path="/" exact>
            <Suspense fallback={<Loading />}>
              <LocalGame />
            </Suspense>
          </Route>
          <Route path={['/r/:roomId', '/r/']}>
            <Suspense fallback={<Loading />}>
              <RemoteGame />
            </Suspense>
          </Route>
        </Switch>
      </SnackbarProvider>
    </HashRouter>
  </ThemeProvider>,
  document.querySelector('#root'),
);
