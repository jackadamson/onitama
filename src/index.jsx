import React, { Suspense } from 'react';
import { createRoot } from 'react-dom/client';
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
import * as serviceWorkerRegistration from './serviceWorkerRegistration';
import TrainingSelect from './TrainingSelect';

const LocalGame = React.lazy(() => import('./LocalGame'));
const RemoteGame = React.lazy(() => import('./RemoteGame'));
const SinglePlayerGame = React.lazy(() => import('./SinglePlayerGame'));
const TrainingGame = React.lazy(() => import('./TrainingGame'));
const Rules = React.lazy(() => import('./Rules'));
const Settings = React.lazy(() => import('./Settings'));

const root = createRoot(document.querySelector('#root'));
root.render(
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
          <Route path="/help" exact>
            <Suspense fallback={<Loading />}>
              <Rules />
            </Suspense>
          </Route>
          <Route path="/settings" exact>
            <Suspense fallback={<Loading />}>
              <Settings />
            </Suspense>
          </Route>
          <Route path={['/r/:roomId', '/r/']}>
            <Suspense fallback={<Loading />}>
              <RemoteGame />
            </Suspense>
          </Route>
          {process.env.REACT_APP_LOCAL_AI ? (
            <Route path="/ai/:difficulty">
              <Suspense fallback={<Loading />}>
                <SinglePlayerGame />
              </Suspense>
            </Route>
          ) : (
            <Route path="/ai/:roomId">
              <Suspense fallback={<Loading />}>
                <RemoteGame isAi />
              </Suspense>
            </Route>
          )}
          <Route path="/ai">
            <AiSelect />
          </Route>
          {process.env.REACT_APP_LOCAL_AI && (
            <Route path="/t/:difficulty">
              <Suspense fallback={<Loading />}>
                <TrainingGame />
              </Suspense>
            </Route>
          )}
          {process.env.REACT_APP_LOCAL_AI && (
            <Route path="/t">
              <TrainingSelect />
            </Route>
          )}
          <Route>
            <Home />
          </Route>
        </Switch>
      </SnackbarProvider>
    </HashRouter>
  </ThemeProvider>,
);

serviceWorkerRegistration.register();
