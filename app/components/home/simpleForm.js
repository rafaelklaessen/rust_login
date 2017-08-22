import React from 'react';
import TextField from 'material-ui/TextField';
import RaisedButton from 'material-ui/RaisedButton';
import { FormContainer, FormTitle, Form } from './Form';
import RequestUtils from '../../utils/RequestUtils';

const simpleForm = (title, apiUrl, fields) => class extends React.Component {
  constructor(props) {
    super(props);

    this.state = {};

    for (let field of fields) {
      this.state[field] = '';
      this.state[`${field}Error`] = '';
    }
  }

  handleChange = (e) => {
    const target = e.target;
    this.setState({ [target.name]: target.value });
  }

  onSubmit = (e) => {
    e.preventDefault();

    RequestUtils.apiRequest(apiUrl, this.state).then((json) => {
      if (json.success) {
        location.reload();
      } else {
        const errorState = {};
        for (let field of fields) {
          errorState[`${field}Error`] = '';
        }
        errorState[`${json.error_type}Error`] = json.error_description;
        this.setState(errorState);
      }
    });
  }

  render() {
    return (
      <FormContainer>
        <FormTitle>{title}</FormTitle>
        <Form onSubmit={this.onSubmit}>
          {fields.map((field) =>
            <div key={field}>
              <TextField
                floatingLabelText={capitalize(field)}
                name={field}
                value={this.state[field]}
                onChange={this.handleChange}
                errorText={this.state[`${field}Error`]}
                type={getType(field)}
                fullWidth
              />
              <br />
            </div>
          )}
          <br />
          <RaisedButton
            label={title}
            onClick={this.onSubmit}
            secondary
            fullWidth
          />
        </Form>
      </FormContainer>
    );
  }
}

export default simpleForm;

const capitalize = (string) =>
    string.charAt(0).toUpperCase() + string.slice(1);

const getType = (field) => field == 'password' ? 'password' : 'text';
