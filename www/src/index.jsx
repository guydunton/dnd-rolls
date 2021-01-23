import React from 'react';
import ReactDOM from 'react-dom';
import * as wasm from 'dnd-rolls';

const App = () => (
    <div>
        <h1>D&D Rolls</h1>
        <p>{wasm.dnd_roll_str('2d8 + 4')}</p>
    </div>
);

ReactDOM.render(<App />, document.getElementById('root'));
