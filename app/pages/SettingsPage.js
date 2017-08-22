import React from 'react';
import glamorous from 'glamorous';
import CircularProgress from 'material-ui/CircularProgress';
import RaisedButton from 'material-ui/RaisedButton';
import { white, red500 } from 'material-ui/styles/colors';
import simpleForm from '../components/simpleForm';
import RequestUtils from '../utils/RequestUtils';

const Container = glamorous.div({
  padding: 24
});

export default class SettingsPage extends React.Component {
  state = {
    loading: true,
    user: {}
  };

  componentDidMount() {
    RequestUtils.apiGetRequest('get_user').then((user) => {
      this.setState({
        loading: false,
        user
      });
    });
  }

  onDelete = (e) => {
    const prompt = window.prompt("Type your username to confirm account deletion. There is no way back!");
    const confirmed = prompt == this.state.user.username;
    if (!confirmed) return;

    RequestUtils.apiRequest('delete_user').then((json) => {
      location.reload();
    });
  }

  render() {
    let SettingsForm;

    if (!this.state.loading) {
      const defaults = this.state.user;
      defaults.password = '';

      SettingsForm = simpleForm(
        'Update settings',
        'update_user',
        ['username', 'email', 'name', 'password'],
        defaults
      );
    }

    return (
      <Container>
        {this.state.loading ? (
          <div style={{ textAlign: 'center' }}>
            <CircularProgress />
          </div>
        ) : (
          <div>
            <h1>Settings for {this.state.user.name}</h1>
            <SettingsForm />
            <br />
            <br />
            <br />
            <br />
            <div style={{ width: '50%' }}>
              <RaisedButton
                label="Delete account"
                labelColor={white}
                backgroundColor={red500}
                onClick={this.onDelete}
                fullWidth
              />
            </div>
          </div>
        )}
      </Container>
    );
  }
}
