import "../styles/status-badge.css"

interface Props {
  status: "active" | "inactive" | "error" | "success" | "failed" | "running"
}

export default function StatusBadge({ status }: Props) {
  const statusConfig = {
    active: { label: "Active", color: "success" },
    inactive: { label: "Inactive", color: "default" },
    error: { label: "Error", color: "error" },
    success: { label: "Success", color: "success" },
    failed: { label: "Failed", color: "error" },
    running: { label: "Running", color: "info" },
  }

  const config = statusConfig[status]

  return (
    <div className={`status-badge status-${config.color}`}>
      <span className="status-dot"></span>
      {config.label}
    </div>
  )
}
