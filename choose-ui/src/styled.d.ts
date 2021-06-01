import 'styled-components';

declare module 'styled-components' {
  export interface DefaultTheme {
    maxWidth: number;

    colors: {
      text: string;
      primary: string;
      primaryLight: string;
      primaryBackground: string;
    };
  }
}
