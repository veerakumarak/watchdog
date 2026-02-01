// import type { JobConfig } from '@/lib/types.ts';
// import type {JSend} from "@/lib/jsend.ts";
// import {API_BASE} from "@/api/common.ts";
//
//
// type JobsListResponse = {
//     jobConfigs: JobConfig[];
// }
// export const JobService = {
//     // GET: List all jobs
//     // getAll: async (): Promise<JobConfig[]> => {
//     //     const res = await fetch(`${API_BASE}/job-configs`);
//     //     let result = await handleResponse<JSend<JobsListResponse>>(res);
//     //     return result.data?.jobConfigs;
//     // },
//
//     // POST: Create a new job
//     create: async (data: JobConfig): Promise<JobConfig> => {
//         const res = await fetch(`${API_BASE}/job-configs`, {
//             method: 'POST',
//             headers: { 'Content-Type': 'application/json' },
//             body: JSON.stringify(data),
//         });
//         return handleResponse(res);
//     },
//
//     // PUT: Update existing job
//     // Route: /jobs/:app_name/:job_name
//     update: async (data: JobConfig): Promise<JobConfig> => {
//         // We use the original app/job name from the data to find the resource
//         const url = `${API_BASE}/job-configs/${data.app_name}/${data.job_name}`;
//
//         const res = await fetch(url, {
//             method: 'PUT',
//             headers: { 'Content-Type': 'application/json' },
//             body: JSON.stringify(data),
//         });
//         return handleResponse(res);
//     },
//
//     // DELETE: Remove a job
//     delete: async (app_name: string, job_name: string): Promise<void> => {
//         const res = await fetch(`${API_BASE}/job-configs/${app_name}/${job_name}`, {
//             method: 'DELETE',
//         });
//         if (!res.ok) throw new Error("Failed to delete job");
//     }
// };