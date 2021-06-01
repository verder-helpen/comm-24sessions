import styled from 'styled-components';

interface PictureRowProps {
  readonly image: string;
}

const PictureRow = styled.div<PictureRowProps>`
  padding-top: 10px;
  
  @media (min-width: ${(p) => p.theme.maxWidth}px) {
    position: relative;
    padding-right: 350px;
    
    &:after {
      position: absolute;
      top: 0;
      right: 0;
      width: 350px;
      height: 350px;
      content: ' ';
      background-image: url(${((props) => props.image)});
      background-repeat: no-repeat;
      background-size: contain;
      background-position: right;
    }
  }
`;

export default PictureRow;
