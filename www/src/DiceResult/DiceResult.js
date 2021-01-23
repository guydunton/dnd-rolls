import React from 'react';

/** @type {React.FunctionComponent<{dice: number, rolls: number[]}>} */
const DiceResult = ({ dice, rolls }) => (
  <div>
    <h3>D{dice}</h3>
    <ol>
      {rolls.map((roll, i) => (
        <li key={i}>{roll}</li>
      ))}
    </ol>
  </div>
);

export default DiceResult;
