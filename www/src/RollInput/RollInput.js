import React, { useState } from 'react';
import './RollInput.css';

/** @type {React.FunctionComponent<{ onSubmit: (text: string) => void}>} */
const RollInput = ({ onSubmit }) => {
  const [text, setText] = useState('');

  /** @type {(e: React.FormEvent<HTMLInputElement>) => void} */
  const onChangeText = (e) => {
    e.preventDefault();
    setText(e.currentTarget.value);
  };

  return (
    <div className='roll_input'>
      <label htmlFor='roll-text'>Roll</label>
      <input
        type='text'
        id='roll-text'
        placeholder='Enter roll e.g. 2d8 + 2'
        onChange={onChangeText}
        value={text}
      />
      <button
        onClick={() => {
          onSubmit(text);
        }}
      >
        Roll
      </button>
    </div>
  );
};

export default RollInput;
