import React, { useState } from 'react';
import ReactDOM from 'react-dom';
import RollInput from './RollInput/RollInput';
import * as wasm from 'dnd-rolls';
import RollResult from './RollResult/RollResult';
import Nav from './Nav/Nav';
import './index.css';

const App = () => {
  const [text, setText] = useState(
    `{"add_dice":[{"dice":8,"rolls":[3,3,1]}],"modifier":-1,"sub_dice":[],"total":7}`
  );
  // const [text, setText] = useState(undefined);

  /** @type {(text: string) => void} */
  const onSubmit = (text) => {
    const result = wasm.dnd_roll_json(text);
    setText(result);
  };

  return (
    <div>
      <div className='container'>
        <Nav />
        <RollInput onSubmit={onSubmit} />
        <RollResult text={text} />
      </div>
    </div>
  );
};

ReactDOM.render(<App />, document.getElementById('root'));
