import type {JobConfig} from "@/lib/types";

export const getId = (jobConfig: JobConfig) => {
    return jobConfig.appName + '-' + jobConfig.jobName;
}