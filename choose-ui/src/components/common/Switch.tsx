import styled from 'styled-components';
import downImage from '../../../images/down.svg';

const Switch = styled.div`
  text-align: center;

  a {
    display: inline-block;
    border: 1px solid black;
    color: black;
    width: 220px;
    height: 50px;
    line-height: 46px;
    text-decoration: none;
    font-size: 18px;
    border-radius: 25px;
    margin: 20px auto;

    &:after {
      display: inline-block;
      content: ' ';
      right: 20px;
      width: 20px;
      height: 10px;
      margin-left: 5px;
      background: url(${downImage}) no-repeat center center;
    }
  }
`;

export default Switch;
