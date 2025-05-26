import { getRepeatIntervalStored, getStartRepeatIntervalStored } from "../helpers/intervalsStoring";

/**
 * Gets the start repeat interval from storage.
 * @returns The start repeat interval.
 */
export function getStartRepeatInterval(): number {
    return getStartRepeatIntervalStored();
}

/**
 * Gets the repeat interval from storage.
 * @returns The repeat interval.
 */
export function getRepeatInterval(): number {
    return getRepeatIntervalStored();
}
