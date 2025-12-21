import { BrowserRouter, Routes, Route } from "react-router-dom"
import { HomePage } from "./pages/HomePage"
import { CreateJobPage } from "./pages/CreateJobPage"
// import { EditJobPage } from "./pages/EditJobPage"
// import { JobRunDetailsPage } from "./pages/JobRunDetailsPage"

function App() {
    return (
        <BrowserRouter>
            <div className="flex min-h-screen bg-background">
                <Routes>
                    <Route path="/" element={<HomePage />} />
                    <Route path="/jobs/create" element={<CreateJobPage />} />
                    {/*<Route path="/jobs/:id/edit" element={<EditJobPage />} />*/}
                    {/*<Route path="/jobs/:id/runs/:runId" element={<JobRunDetailsPage />} />*/}
                </Routes>
            </div>
        </BrowserRouter>
    )
}

export default App
