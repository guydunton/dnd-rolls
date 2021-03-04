import React from 'react';
import './RollHelp.css';

const RollHelp = () => (
  <div className='help'>
    <h2>How to use</h2>
    <p>Enter text to roll digital dice</p>
    <ul>
      <li>d6</li>
      <li>3d8</li>
      <li>3d8 + 2</li>
      <li>2d8 + d6 - 1</li>
    </ul>
  </div>
);

export default RollHelp;
