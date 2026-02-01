import { CheckCircle2, XCircle, MinusCircle, Clock, PlayCircle } from "lucide-react";
import type { JobRunStage, JobRunStageStatus } from '@/lib/types';
import { format } from 'date-fns';

const StatusIcon = ({ status }: { status: JobRunStageStatus | null }) => {
    if (!status) return <Clock className="w-4 h-4 text-slate-300" />; // Pending/Null
    switch (status) {
        case 'Occurred': return <CheckCircle2 className="w-4 h-4 text-emerald-600" />;
        case 'Failed': return <XCircle className="w-4 h-4 text-red-600" />;
        case 'Missed': return <MinusCircle className="w-4 h-4 text-amber-500" />;
    }
};

const StatusBadge = ({ label, status, time }: { label: string, status: JobRunStageStatus | null, time: string | null }) => {
    if (!status && !time) return null;

    const colorClass =
        status === 'Occurred' ? 'bg-emerald-50 text-emerald-700 border-emerald-200' :
            status === 'Failed' ? 'bg-red-50 text-red-700 border-red-200' :
                status === 'Missed' ? 'bg-amber-50 text-amber-700 border-amber-200' :
                    'bg-slate-100 text-slate-600';

    return (
        <div className={`flex items-center gap-2 text-xs border rounded px-2 py-1 ${colorClass}`}>
            <span className="font-bold uppercase tracking-wider text-[10px]">{label}</span>
            {time && <span className="font-mono">{format(new Date(time), 'HH:mm:ss')}</span>}
            <StatusIcon status={status} />
        </div>
    )
}

const RunTimeline = ({ stages }: { stages: JobRunStage[] }) => {
    return (
        <div className="relative space-y-0 pl-4 py-2">
            {stages.map((stage, idx) => (
                <div key={idx} className="relative pl-6 pb-6 last:pb-0 group">

                    {/* Vertical Connector Line */}
                    {idx !== stages.length - 1 && (
                        <div className="absolute left-[5px] top-2 h-full w-[2px] bg-slate-200 group-last:hidden" />
                    )}

                    {/* Node Dot */}
                    <div className="absolute left-[-4px] top-1.5 h-5 w-5 rounded-full border-2 border-white bg-slate-200 z-10 flex items-center justify-center">
                        <div className="h-2 w-2 rounded-full bg-slate-400" />
                    </div>

                    <div className="flex flex-col gap-2">
                        <h4 className="text-sm font-semibold text-slate-800">{stage.name}</h4>

                        {/* Events Grid */}
                        <div className="flex flex-wrap gap-2">
                            {/* Start Event */}
                            <StatusBadge
                                label="Start"
                                status={stage.startStatus}
                                time={stage.startDateTime}
                            />

                            {/* Complete Event */}
                            <StatusBadge
                                label="End"
                                status={stage.completeStatus}
                                time={stage.completeDateTime}
                            />
                        </div>

                        {/* Logic: If Start Occurred but End Failed/Missed */}
                        {stage.startStatus === 'Occurred' && !stage.completeStatus && (
                            <span className="text-[10px] text-blue-600 animate-pulse flex items-center gap-1">
                    <PlayCircle className="w-3 h-3"/> Stage in progress...
                 </span>
                        )}
                    </div>
                </div>
            ))}
        </div>
    );
};

export default RunTimeline;