import styled from 'styled-components';

const Contained = styled.div`
  padding: 20px 0 40px 0;
  border-radius: 12px;
  font-size: 18px;
  line-height: 40px;

  @media (min-width: ${(p) => p.theme.maxWidth}px) {
    padding: 20px 40px;
    max-width: 450px;
    margin: auto;
    box-shadow: 0 0 12px rgba(0, 0, 0, 0.15);
  }
`;

export default Contained;
