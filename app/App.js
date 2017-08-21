import React from 'react';
import glamorous from 'glamorous';
import MuiThemeProvider from 'material-ui/styles/MuiThemeProvider';
import AppBar from 'material-ui/AppBar';
import {
  BrowserRouter as Router,
  Route,
  Switch,
  Link
} from 'react-router-dom';
import Index from 'pages/Index';
import NotFoundPage from 'pages/NotFoundPage';

const WithFont = glamorous.div({
  fontFamily: 'Roboto'
});

const App = () => (
  <WithFont>
    <MuiThemeProvider>
      <div>
        <AppBar
          title="Rust login"
          iconElementLeft={<span />}
        />
        <Router>
          <Switch>
            <Route exact path="/" component={Index} />
            <Route component={NotFoundPage} />
          </Switch>
        </Router>
      </div>
    </MuiThemeProvider>
  </WithFont>
);

export default App;
