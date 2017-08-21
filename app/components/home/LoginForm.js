import React from 'react';
import TextField from 'material-ui/TextField';
import RaisedButton from 'material-ui/RaisedButton';
import RequestUtils from '../../utils/RequestUtils';

export default class LoginForm extends React.Component {
  state = {
    username: '',
    password: '',
    usernameError: '',
    passwordError: ''
  };

  handleUsernameChange = (e) => {
    this.setState({ username: e.target.value });
  }

  handlePasswordChange = (e) => {
    this.setState({ password: e.target.value });
  }

  onSubmit = (e) => {
    e.preventDefault();

    const { username, password } = this.state;
    RequestUtils.apiRequest('login', {
        username,
        password
      }).then((json) => {
        if (json.success) {
          location.reload();
        } else {
          this.setState({
            usernameError: '',
            passwordError: '',
            [json.error_type + 'Error']: json.error_description
          });
        }
      });
  }

  render() {
    return (
      <div>
        <h1>Login</h1>
        <form
          onSubmit={this.onSubmit}
          style={{ width: '50%' }}
        >
          <TextField
            floatingLabelText="Username"
            value={this.state.username}
            onChange={this.handleUsernameChange}
            errorText={this.state.usernameError}
            fullWidth
          />
          <br />
          <TextField
            floatingLabelText="Password"
            value={this.state.password}
            onChange={this.handlePasswordChange}
            errorText={this.state.passwordError}
            type="password"
            fullWidth
          />
          <br />
          <br />
          <RaisedButton
            label="Login"
            onClick={this.onSubmit}
            secondary
            fullWidth
          />
        </form>
      </div>
    );
  }
}
