import numpy as np
import soundfile as sf
import dtln_rs


def slice_frames(data, frame_size):
    """Slice audio data into frames of specified size"""
    frames = []
    for i in range(0, len(data), frame_size):
        frame = data[i : i + frame_size]
        if len(frame) < frame_size:
            # Pad with zeros if frame is too short
            padded = np.zeros(frame_size, dtype=np.float32)
            padded[: len(frame)] = frame
            frames.append(padded)
        else:
            frames.append(frame)
    return frames


def merge_frames(frames, frame_size):
    """Merge frames back into a single array"""
    result = np.zeros(len(frames) * frame_size, dtype=np.float32)
    for i, frame in enumerate(frames):
        result[i * frame_size : (i + 1) * frame_size] = frame
    return result


def denoise_audio(input_file, output_file, frame_size=512):
    """Denoise audio file using DTLN"""

    # 1. Read audio file
    audio_data, sample_rate = sf.read(input_file, dtype="float32")

    # Handle mono/stereo
    if len(audio_data.shape) == 1:
        # Mono audio
        left_channel = audio_data
        right_channel = None
    else:
        # Stereo audio
        left_channel = audio_data[:, 0]
        right_channel = audio_data[:, 1] if audio_data.shape[1] > 1 else None

    # 2. Prepare frames
    left_frames = slice_frames(left_channel, frame_size)
    right_frames = (
        slice_frames(right_channel, frame_size) if right_channel is not None else None
    )

    # 3. Initialize DTLN processor
    processor = dtln_rs.DtlnProcessor()

    # 4. Denoise frames
    denoised_left_frames = []
    denoised_right_frames = []

    print(f"Processing {len(left_frames)} frames...")

    for i, frame in enumerate(left_frames):
        # Process left channel
        denoised_samples, processor_starved = processor.denoise(frame.tolist())
        denoised_left_frames.append(np.array(denoised_samples, dtype=np.float32))

        # Process right channel if it exists
        if right_frames:
            denoised_samples, _ = processor.denoise(right_frames[i].tolist())
            denoised_right_frames.append(np.array(denoised_samples, dtype=np.float32))

        # Progress indicator
        if i % 100 == 0:
            print(f"Processed {i}/{len(left_frames)} frames")

    # 5. Stop processor
    processor.stop()

    # 6. Merge frames back
    denoised_left = merge_frames(denoised_left_frames, frame_size)
    denoised_right = (
        merge_frames(denoised_right_frames, frame_size) if right_frames else None
    )

    # 7. Prepare output data
    if denoised_right is not None:
        # Stereo output
        output_data = np.column_stack((denoised_left, denoised_right))
    else:
        # Mono output
        output_data = denoised_left

    # 8. Write output file
    sf.write(output_file, output_data, sample_rate)
    print(f"Denoised audio saved to {output_file}")


# Example usage
if __name__ == "__main__":
    denoise_audio("call.wav", "output.wav", frame_size=160)
