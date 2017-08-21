import React from 'react';
import HomePage from './HomePage';
import SettingsPage from './SettingsPage';

export default function Index() {
  if (loggedIn) {
    return <SettingsPage />;
  } else {
    return <HomePage />;
  }
}
