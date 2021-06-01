import React from 'react';
import styled from 'styled-components';

type FontSize = 'small'|'default'|'large';

interface TitleProps {
  content: string;
  colored?: string;
  size?: FontSize;
}

interface H2Props {
  size: FontSize;
}

const fontSizes = {
  small: 24,
  default: 35,
  large: 45,
};

const H2 = styled.h2<H2Props>`
  font-weight: 600;
  font-size: ${(p) => fontSizes[p.size]}px;
  color: ${(p) => p.theme.colors.text};
  padding: 0;
  margin: 20px auto 40px auto;
  line-height: 55px;

  em {
    position: relative;
    font-style: normal;

    &:after {
      position: absolute;
      content: ' ';
      width: 90%;
      min-width: 16px;
      height: 6px;
      border-radius: 4px;
      background: ${(p) => (p.color ? p.color : p.theme.colors.primaryLight)};
      bottom: -10px;
      left: 5%;
    }
  }
  
  strong {
    color: ${(p) => p.theme.colors.primary};
    text-transform: lowercase;
    
    &:before {
      content: ' ';
    }
  }
`;

export default function Title({ content, colored, size = 'default' }: TitleProps) {
  return (
    <H2 size={size}>
      <em>{content[0]}</em>
      <span>{content.slice(1)}</span>
      {colored && (
        <strong>
          {colored}
        </strong>
      )}
    </H2>
  );
}
