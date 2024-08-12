import React, { Component } from 'react';

class ErrorBoundary extends Component {
    constructor(props) {
        super(props);
        this.state = { hasError: false, error: null, errorInfo: null };
    }

    static getDerivedStateFromError(error) {
        return { hasError: true };
    }

    componentDidCatch(error, errorInfo) {
        this.setState({
            error: error,
            errorInfo: errorInfo
        });
        console.log(error, errorInfo);
    }

    render() {
        if (this.state.hasError) {
            return <div>
                <h2>Something went wrong.</h2>
                <details style={{ whiteSpace: 'pre-wrap' }}>
                    {this.state.error && this.state.error.toString()}
                    <br />
                    {this.state.errorInfo.componentStack}
                </details>
            </div>;
        }

        return this.props.children; 
    }
}

export default ErrorBoundary;
```
```javascript
import React, { lazy, Suspense } from 'react';
import ReactDOM from 'react-dom';
import { BrowserRouter as Router, Routes, Route } from 'react-router-dom';
import ErrorBoundary from './components/ErrorBoundary'; 
const App = lazy(() => import('./App'));
const HomePage = lazy(() => import('./components/HomePage'));
const NotFoundPage = lazy(() => import('./components/NotFoundPage'));

import './index.css';

const API_URL = process.env.REACT_APP_API_URL;

function Main() {
    return (
        <ErrorBoundary>
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
        </ErrorBoundary>
    );
}

ReactDOM.render(<Main />, document.getElementById('root'));