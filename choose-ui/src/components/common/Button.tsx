import styled, { css } from 'styled-components';
import { Link } from 'react-router5';
import React from 'react';

const Button = styled(({ primary, fullWidth, ...props }) => <Link {...props} />)`
  display: inline-block;
  border: 1px solid black;
  color: black;
  font-size: 20px;
  padding: 12px 40px;
  border-radius: 24px;
  text-decoration: none;
  text-align: center;
  
  ${(props) => props.fullWidth && css`
    display: block;
    width: 100%;
  `}
  
  ${(props) => props.primary && css`
    background: ${(p) => p.theme.colors.primary};
    color: white;
    border: none;
  `}
`;

export default Button;
