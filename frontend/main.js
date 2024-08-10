import React, { lazy, Suspense } from 'react';
import ReactDOM from 'react-dom';
import { BrowserRouter as Router, Routes, Route } from 'react-router-dom';
// Lazy load components
const App = lazy(() => import('./App'));
const HomePage = lazy(() => import('./components/HomePage'));
const NotFoundPage = lazy(() => import('./components/NotFoundPage'));

import './index.css';

const API_URL = process.env.REACT_APP_API_URL;

function Main() {
    return (
        <Router>
            <Suspense fallback={<div>Loading...</div>}>
                <Routes>
                    <Route path="/" element={<App />}>
                        <Route index element={<HomePage apiUrl={API_URL} />} />
                        <Route path="*" element={<NotFoundPage />} />
                    </Route>
                </Routes>
            </Suspense>
        </Router>
    );
}

ReactDOM.render(<Main />, document.getElementById('root'));