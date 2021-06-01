import React from 'react';
import { render } from 'react-dom';
import { ThemeProvider } from 'styled-components';
import App from './components/App';
import theme, { GlobalStyle } from './theme';

const root = document.getElementById('root');

render((
  <ThemeProvider theme={theme}>
    <App />
    <GlobalStyle />
  </ThemeProvider>
), root);
