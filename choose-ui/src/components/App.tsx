import React, { useEffect, useState } from 'react';
import styled from 'styled-components';
import translations from '../translations';
import Layout from './Layout';
import Title from './Title';
import Contained from './common/Contained';
import Action from './Action';
import Actions from './common/Actions';
import Header from './Header';
import parseJwk, { JWK } from "../../node_modules/jose/dist/browser/jwk/parse"
import compactVerify from "../../node_modules/jose/dist/browser/jws/compact/verify"

declare const JWT_VERIFY_KEY: JWK;

// a comm or auth method representation for the core
interface MethodDefinition {
  tag: string;
  name: string;
  // eslint-disable-next-line camelcase
  image_path: string;
}

// a comm or auth method representation for this app
export interface Method {
  tag: string;
  name: string;
  imagePath: string;
}

const Instruction = styled.div`
  font-size: 18px;
  line-height: 25px;
  margin-top: 25px;
`;

declare const ID_CONTACT_ENDPOINT: string;

function decorateMethod(method: MethodDefinition): Method {
  return {
    tag: method.tag,
    name: method.name,
    imagePath: `${ID_CONTACT_ENDPOINT}${method.image_path}`,
  };
}

export default function App({ purpose, comm_display_name } : {purpose: string, comm_display_name: string}) {
  const url = `${ID_CONTACT_ENDPOINT}/session_options/${purpose}`;

  const [inputData, setInput] = useState({decoded: false, purpose: "", comm: "", start: ""});
  const [state, setState] = useState({loaded: false, methods: []});
  useEffect(() => {
    (async () => {
      const key = await parseJwk(JWT_VERIFY_KEY, 'RS256');
      const pathParts = document.location.pathname.split('/');
      const jwt = pathParts[pathParts.length-1];
      const { payload: urldata } = await compactVerify(jwt, key);
      const decoder = new TextDecoder();
      const inputdata = JSON.parse(decoder.decode(urldata));
      setInput({decoded: true, purpose: inputdata.purpose, start: inputdata.start_url, comm: inputdata.display_name});
      const url = `${ID_CONTACT_ENDPOINT}/session_options/${inputdata.purpose}`;
      const response = await fetch(url);
      const responsedata = await response.json();
      setState({loaded: true, methods: responsedata.auth_methods.map(decorateMethod)});
    })();
  }, []);

  if (!state.loaded || !inputData.decoded) {
    return <Layout>
      <Header />
      <Instruction>
        Loading...
      </Instruction>
    </Layout>
  }

  return (
    <Layout>
      <Header />
      <Contained>
        <Title
          content={`${translations.choose_auth_title}:`}
          colored={inputData.comm}
        />
        <Instruction>
          Om u veilig en snel te kunnen helpen vragen wij u om eerst in te loggen.
        </Instruction>
        <Actions>
          {state.methods.map((method) => (
            <Action
              key={method.tag}
              method={method}
              onClick={()=>{
                fetch(inputData.start, {
                  method: 'POST',
                  headers: {
                    Accept: 'application/json',
                    'Content-Type': 'application/json',
                  },
                  body: JSON.stringify({
                    auth_method: method.tag,
                    purpose: inputData.purpose,
                  }),
                })
                  .then((response) => response.json())
                  .then((json) => {window.location.href = json.client_url;});
              }}
            />
          ))}
        </Actions>
      </Contained>
    </Layout>
  );
}
