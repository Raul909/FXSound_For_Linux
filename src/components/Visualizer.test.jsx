import { render } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach } from 'vitest';
import Visualizer from './Visualizer';

// Mock Tauri invoke
vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn().mockRejectedValue(new Error('Backend not available'))
}));

describe('Visualizer tryWebAudio error handling', () => {
    beforeEach(() => {
        vi.restoreAllMocks();

        // Mock requestAnimationFrame and cancelAnimationFrame
        window.requestAnimationFrame = vi.fn((cb) => setTimeout(cb, 16));
        window.cancelAnimationFrame = vi.fn((id) => clearTimeout(id));
    });

    it('gracefully handles navigator.mediaDevices.getDisplayMedia error and falls back', async () => {
        // Mock navigator.mediaDevices.getDisplayMedia to throw an error
        const mockGetDisplayMedia = vi.fn().mockRejectedValue(new Error('Permission denied'));
        Object.defineProperty(navigator, 'mediaDevices', {
            value: {
                getDisplayMedia: mockGetDisplayMedia,
            },
            configurable: true,
            writable: true
        });

        const { container } = render(<Visualizer powered={true} />);

        // Wait a small amount for the async init to run through all strategies
        await new Promise(resolve => setTimeout(resolve, 150));

        // It should have called getDisplayMedia
        expect(mockGetDisplayMedia).toHaveBeenCalled();

        // It shouldn't crash and should still render bars
        const bars = container.querySelectorAll('.visualizer__bar');
        expect(bars.length).toBeGreaterThan(0);

        // The test passes if it didn't throw an unhandled promise rejection
        // and rendered successfully.
    });
});
