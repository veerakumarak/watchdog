export const STATUS = {
    success: "success",
    failure: "failure",
    error: "error",
} as const;

export type STATUS = typeof STATUS[keyof typeof STATUS];

export interface JSend<T> {
    status: STATUS;
    data?: T;
    reasons?: Map<string, string>;
    message?: string;
    code?: number;
}
