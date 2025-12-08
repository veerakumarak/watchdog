"use client"

import { useState, useEffect } from "react"
import { Link } from "react-router-dom"
import StatusBadge from "../components/StatusBadge"
import "../styles/jobs-list.css"

interface Job {
  id: string
  name: string
  status: "active" | "inactive" | "error"
  lastRun: string
  nextRun: string
  successRate: number
  description: string
}

export default function JobsList() {
  const [jobs, setJobs] = useState<Job[]>([])
  const [searchTerm, setSearchTerm] = useState("")

  useEffect(() => {
    // Mock data
    const mockJobs: Job[] = [
      {
        id: "1",
        name: "Database Backup",
        status: "active",
        lastRun: "2024-01-15 14:30:00",
        nextRun: "2024-01-16 14:30:00",
        successRate: 98,
        description: "Daily database backup to S3",
      },
      {
        id: "2",
        name: "Cache Refresh",
        status: "active",
        lastRun: "2024-01-15 12:00:00",
        nextRun: "2024-01-15 14:00:00",
        successRate: 100,
        description: "Refresh Redis cache every 2 hours",
      },
      {
        id: "3",
        name: "Report Generation",
        status: "inactive",
        lastRun: "2024-01-14 08:00:00",
        nextRun: "2024-01-16 08:00:00",
        successRate: 95,
        description: "Generate daily analytics report",
      },
      {
        id: "4",
        name: "Cleanup Service",
        status: "error",
        lastRun: "2024-01-15 10:15:00",
        nextRun: "Paused",
        successRate: 87,
        description: "Remove old temporary files",
      },
    ]
    setJobs(mockJobs)
  }, [])

  const filteredJobs = jobs.filter((job) => job.name.toLowerCase().includes(searchTerm.toLowerCase()))

  return (
    <div className="jobs-list-page">
      <div className="page-header">
        <div>
          <h1>Jobs</h1>
          <p>Manage and monitor scheduled jobs</p>
        </div>
        <Link to="/create" className="btn btn-primary">
          + New Job
        </Link>
      </div>

      <div className="search-bar">
        <input
          type="text"
          placeholder="Search jobs..."
          value={searchTerm}
          onChange={(e) => setSearchTerm(e.target.value)}
        />
      </div>

      <div className="jobs-table">
        <table>
          <thead>
            <tr>
              <th>Job Name</th>
              <th>Status</th>
              <th>Last Run</th>
              <th>Next Run</th>
              <th>Success Rate</th>
              <th>Actions</th>
            </tr>
          </thead>
          <tbody>
            {filteredJobs.map((job) => (
              <tr key={job.id}>
                <td>
                  <div className="job-name-cell">
                    <h3>{job.name}</h3>
                    <p>{job.description}</p>
                  </div>
                </td>
                <td>
                  <StatusBadge status={job.status} />
                </td>
                <td className="timestamp">{job.lastRun}</td>
                <td className="timestamp">{job.nextRun}</td>
                <td>
                  <div className="success-rate">
                    <div className="rate-bar">
                      <div className="rate-fill" style={{ width: `${job.successRate}%` }}></div>
                    </div>
                    <span>{job.successRate}%</span>
                  </div>
                </td>
                <td>
                  <div className="actions">
                    <Link to={`/job/${job.id}/runs/latest`} className="link-btn">
                      View Runs
                    </Link>
                    <Link to={`/edit/${job.id}`} className="link-btn">
                      Edit
                    </Link>
                  </div>
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </div>
  )
}
