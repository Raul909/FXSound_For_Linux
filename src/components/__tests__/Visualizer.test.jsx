import { render, act } from '@testing-library/react';
import { vi, describe, beforeEach, afterEach, it, expect } from 'vitest';
import Visualizer from '../Visualizer';
import { invoke } from '@tauri-apps/api/core';

// Mock Tauri invoke
vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn(),
}));

describe('Visualizer WebAudio Error Handling', () => {
  beforeEach(() => {
    vi.clearAllMocks();

    // Default invoke mock to reject so it falls through to tryWebAudio
    invoke.mockRejectedValue(new Error('Backend not available'));

    // Setup requestAnimationFrame mock
    vi.spyOn(window, 'requestAnimationFrame').mockImplementation(cb => setTimeout(cb, 0));
    vi.spyOn(window, 'cancelAnimationFrame').mockImplementation(id => clearTimeout(id));
  });

  afterEach(() => {
    vi.restoreAllMocks();
  });

  it('handles getDisplayMedia errors gracefully', async () => {
    // Mock navigator.mediaDevices.getDisplayMedia to throw an error
    const error = new Error('Permission denied');
    Object.defineProperty(navigator, 'mediaDevices', {
      value: {
        getDisplayMedia: vi.fn().mockRejectedValue(error),
      },
      writable: true,
    });

    // We want to verify it falls back to idle mode gracefully without crashing

    await act(async () => {
      render(<Visualizer powered={true} />);
      // Wait a bit for the async operations to complete
      await new Promise(resolve => setTimeout(resolve, 100));
    });

    // Verify getDisplayMedia was called
    expect(navigator.mediaDevices.getDisplayMedia).toHaveBeenCalledWith({
      audio: true,
      video: true
    });

    // We can't directly check sourceRef, but we know it should have rendered successfully
    // without crashing, and visually it would fall back to idle mode
  });
});
