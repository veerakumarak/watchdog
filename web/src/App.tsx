import {BrowserRouter, Routes, Route, Navigate} from "react-router-dom"
import GlobalSettingsPage from "@/pages/GlobalSettingsPage.tsx";
import AppLayout from "@/AppLayout.tsx";
import {JobConfigPage} from "@/pages/JobConfigPage.tsx";
import ChannelsPage from "@/pages/ChannelsPage.tsx";
import JobRunsPage from "@/pages/JobRunPage.tsx";
import {Toaster} from "sonner";

function App() {
    return (
        <BrowserRouter>
            <div className="flex min-h-screen bg-background">
                <Routes>
                    {/* Wrap everything in the AppLayout */}
                    <Route element={<AppLayout />}>

                        {/* Redirect root "/" to "/jobs" */}
                        <Route path="/" element={<Navigate to="/jobs" replace />} />

                        {/* Define the Pages */}
                        <Route path="/jobs" element={<JobConfigPage />} />
                        <Route path="/channels" element={<ChannelsPage />} />
                        <Route path="/settings" element={<GlobalSettingsPage />} />
                        <Route path="/history" element={<JobRunsPage />} />
                    </Route>
                </Routes>
            </div>
            <Toaster /> {/* Add this here */}
        </BrowserRouter>
    )
}

export default App
