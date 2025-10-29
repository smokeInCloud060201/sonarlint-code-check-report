import { BrowserRouter as Router, Routes, Route } from 'react-router-dom';
import { ProjectListPage } from './pages/ProjectListPage';
import './App.css';

function App() {
  return (
    <Router>
      <div className="app">
        <Routes>
          <Route path="/" element={<ProjectListPage />} />
        </Routes>
      </div>
    </Router>
  );
}

export default App;
