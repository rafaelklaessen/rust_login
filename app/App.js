import React from 'react';
import 'whatwg-fetch';
import glamorous from 'glamorous';
import MuiThemeProvider from 'material-ui/styles/MuiThemeProvider';
import AppBar from 'material-ui/AppBar';
import FlatButton from 'material-ui/FlatButton';
import {
  BrowserRouter as Router,
  Route,
  Switch,
  Link
} from 'react-router-dom';
import Index from 'pages/Index';
import NotFoundPage from 'pages/NotFoundPage';
import RequestUtils from 'utils/RequestUtils';

const WithFont = glamorous.div({
  fontFamily: 'Roboto'
});

const logout = () => {
  RequestUtils.apiRequest('logout').then(() => location.reload());
};

const logoutBtn = loggedIn
                  ? <FlatButton label="Logout" onClick={logout} />
                  : <span />;

const App = () => (
  <WithFont>
    <MuiThemeProvider>
      <div>
        <AppBar
          title="Rust login"
          iconElementLeft={<span />}
          iconElementRight={logoutBtn}
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
