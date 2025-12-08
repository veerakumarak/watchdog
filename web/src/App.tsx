import { BrowserRouter as Router, Routes, Route, Link } from "react-router-dom"
import JobsList from "./pages/JobsList"
import CreateJob from "./pages/CreateJob"
import EditJob from "./pages/EditJob"
import JobRunDetails from "./pages/JobRunDetails"
import "./App.css"

function App() {
  return (
    <Router>
      <div className="app-container">
        <nav className="sidebar">
          <div className="nav-header">
            <h1>Job Monitor</h1>
          </div>
          <ul className="nav-links">
            <li>
              <Link to="/">Jobs</Link>
            </li>
            <li>
              <Link to="/create">New Job</Link>
            </li>
          </ul>
        </nav>
        <main className="main-content">
          <Routes>
            <Route path="/" element={<JobsList />} />
            <Route path="/create" element={<CreateJob />} />
            <Route path="/edit/:id" element={<EditJob />} />
            <Route path="/job/:id/runs/:runId" element={<JobRunDetails />} />
          </Routes>
        </main>
      </div>
    </Router>
  )
}

export default App
