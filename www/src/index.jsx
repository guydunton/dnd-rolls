import React, {useState} from 'react';
import ReactDOM from 'react-dom';
import RollInput from './RollInput/RollInput';
import * as wasm from 'dnd-rolls';
import RollResult from './RollResult/RollResult';

const App = () => {
    const [text, setText] = useState(undefined);

    /** @type {(text: string) => void} */
    const onSubmit = (text) => {
        setText(wasm.dnd_roll_json(text));
    };
    
    return (
    <div>
        <h1>D&D Rolls</h1>
        <RollInput onSubmit={onSubmit}/>
        <h2>Result</h2>
        <RollResult text={text} />
    </div>
);
};

ReactDOM.render(<App />, document.getElementById('root'));
