import {Result} from "ts-fp-utils/dist/result";
import {ApiFailure} from "ts-fp-utils/dist/failures/ApiFailure";
import {AuthFailure} from "ts-fp-utils/dist/failures/AuthFailure";
import type {JSend} from "@/lib/jsend";
import {EXTERNAL_API_BASE_URL} from "@/api/common.ts";
import camelcaseKeys from "camelcase-keys";
import snakecaseKeys from "snakecase-keys";

export type Empty = {
    [key: string]: unknown;
};

type TPayload = Record<string, unknown> | readonly Record<string, unknown>[];

async function getBody<T>(res: Response): Promise<Result<JSend<T>>> {
    return Result.of(async () => {
        const contentType = res.headers.get('Content-Type');
        console.log(contentType);
        const isJson = contentType?.includes('application/json');
        if (!isJson) {
            console.log(await res.text());
            throw new ApiFailure("non json format is not supported, body: " + await res.text());
        }
        return await res.json();
    });
}

async function execute<T>(method: () => Promise<Response>): Promise<Result<T>> {
    return Result.of(async () => {
        const res = await method();
        console.log(res.ok);
        console.log(res.status);
        const bodyResult = await getBody<T>(res);
        if (bodyResult.isFailure()) {
            throw bodyResult.failure();
        }
        const bodyJsend = bodyResult.get();
        console.log(bodyJsend);
        if (!res.ok) {
            let message = "";
            // if (body.status === STATUS.error) {
            //     message = body.message || '';
            // } else {
            message = JSON.stringify(bodyJsend);
            // }
            if (res.status === 401) {
                throw new AuthFailure(message);
            }
            throw new ApiFailure(message);
        }
        return camelcaseKeys(bodyJsend.data as TPayload, { deep: true }) as T;
    });
}

export async function get<TResponse>(path: string): Promise<Result<TResponse>> {
    const get = async () => {
        return await fetch(EXTERNAL_API_BASE_URL + path, {
                method: 'GET',
                headers: getAndDelHeaders(),
            }
        )};

    return execute<TResponse>(get);
}

export async function post<TRequest extends TPayload, TResponse>(path: string, payload: TRequest): Promise<Result<TResponse>> {
    const post = async () => {
        return await fetch(EXTERNAL_API_BASE_URL + path, {
            method: 'POST',
            headers: postAndPutHeaders(),
            body: JSON.stringify(snakecaseKeys(payload, { deep: true }))
        });
    };

    return execute<TResponse>(post);
}

export async function put<TRequest extends Record<string, unknown> | ReadonlyArray<Record<string, unknown>>, TResponse>(path: string, payload: TRequest): Promise<Result<TResponse>> {
    const put = async () => {
        return await fetch(EXTERNAL_API_BASE_URL + path, {
            method: 'PUT',
            headers: postAndPutHeaders(),
            body: JSON.stringify(snakecaseKeys(payload, { deep: true }))
        });
    };

    return execute<TResponse>(put);
}

export async function del<TResponse>(path: string): Promise<Result<TResponse>> {
    const del = async () => {
        return await fetch(EXTERNAL_API_BASE_URL + path, {
                method: 'DELETE',
                headers: getAndDelHeaders()
            }
        )};

    return execute<TResponse>(del);
}

function postAndPutHeaders(): Record<string, string> {
    return {
        "Content-Type": "application/json",
        'Accept': 'application/json',
    };
}
function getAndDelHeaders(): Record<string, string> {
    return {
        'Accept': 'application/json',
    };
}
