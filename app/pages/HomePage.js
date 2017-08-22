import React from 'react';
import glamorous from 'glamorous';
import simpleForm from '../components/home/simpleForm';

const Col = glamorous.div({
  float: 'left',
  padding: 24,
  width: '50%',
  textAlign: 'center',
  boxSizing: 'border-box'
});

const LoginForm = simpleForm(
  'Login',
  'login',
  ['username', 'password']
);

const RegisterForm = simpleForm(
  'Register',
  'register',
  ['username', 'email', 'name', 'password']
);

const HomePage = () => (
  <div>
    <Col>
      <LoginForm />
    </Col>
    <Col>
      <RegisterForm />
    </Col>
  </div>
);

export default HomePage;
