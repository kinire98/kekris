export default async function playAudio(audio: string) {
  const audioContext = new AudioContext();
  const response = await fetch(audio); // Load audio file
  const arrayBuffer = await response.arrayBuffer();
  const audioBuffer = await audioContext.decodeAudioData(arrayBuffer);

  function play() {
    const source = audioContext.createBufferSource();
    source.buffer = audioBuffer;
    source.loop = false; // We handle looping manually

    const gainNode = audioContext.createGain();
    gainNode.gain.value = 0.5; // Set initial volume to 50%

    source.connect(gainNode);
    gainNode.connect(audioContext.destination);

    const duration = audioBuffer.duration;
    const fadeOutTime = 3; // 3 seconds fade out at the end

    // Schedule fade-out before the end
    gainNode.gain.setValueAtTime(0.5, duration - fadeOutTime);
    gainNode.gain.linearRampToValueAtTime(0, duration);

    // Restart audio after it ends

    source.start(0);
    source.onended = () => playAudio(audio);
  }

  play();

}
