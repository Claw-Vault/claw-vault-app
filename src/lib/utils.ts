import { type ClassValue, clsx } from "clsx";
import { twMerge } from "tailwind-merge";
import type { EncryptResponse } from "./api/models/encrypt";
import type { DecryptResponse } from "./api/models/decrypt";
import type { ApiEmpty } from "./api/models/empty";

export function cn(...inputs: ClassValue[]) {
    return twMerge(clsx(inputs));
}

export function isType<T>(value: any, field: string): value is T {
    if (value === null || typeof value !== "object") {
        return false;
    }
    return field in value;
}

export function isEncryptResponse(value: any): value is EncryptResponse {
    return isType<EncryptResponse>(value, "valid_for");
}
export function isDecryptResponse(value: any): value is DecryptResponse {
    return isType<DecryptResponse>(value, "data");
}
export function isApiEmpty(value: any): value is ApiEmpty {
    return isType<ApiEmpty>(value, "status");
}
