const formurlencoded = require('form-urlencoded');

export default class RequestUtils {
  static apiGetRequest(url, data = {}) {
return this.request(`/api/${url}`, 'get', data);
  }

  static apiRequest(url, data = {}) {
    return this.request(`/api/${url}`, 'post', data);
  }

  static request(url, method, data = {}) {
    return fetch(url, {
        headers: {
          'Content-Type': 'application/x-www-form-urlencoded'
        },
        credentials: 'include',
        method,
        body: formurlencoded(data) || null
      })
      .then((response) => response.json());
  }
}
