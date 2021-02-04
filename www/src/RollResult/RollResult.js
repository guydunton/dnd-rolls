import React from 'react';
import ResultField from '../ResultField/ResultField';
import './RollResult.css';

/** @type {React.FunctionComponent<{text?: string}> */
const RollResult = ({ text }) => {
  try {
    const { add_dice, sub_dice, modifier, total } = JSON.parse(text);

    return (
      <div className='results'>
        <h2>Result</h2>
        {add_dice.map(({ dice, rolls }) => (
          <ResultField key={dice} name={`D${dice}`} results={rolls} />
        ))}
        {sub_dice.map(({ dice, rolls }) => (
          <ResultField key={dice} name={`D${dice}`} results={rolls} />
        ))}
        <ResultField name='Modifier' results={[modifier]} />
        <div className='result__total'>
          <ResultField name='Total' results={[total]} />
        </div>
      </div>
    );
  } catch (err) {
    return <div></div>;
  }
};

export default RollResult;
