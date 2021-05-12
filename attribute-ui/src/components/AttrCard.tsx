import React from 'react';

interface Props {
    name: String,
    attributes: {
        [Key in string]: string
    }
}

export const AttrCard = ({ name, attributes }) => 
    <div>Name: {name}<br/> Attributes: {JSON.stringify(attributes)}<br/></div>
