import styled from 'styled-components';
import { Link } from 'react-router5';
import arrowVector from '../../../images/arrow.svg';

const LinkArrow = styled(Link)`
  font-size: inherit;
  color: ${(p) => p.theme.colors.text};

  &:after {
    display: inline-block;
    content: ' ';
    width: 18px;
    height: 16px;
    margin-left: 10px;

    background: url(${arrowVector}) no-repeat right center;
  }
`;

export default LinkArrow;
