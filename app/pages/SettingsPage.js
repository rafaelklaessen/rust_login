import React from 'react';
import glamorous from 'glamorous';
import CircularProgress from 'material-ui/CircularProgress';
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

  render() {
    return (
      <Container>
        {this.state.loading ? (
          <div style={{ textAlign: 'center' }}>
            <CircularProgress />
          </div>
        ) : (
          <h1>Settings for {this.state.user.name}</h1>
        )}
      </Container>
    );
  }
}
