import React from 'react';
import DiceResult from '../DiceResult/DiceResult';
import './RollResult.css';

/** @type {React.FunctionComponent<{text?: string}> */
const RollResult = ({ text }) => {
  try {
    const { add_dice, sub_dice, modifier, total } = JSON.parse(text);

    return (
      <div>
        {add_dice.map(({ dice, rolls }) => (
          <DiceResult key={dice} dice={dice} rolls={rolls} />
        ))}
        {sub_dice.map(({ dice, rolls }) => (
          <DiceResult key={dice} dice={dice} rolls={rolls} />
        ))}
        <h3>Modifier</h3>
        <p>{modifier}</p>
        <h3>Total</h3>
        <p>{total}</p>
      </div>
    );
  } catch (err) {
    return <div></div>;
  }
};

export default RollResult;
