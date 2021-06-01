import React, { ReactEventHandler, ReactNode } from 'react';
import styled from 'styled-components';
import { Method } from '../fetchDependenciesMiddleware';

import next from '../../images/next.svg';
import call from '../../images/phone.svg';
import chat from '../../images/chat.svg';
import video from '../../images/video.svg';
import irma from '../../images/irma.svg';
import digid from '../../images/digid.svg';

const images: {[key: string]: string} = {
  call,
  chat,
  video,
  irma,
  digid,
  test: video,
};

const iconColors: {[key: string]: string} = {
  call: '#018361',
  chat: '#3D9AAB',
  video: '#D95E35',
  test: '#D95E35',
};

interface ActionProps {
  method: Method;
  onClick: ReactEventHandler;
  children?: ReactNode;
}

const ListItem = styled.li`
  display: block;
  margin: 20px 0;

  a {
    position: relative;
    display: block;
    color: black;
    text-decoration: none;
    border-radius: 24px;
    width: 100%;
    height: 80px;
    margin: 24px auto;
    max-width: 100%;
    font-size: 18px;
    background: white;
    cursor: pointer;
    box-shadow: 0 2px 2px rgba(0, 0, 0, 0.15);
    line-height: 80px;
    text-align: center;
    transition:
      box-shadow 100ms ease-in-out,
      color 80ms ease-in-out;

    &:hover {
      color: #000;
      border-color: darken(${(p) => p.theme.colors.primary}, 10%);
      box-shadow: 0 0.4rem 0.4rem rgba(#000, 0.25);
    }

    &:after {
      content: ' ';
      position: absolute;
      right: 20px;
      width: 10px;
      height: 80px;
      background: url(${next}) no-repeat center center;
    }
  }

  em {
    color: ${(p) => p.theme.colors.primary};
    font-style: normal;
  }
`;

interface IconProps {
  variant: string;
  image: string;
}

const Icon = styled.span<IconProps>`
  position: absolute;
  left: 0;
  display: inline-block;
  width: 68px;
  height: 68px;
  border-radius: 24px;
  margin: 6px;
  text-align: center;
  vertical-align: bottom;
  background: url(${(props) => props.image}) no-repeat center center;
  background-color: ${(props) => props.color || 'transparent'};
  background-size: ${(props) => (props.color ? 'auto' : '100%')};
`;

/**
 * Generic action button
 * @param {Method} method
 * @param {string} routeName
 * @param {{[p: string]: string}} params
 * @param {React.ReactElement} children
 * @returns {JSX.Element}
 * @constructor
 */
export default function Action({
  method, onClick, children,
}: ActionProps) {
  return (
    <ListItem>
      <a
        title={method.name}
        onClick={onClick}
      >
        <Icon
          variant={method.tag}
          image={images[method.tag]}
          color={iconColors[method.tag] || null}
        />
        <span>
          {method.name}
        </span>
      </a>
      {children}
    </ListItem>
  );
}
