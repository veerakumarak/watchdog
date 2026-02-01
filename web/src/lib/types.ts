
export type ProviderType = 'EmailSmtp' | 'GchatWebhook' | 'SLACK';

export type Channel = {
    id: string;
    name: string;
    providerType: ProviderType;
    configuration: string;
    createdAt?: string;
}

export type JobConfig = {
    appName: string;
    jobName: string;
    schedule: string;
    zoneId: string;
    channel_ids: string;
    enabled: boolean;
    stages: Stage[];
};

export type Stage = {
    name: string;
    start: number | null;
    complete: number | null;
};


export interface GlobalSettings {
    retention_days: number;
    admin_emails: string;
    maintenance_mode: boolean;
    default_timezone: string;
}

// Enums
export type JobRunStatus = 'InProgress' | 'Complete' | 'Failed';
export type JobRunStageStatus = 'Occurred' | 'Failed' | 'Missed';

// Structs
export type JobRunStage = {
    name: string;
    startStatus: JobRunStageStatus | null;
    startDateTime: string | null; // ISO Date String
    completeStatus: JobRunStageStatus | null;
    completeDateTime: string | null; // ISO Date String
}

export type JobRun = {
    id: string; // UUID
    appName: string;
    jobName: string;
    triggeredAt: string;
    status: JobRunStatus;
    stages: JobRunStage[];
    createdAt: string;
    updatedAt: string;
}