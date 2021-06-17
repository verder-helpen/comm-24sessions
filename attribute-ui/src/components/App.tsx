import React, { useEffect, useState } from 'react';
import { AttrCard, OrderedGuestAttributes } from './AttrCard';
import { NoAttrs } from './NoAttrs';
import { PoweredBy } from './PoweredBy';

type GuestAttributes = {
    attributes: {
        [Key in string]: string
    },
    name: string,
}

type SessionAttributes = {
    [Id in string]: GuestAttributes
}

declare global {
    interface Window {
        SERVER_URL: string
    }
};

export const App = () => {
    const serverUrl = window.SERVER_URL;
    const hostToken = window.location.pathname.substr(1);

    const poll = (): Promise<SessionAttributes> => fetch(`${serverUrl}/session_info/${hostToken}`).then(r => r.json())

    const [attrs, setAttrs] = useState<OrderedGuestAttributes[]>(null);

    const prepareAttrs = (s: SessionAttributes): OrderedGuestAttributes[] =>
        Object.values(s)
            .sort((record1, record2) => record1.name.localeCompare(record2.name))
            .map(r => ({
                name: r.name, attributes: Object.entries(r.attributes)
                    .sort(([a], [b]) => a.localeCompare(b)).map(([k, v]) => [k, v])
            }));
    // Poll backend to check whether attributes have been received for current session
    useEffect(() => {
        poll().then(s => setAttrs(prepareAttrs(s)));
        const interval = setInterval(() => void poll().then(s => setAttrs(prepareAttrs(s))), 5000);
        return () => clearInterval(interval);
    }, [])

    const attrsAvailable = attrs && Object.keys(attrs).length > 0;

    return (<>
        <div className="id-contact">
            {attrsAvailable
                ? attrs
                    .map((record, i) => (
                        record ? <AttrCard key={i} name={record.name} attributes={record.attributes} /> : <></>
                    ))
                : <NoAttrs />}
            <PoweredBy />
        </div>
    </>)
}
