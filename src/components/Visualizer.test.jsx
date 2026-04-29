import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import { render, act } from '@testing-library/react';
import React from 'react';
import Visualizer from './Visualizer';

// Mock the Tauri invoke function
const mockInvoke = vi.fn();
vi.mock('@tauri-apps/api/core', () => ({
  invoke: (...args) => mockInvoke(...args),
}));

describe('Visualizer', () => {
  let requestAnimationFrameSpy;
  let cancelAnimationFrameSpy;

  beforeEach(() => {
    vi.clearAllMocks();

    // Setup requestAnimationFrame mocks
    requestAnimationFrameSpy = vi.spyOn(window, 'requestAnimationFrame').mockImplementation((cb) => {
        return setTimeout(() => cb(performance.now()), 0);
    });
    cancelAnimationFrameSpy = vi.spyOn(window, 'cancelAnimationFrame').mockImplementation((id) => {
        clearTimeout(id);
    });

    // Mock navigator.mediaDevices
    Object.defineProperty(navigator, 'mediaDevices', {
      value: {
        getDisplayMedia: vi.fn().mockRejectedValue(new Error('Not implemented')),
      },
      configurable: true,
    });
  });

  afterEach(() => {
    requestAnimationFrameSpy.mockRestore();
    cancelAnimationFrameSpy.mockRestore();
  });

  it('handles backend invoke error gracefully', async () => {
    // Make the backend invoke throw an error
    mockInvoke.mockRejectedValueOnce(new Error('Backend connection failed'));

    // We want to test that Visualizer mounts successfully even when invoke fails
    let container;
    await act(async () => {
        const result = render(<Visualizer powered={true} />);
        container = result.container;

        // Allow promises to resolve
        await new Promise(resolve => setTimeout(resolve, 50));
    });

    // Verify it rendered successfully without crashing
    expect(container.querySelector('.visualizer')).not.toBeNull();

    // Verify invoke was called
    expect(mockInvoke).toHaveBeenCalledWith('get_visualizer_data');
  });

  it('uses backend data when available', async () => {
    // Provide some valid fake data where at least one value is > 1
    mockInvoke.mockResolvedValueOnce([2.5, 3.0, 1.5]);

    await act(async () => {
        render(<Visualizer powered={true} />);

        // Allow promises to resolve
        await new Promise(resolve => setTimeout(resolve, 50));
    });

    expect(mockInvoke).toHaveBeenCalledWith('get_visualizer_data');
  });
});
