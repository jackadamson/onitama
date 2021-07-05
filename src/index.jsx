import React, { Suspense } from 'react';
import ReactDOM from 'react-dom';
import { CssBaseline } from '@material-ui/core';
import { ThemeProvider } from '@material-ui/core/styles';
import { SnackbarProvider } from 'notistack';
import { HashRouter } from 'react-router-dom';
import { Route, Switch } from 'react-router';
import 'typeface-roboto';
import theme from './theme';
import Loading from './Loading';
import Home from './Home';
import AiSelect from './AiSelect';

const LocalGame = React.lazy(() => import('./LocalGame'));
const RemoteGame = React.lazy(() => import('./RemoteGame'));

ReactDOM.render(
  <ThemeProvider theme={theme}>
    <HashRouter>
      <CssBaseline />
      <SnackbarProvider maxSnack={2}>
        <Switch>
          <Route path="/l/" exact>
            <Suspense fallback={<Loading />}>
              <LocalGame />
            </Suspense>
          </Route>
          <Route path={['/r/:roomId', '/r/']}>
            <Suspense fallback={<Loading />}>
              <RemoteGame />
            </Suspense>
          </Route>
          <Route path="/ai/:roomId">
            <Suspense fallback={<Loading />}>
              <RemoteGame isAi />
            </Suspense>
          </Route>
          <Route path="/ai">
            <AiSelect />
          </Route>
          <Route>
            <Home />
          </Route>
        </Switch>
      </SnackbarProvider>
    </HashRouter>
  </ThemeProvider>,
  document.querySelector('#root'),
);
