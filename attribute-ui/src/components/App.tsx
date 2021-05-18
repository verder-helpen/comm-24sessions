import React, { useEffect, useState } from 'react';
import { AttrCard } from './AttrCard';
import { NoAttrs } from './NoAttrs';
import { PoweredBy } from './PoweredBy';

type Attributes = {
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

    const [attrs, setAttrs] = useState<Attributes>(null);

    // Poll backend to check whether attributes have been received for current session
    useEffect(() => {
        poll().then(setAttrs);
        const interval = setInterval(() => void poll().then(setAttrs), 5000);
        return () => clearInterval(interval);
    }, [])

    const attrsAvailable = attrs && Object.keys(attrs).length > 0;

    return (<>
        <div className="id-contact">
            {attrsAvailable
                ? Object.entries(attrs).map(([name, { attributes }]) => (
                    <AttrCard key={name} name={name} attributes={attributes} />
                ))
                : <NoAttrs />}
            <PoweredBy />
        </div>
    </>)
}
