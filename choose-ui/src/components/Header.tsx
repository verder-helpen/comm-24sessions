import React from 'react';
import styled from 'styled-components';
import translations from '../translations';
import logo from '../../images/demodam.svg';

interface HeaderProps {
  routeName?: string;
  routeParams?: {
    [key: string]: any;
  };
}

const HeaderContainer = styled.header`
  display: flex;
  justify-content: space-between;
  padding: 10px 0;
  height: 20px;

  @media (min-width: ${(p) => p.theme.maxWidth}px) {
    position: absolute;
    width: 94vw;
    margin-left: -47vw;
    left: 50%;
    top: 40px;
  }

  @media (min-width: 1200px) {
    width: 1000px;
    margin-left: -500px;
  }
`;

const Provider = styled.div`
  position: relative;
  top: -8px;
  background: url(${logo}) no-repeat;
  background-size: contain;
  width: 130px;
  height: 40px;

  span {
    display: none;
  }
`;

export default function Header({ ...props }: HeaderProps) {
  return (
    <HeaderContainer>
      <span />
      <Provider>
        <span>
          {translations.provider_name}
        </span>
      </Provider>
    </HeaderContainer>
  );
}
