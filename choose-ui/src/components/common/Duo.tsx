import styled from 'styled-components';

interface DuoProps {
  reversed: boolean;
}

const Duo = styled.div<DuoProps>`
  display: flex;
  margin: 20px -20px;
  
  & > * {
    flex: 1;
    margin: 20px;
  }
  
  @media (max-width: ${(p) => p.theme.maxWidth}px) {
    flex-direction: ${(p) => (p.reversed ? 'column-reverse' : 'column')};
  }
`;

export default Duo;
