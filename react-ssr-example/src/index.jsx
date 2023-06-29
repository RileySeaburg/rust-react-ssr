import React from 'react';
import ReactDOM from 'react-dom';
import './index.css';
import App from './App';
import reportWebVitals from './reportWebVitals';
import { renderToString } from 'react-dom/server';

ReactDOM.hydrate(

  <App />,
  document.getElementById('root')
);

reportWebVitals();