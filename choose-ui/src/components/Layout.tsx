import React, { ReactNode } from 'react';
import styled from 'styled-components';
import translations from '../translations';
import logo from '../../images/logo.svg';

const Container = styled.div`
  position: relative;
  margin: auto;
  max-width: ${(p) => p.theme.maxWidth}px;
  padding: 0 25px 25px 25px;
`;

const circleSize = 160;

const Logo = styled.h1`
  position: relative;
  background: url('${logo}') no-repeat bottom center;
  height: 35px;
  font-weight: 600;
  margin: 30px auto 40px auto;

  @media (min-width: ${(p) => p.theme.maxWidth}px) {
    margin-bottom: 80px;
  }

  // big white circle
  &:before {
    z-index: -1;
    content: ' ';
    display: block;
    position: absolute;
    width: ${circleSize}vw;
    height: ${circleSize}vw;
    background: white;
    border-radius: ${circleSize / 2}vw;
    top: calc(-${circleSize}vw + 50px);
    left: 50%;
    margin-left: ${-circleSize / 2}vw;

    @media (min-width: ${(p) => p.theme.maxWidth}px) {
      top: calc(-${circleSize}vw + 70px);
    }
  }

  span {
    display: none;
  }
`;

interface LayoutProps {
  children: ReactNode;
}

export default function Layout({ children }: LayoutProps) {
  return (
    <Container>
      <Logo>
        <span>{translations.title}</span>
      </Logo>
      {children}
    </Container>
  );
}
