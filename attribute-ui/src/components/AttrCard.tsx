import React from 'react';
import DoneIcon from '@material-ui/icons/Done';
import VpnKeyIcon from '@material-ui/icons/VpnKey';
interface Props {
    name: String,
    attributes: {
        [Key in string]: string
    }
}

export const AttrCard = ({ name, attributes }: Props) => (
    <div className="attr-card">
        <div className="attr attr-header">
            <span className="attr-key check"><DoneIcon htmlColor="green" fontSize="inherit"/></span>
            <span className="attr-value">{name}</span>
        </div>
        {Object.entries(attributes).map(([key, value]) => (
            <div className="attr" key={key}>
                <span className="attr-key">{key}:</span>
                <span className="attr-value attr-badge"><VpnKeyIcon htmlColor="gray" fontSize="inherit"/>{value}</span>
            </div>
        ))}
    </div>
);