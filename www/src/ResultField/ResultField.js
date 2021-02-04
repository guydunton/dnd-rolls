import React from 'react';
import './ResultField.css';

/** @type {React.FunctionComponent<{ name: string, results: number[] }>} */
const ResultField = ({ name, results }) => (
  <div className='result'>
    <h3>{name}</h3>
    <ul>
      {results.map((roll, i) => (
        <li key={i}>{roll}</li>
      ))}
    </ul>
  </div>
);

export default ResultField;
