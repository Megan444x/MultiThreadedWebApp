import React from 'react';
import ReactDOM from 'react-dom';
import { BrowserRouter as Router, Routes, Route } from 'react-router-dom';
import App from './App';
import HomePage from './components/HomePage';
import NotFoundPage from './components/NotFoundPage';
import './index.css';

const API_URL = process.env.REACT_APP_API_URL;

function Main() {
    return (
        <Router>
            <Routes>
                <Route path="/" element={<App />}>
                    <Route index element={<HomePage apiUrl={API_URL} />} />
                    <Route path="*" element={<NotFoundPage />} />
                </Route>
            </Routes>
        </Router>
    );
}

ReactDOM.render(<Main />, document.getElementById('root'));