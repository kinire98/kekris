import { getRepeatIntervalStored, getStartRepeatIntervalStored } from "../helpers/intervalsStoring";



export function getStartRepeatInterval(): number {
    return getStartRepeatIntervalStored();
}


export function getRepeatInterval(): number {
    return getRepeatIntervalStored();
}
