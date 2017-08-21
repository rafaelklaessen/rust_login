import React from 'react';
import glamorous from 'glamorous';
import LoginForm from '../components/home/LoginForm';
import RegisterForm from '../components/home/RegisterForm';

const Col = glamorous.div({
  float: 'left',
  padding: 20,
  width: '50%',
  boxSizing: 'border-box'
});

const HomePage = () => (
  <div>
    <div>
      <Col>
        <LoginForm />
      </Col>
      <Col>
        <RegisterForm />
      </Col>
    </div>
  </div>
);

export default HomePage;
