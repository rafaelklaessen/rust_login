const formurlencoded = require('form-urlencoded');

export default class RequestUtils {
  static apiRequest(url, data = {}) {
    return this.request(`/api/${url}`, data);
  }

  static request(url, data = {}) {
    return fetch(url, {
        headers: {
          'Content-Type': 'application/x-www-form-urlencoded'
        },
        credentials: 'include',
        method: 'post',
        body: formurlencoded(data)
      })
      .then((response) => response.json());
  }
}
