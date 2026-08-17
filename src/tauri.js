import { invoke } from "@tauri-apps/api/core";

/**
 * `invoke()` that always returns a promise.
 *
 * Outside a Tauri runtime — `npm run dev` in a plain browser — the injected
 * internals object is missing and `invoke` throws *synchronously*, before it
 * ever returns a promise. A trailing `.catch()` cannot see that, so the error
 * escaped out of the `useEffect` callbacks that called it. Normalising here
 * keeps every call site to a single error path.
 */
export function call(command, args) {
    try {
        return Promise.resolve(invoke(command, args));
    } catch (err) {
        return Promise.reject(err);
    }
}
