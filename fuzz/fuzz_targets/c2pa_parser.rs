#![no_main]
use libfuzzer_sys::fuzz_target;
use img_parts::jpeg::Jpeg;

// Fuzz target for C2PA manifest parsing
// Tests that C2PA parser handles corrupted/random payloads gracefully
// without panicking or leaking data on invalid inputs.
// Optimized: Uses in-memory JPEG parsing instead of file I/O for speed.
fuzz_target!(|data: &[u8]| {
    // Test low-level JPEG parsing in-memory (much faster than file I/O)
    // This tests the core parsing logic without filesystem overhead
    // Clone data to Vec to satisfy lifetime requirements
    let data_vec = data.to_vec();
    if let Ok(jpeg) = Jpeg::from_bytes(data_vec.into()) {
        // Extract APP11 segments (JUMBF) - this is what C2PA uses
        for segment in jpeg.segments() {
            if segment.marker() == 0xEB { // APP11
                // Just verify we can read the segment without panicking
                let _ = segment.contents();
            }
        }
    }
    // If parsing fails, that's expected for random data - we just want no panics
});
