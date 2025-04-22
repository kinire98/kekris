export function formatSecondsToHHMMSS(secondsMark: number): string {
    const hours = Math.floor(secondsMark / 3600);
    const minutes = Math.floor((secondsMark % 3600) / 60);
    const seconds = secondsMark % 60;

    const pad = (n: number) => n.toString().padStart(2, "0");

    return `${pad(hours)}:${pad(minutes)}:${pad(seconds)}`;
}