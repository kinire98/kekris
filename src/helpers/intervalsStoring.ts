export const minStartRepeatInterval: number = 17;
export const maxStartRepeatInterval: number = 333;
export const minRepeatInterval: number = 0;
export const maxRepeatInterval: number = 83;

export const defaultStartRepeatInterval: number = 167;
export const defaultRepeatInterval: number = 33;

const startKey: string = "start_repeat_key_interval";
const repeatKey: string = "repeat_key_interval";


export function getStartRepeatIntervalStored(): number {
    let value = localStorage.getItem(startKey)
    if (value == null) {
        return defaultStartRepeatInterval;
    }
    return parseInt(value) ?? defaultStartRepeatInterval;
}
export function setStartRepeatInterval(interval: number) {
    localStorage.setItem(startKey, interval.toString());
}
export function getRepeatIntervalStored(): number {
    let value = localStorage.getItem(repeatKey);
    if (value == null) {
        return defaultRepeatInterval;
    }

    return parseInt(value) ?? defaultRepeatInterval;
}
export function setRepeatInterval(interval: number) {
    localStorage.setItem(repeatKey, interval.toString());
}