import { DefaultTheme, createGlobalStyle } from 'styled-components';
import font400woff from '../fonts/montserrat-v15-latin-regular.woff';
import font400woff2 from '../fonts/montserrat-v15-latin-regular.woff2';
import font600woff from '../fonts/montserrat-v15-latin-600.woff';
import font600woff2 from '../fonts/montserrat-v15-latin-600.woff2';

const theme: DefaultTheme = {
  maxWidth: 900,

  colors: {
    text: '#363636',
    primary: '#1F7B8C',
    primaryLight: '#77C0D1',
    primaryBackground: '#E9F0FF',
  },
};

export const GlobalStyle = createGlobalStyle`
  html, body, div, span, applet, object, iframe,
  h1, h2, h3, h4, h5, h6, p, blockquote, pre,
  a, abbr, acronym, address, big, cite, code,
  del, dfn, em, img, ins, kbd, q, s, samp,
  small, strike, strong, sub, sup, tt, var,
  b, u, i, center,
  dl, dt, dd, ol, ul, li,
  fieldset, form, label, legend,
  table, caption, tbody, tfoot, thead, tr, th, td,
  article, aside, canvas, details, embed,
  figure, figcaption, footer, header, hgroup,
  menu, nav, output, ruby, section, summary,
  time, mark, audio, video {
    margin: 0;
    padding: 0;
    border: 0;
    font-size: 100%;
    vertical-align: baseline;
  }

  article, aside, details, figcaption, figure,
  footer, header, hgroup, menu, nav, section {
    display: block;
  }

  html {
    box-sizing: border-box;
  }

  body {
    line-height: 1;
  }

  ol, ul {
    list-style: none;
  }

  blockquote, q {
    quotes: none;
  }

  blockquote:before, blockquote:after,
  q:before, q:after {
    content: none;
  }

  table {
    border-collapse: collapse;
    border-spacing: 0;
  }

  *, *:before, *:after {
    box-sizing: inherit;
  }

  // see https://google-webfonts-helper.herokuapp.com/fonts/montserrat?subsets=latin

  /* montserrat-regular - latin */
  @font-face {
    font-family: 'Montserrat';
    font-style: normal;
    font-weight: 400;
    src: local(''),
    url(${font400woff2}) format('woff2'), /* Chrome 26+, Opera 23+, Firefox 39+ */
    url(${font400woff}) format('woff'); /* Chrome 6+, Firefox 3.6+, IE 9+, Safari 5.1+ */
  }

  /* montserrat-600 - latin */
  @font-face {
    font-family: 'Montserrat';
    font-style: normal;
    font-weight: 600;
    src: local(''),
    url(${font600woff2}) format('woff2'), /* Chrome 26+, Opera 23+, Firefox 39+ */
    url(${font600woff}) format('woff'); /* Chrome 6+, Firefox 3.6+, IE 9+, Safari 5.1+ */
  }

  body {
    font-family: "Montserrat", sans-serif;
    color: ${(p) => p.theme.colors.text};
    font-size: 18px;
    background-color: ${(p) => p.theme.colors.primaryBackground};
  }

  html, body, #root {
    width: 100%;
    height: 100%;
    overflow-x: hidden;
  }
`;

export default theme;
