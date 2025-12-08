import "../styles/timeline.css"

interface LogEntry {
  timestamp: string
  level: "info" | "warning" | "error"
  message: string
}

interface Props {
  logs: LogEntry[]
}

export default function Timeline({ logs }: Props) {
  return (
    <div className="timeline">
      {logs.map((log, index) => (
        <div key={index} className={`timeline-item level-${log.level}`}>
          <div className="timeline-marker"></div>
          <div className="timeline-content">
            <div className="timeline-header">
              <span className={`log-level log-${log.level}`}>{log.level.toUpperCase()}</span>
              <time>{log.timestamp}</time>
            </div>
            <p className="log-message">{log.message}</p>
          </div>
        </div>
      ))}
    </div>
  )
}
