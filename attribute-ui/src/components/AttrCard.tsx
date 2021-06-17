import React from 'react';
import DoneIcon from '@material-ui/icons/Done';
import VpnKeyIcon from '@material-ui/icons/VpnKey';

type Attributes = [string, string];

export type OrderedGuestAttributes = {
    attributes: Attributes[],
    name: string,
}

export const AttrCard = ({ name, attributes }: OrderedGuestAttributes) => (
    <div className="attr-card">
        <div className="attr attr-header">
            <span className="attr-key check"><DoneIcon htmlColor="green" fontSize="inherit" /></span>
            <span className="attr-value">{name}</span>
        </div>
        {Object.values(attributes).map(([key, value]) => (
            <div className="attr" key={key}>
                <span className="attr-key">{key}:</span>
                <span className="attr-value attr-badge"><VpnKeyIcon htmlColor="gray" fontSize="inherit" />{value}</span>
            </div>
        ))}
    </div>
);