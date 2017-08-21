import React from 'react';
import TextField from 'material-ui/TextField';
import RaisedButton from 'material-ui/RaisedButton';

export default class RegisterForm extends React.Component {
  state = {
    username: '',
    email: '',
    name: '',
    password: '',
    usernameError: '',
    emailError: '',
    nameError: '',
    passwordError: ''
  };

  handleChange = (e) => {
    const target = e.target;
    this.setState({ [target.name]: target.value });
  }

  onSubmit = (e) => {
    e.preventDefault();
    alert('form submit');
  }

  render() {
    return (
      <div>
        <h1>Register</h1>
        <form
          onSubmit={this.onSubmit}
          style={{ width: '50%' }}
        >
          <TextField
            floatingLabelText="Username"
            name="username"
            value={this.state.username}
            onChange={this.handleChange}
            errorText={this.state.usernameError}
            fullWidth
          />
          <br />
          <TextField
            floatingLabelText="Email"
            name="email"
            value={this.state.email}
            onChange={this.handleChange}
            errorText={this.state.emailError}
            fullWidth
          />
          <br />
          <TextField
            floatingLabelText="Name"
            name="name"
            value={this.state.name}
            onChange={this.handleChange}
            errorText={this.state.nameError}
            fullWidth
          />
          <br />
          <TextField
            floatingLabelText="Password"
            name="password"
            value={this.state.password}
            onChange={this.handleChange}
            errorText={this.state.passwordError}
            type="password"
            fullWidth
          />
          <br />
          <br />
          <RaisedButton
            label="Register"
            onClick={this.onSubmit}
            secondary
            fullWidth
          />
        </form>
      </div>
    );
  }
}
