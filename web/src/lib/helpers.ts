import type {JobConfig} from "@/lib/types.ts";

export const getId = (jobConfig: JobConfig) => {
    return jobConfig.app_name + '-' + jobConfig.job_name;
}