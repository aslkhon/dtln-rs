const fs = require('node:fs');
const wav = require('node-wav');
const WavEncoder = require('wav-encoder');
const Dtln = require('./dtln.js');

(async () => {
  // 1. Decode
  const buffer = fs.readFileSync('call.wav');
  const { sampleRate, channelData } = wav.decode(buffer);

  // 2. Prepare frames
  const FRAME_SIZE = 512;
  function sliceFrames(data) {
    const frames = [];
    for (let i = 0; i < data.length; i += FRAME_SIZE) {
      const frame = data.subarray(i, i + FRAME_SIZE);
      if (frame.length < FRAME_SIZE) {
        const padded = new Float32Array(FRAME_SIZE);
        padded.set(frame);
        frames.push(padded);
      } else frames.push(frame);
    }
    return frames;
  }
  const leftFrames = sliceFrames(channelData[0]);
  const rightFrames = channelData[1] ? sliceFrames(channelData[1]) : null;

  // 3. Init model
  const handle = Dtln.dtln_create();

  // 4. Denoise
  const outLeft = [];
  const outRight = [];
  for (let i = 0; i < leftFrames.length; i++) {
    const inL = leftFrames[i];
    const outL = new Float32Array(FRAME_SIZE);
    Dtln.dtln_denoise(handle, inL, outL);
    outLeft.push(outL);

    if (rightFrames) {
      const inR = rightFrames[i];
      const outR = new Float32Array(FRAME_SIZE);
      Dtln.dtln_denoise(handle, inR, outR);
      outRight.push(outR);
    }
  }

  // 5. Destroy model
//   Dtln.dtln_destroy(handle);

  // 6. Merge frames
  function merge(frames) {
    const result = new Float32Array(frames.length * FRAME_SIZE);
    frames.forEach((f, i) => result.set(f, i * FRAME_SIZE));
    return result;
  }
  const denoisedLeft = merge(outLeft);
  const denoisedRight = rightFrames ? merge(outRight) : null;

  // 7. Encode & write
  const audioData = {
    sampleRate,
    channelData: rightFrames ? [denoisedLeft, denoisedRight] : [denoisedLeft],
  };
  const wavBuffer = await WavEncoder.encode(audioData);
  fs.writeFileSync('output_wav.wav', Buffer.from(wavBuffer));
})();
