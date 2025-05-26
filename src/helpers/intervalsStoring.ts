export const minStartRepeatInterval: number = 17;
export const maxStartRepeatInterval: number = 333;
export const minRepeatInterval: number = 0;
export const maxRepeatInterval: number = 83;

export const defaultStartRepeatInterval: number = 167;
export const defaultRepeatInterval: number = 33;

const startKey: string = "start_repeat_key_interval";
const repeatKey: string = "repeat_key_interval";

/**
 * Gets the stored start repeat interval from local storage.
 * @returns The stored start repeat interval, or the default value if not found.
 */
export function getStartRepeatIntervalStored(): number {
    let value = localStorage.getItem(startKey)
    if (value == null) {
        return defaultStartRepeatInterval;
    }
    return parseInt(value) ?? defaultStartRepeatInterval;
}

/**
 * Sets the start repeat interval in local storage.
 * @param interval The start repeat interval to set.
 */
export function setStartRepeatInterval(interval: number) {
    localStorage.setItem(startKey, interval.toString());
}

/**
 * Gets the stored repeat interval from local storage.
 * @returns The stored repeat interval, or the default value if not found.
 */
export function getRepeatIntervalStored(): number {
    let value = localStorage.getItem(repeatKey);
    if (value == null) {
        return defaultRepeatInterval;
    }

    return parseInt(value) ?? defaultRepeatInterval;
}

/**
 * Sets the repeat interval in local storage.
 * @param interval The repeat interval to set.
 */
export function setRepeatInterval(interval: number) {
    localStorage.setItem(repeatKey, interval.toString());
}