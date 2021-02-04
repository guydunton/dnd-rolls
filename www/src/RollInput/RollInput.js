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
      <form
        onSubmit={(e) => {
          e.preventDefault();
          onSubmit(text);
        }}
      >
        <div className='roll_input__field'>
          <input
            type='text'
            id='roll-text'
            placeholder='Enter roll e.g. 2d8 + 2'
            aria-label='Roll text field'
            onChange={onChangeText}
            value={text}
          />
          <button type='submit'>Roll</button>
        </div>
      </form>
    </div>
  );
};

export default RollInput;
