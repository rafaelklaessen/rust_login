import React from 'react';
import glamorous from 'glamorous';
import MuiThemeProvider from 'material-ui/styles/MuiThemeProvider';
import RaisedButton from 'material-ui/RaisedButton';

const WithFont = glamorous.div({
  fontFamily: 'Roboto'
});

const App = () => (
  <WithFont>
    <MuiThemeProvider>
      <RaisedButton label="Rust login" primary />
    </MuiThemeProvider>
  </WithFont>
);

export default App;
