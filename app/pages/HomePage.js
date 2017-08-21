import React from 'react';
import glamorous from 'glamorous';
import AppBar from 'material-ui/AppBar';
import LoginForm from '../components/home/LoginForm';

const Col = glamorous.div({
  float: 'left',
  padding: 20,
  width: '50%',
  boxSizing: 'border-box'
});

const HomePage = () => (
  <div>
    <AppBar
      title="Rust login"
      iconElementLeft={<span />}
    />
    <div>
      <Col>
        <LoginForm />
      </Col>
      <Col>
        <h1>Register</h1>
      </Col>
    </div>
  </div>
);

export default HomePage;
