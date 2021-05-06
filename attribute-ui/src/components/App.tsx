import React, { useEffect, useState } from 'react';
import { AttrCard } from './AttrCard';

type Attrubutes = {
    [Name in string]: {
        attributes: {
            [Key in string]: string
        }
    }
}

declare global {
    interface Window {
        SERVER_URL: string
    }
};

export const App = () => {
    const serverUrl = window.SERVER_URL;
    const hostToken = window.location.pathname.substr(1);

    const poll = () => fetch(`${serverUrl}/session_info/${hostToken}`).then(r => r.json())

    const [attrs, setAttrs] = useState<Attrubutes>(null);
    useEffect(() => void poll().then(setAttrs), [])
    if (!attrs) {
        return <div>There are no identified guests yet</div>
    }  
    

    return (<>
        
        {Object.entries(attrs).map(([name, {attributes}]) => {
            return <AttrCard key={name} name={name} attributes={attributes}/>
        })}
    </>);
}
