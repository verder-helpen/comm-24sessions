import styled from 'styled-components';

interface PictureDivProps {
  readonly image: string;
}

const PictureDiv = styled.div<PictureDivProps>`
  min-height: 250px;
  background-image: url(${((props) => props.image)});
  background-repeat: no-repeat;
  background-size: contain;
  background-position: top center;
`;

export default PictureDiv;
