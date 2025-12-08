"use client"

import { useState, useEffect } from "react"
import { useParams, useNavigate } from "react-router-dom"
import Timeline from "../components/Timeline"
import "../styles/job-run-details.css"

interface RunLog {
  timestamp: string
  level: "info" | "warning" | "error"
  message: string
}

interface JobRun {
  id: string
  jobName: string
  status: "success" | "failed" | "running"
  startTime: string
  endTime: string
  duration: string
  logs: RunLog[]
}

export default function JobRunDetails() {
  const { id, runId } = useParams()
  const navigate = useNavigate()
  const [run, setRun] = useState<JobRun | null>(null)

  useEffect(() => {
    // Mock data
    const mockRun: JobRun = {
      id: runId || "1",
      jobName: "Database Backup",
      status: "success",
      startTime: "2024-01-15 14:30:00",
      endTime: "2024-01-15 14:45:32",
      duration: "15m 32s",
      logs: [
        { timestamp: "2024-01-15 14:30:00", level: "info", message: "Job started" },
        { timestamp: "2024-01-15 14:30:15", level: "info", message: "Connecting to database..." },
        { timestamp: "2024-01-15 14:30:45", level: "info", message: "Starting backup process" },
        { timestamp: "2024-01-15 14:35:20", level: "info", message: "Backup completed: 2.5GB" },
        { timestamp: "2024-01-15 14:36:00", level: "info", message: "Uploading to S3..." },
        { timestamp: "2024-01-15 14:45:32", level: "info", message: "Job completed successfully" },
      ],
    }
    setRun(mockRun)
  }, [id, runId])

  if (!run) {
    return <div className="loading">Loading...</div>
  }

  return (
    <div className="job-run-details-page">
      <button className="back-button" onClick={() => navigate("/")}>
        ← Back to Jobs
      </button>

      <div className="run-header">
        <div className="header-content">
          <h1>{run.jobName}</h1>
          <p>Run ID: {run.id}</p>
        </div>
        <div className={`status-badge status-${run.status}`}>
          {run.status.charAt(0).toUpperCase() + run.status.slice(1)}
        </div>
      </div>

      <div className="run-stats">
        <div className="stat-card">
          <label>Started</label>
          <p>{run.startTime}</p>
        </div>
        <div className="stat-card">
          <label>Completed</label>
          <p>{run.endTime}</p>
        </div>
        <div className="stat-card">
          <label>Duration</label>
          <p>{run.duration}</p>
        </div>
      </div>

      <div className="logs-section">
        <h2>Execution Logs</h2>
        <Timeline logs={run.logs} />
      </div>
    </div>
  )
}
