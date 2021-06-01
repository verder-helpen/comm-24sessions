import styled from 'styled-components';
import { Link } from 'react-router5';
import backVector from '../../../images/back.svg';

const Back = styled(Link)`
  font-size: 18px;
  color: ${(p) => p.theme.colors.text};

  &:before {
    display: inline-block;
    content: ' ';
    width: 18px;
    height: 16px;
    margin-right: 10px;

    background: url(${backVector}) no-repeat left bottom;
  }
`;

export default Back;
