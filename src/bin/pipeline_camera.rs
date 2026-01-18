use media_core::camera::pipeline::CaptureFrame;
use media_core::pipeline::{MediaContext, Pipeline};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("===========================================");
    println!("       Camera Pipeline Example");
    println!("===========================================\n");

    // 1. Initialize Context
    // Use camera ID 0 (default webcam)
    // In a real application, you might want to list available cameras first
    println!("Initializing camera source (ID: 0)...");
    let context = MediaContext::from_camera(0);

    // 2. Build Pipeline
    // Add the CaptureFrame step which will:
    // - Lazy-load the OpenCV video capture
    // - Capture a single frame
    // - Encode it to Base64
    // - Store it in context.extracted_frames
    let pipeline = Pipeline::new().add_node(CaptureFrame);

    // 3. Execute Pipeline
    println!("Executing pipeline...");
    let result_context = pipeline.execute(context)?;

    // 4. Inspect Results
    let frames = &result_context.extracted_frames;
    println!("\n✅ Pipeline Execution Completed");
    println!("   Captured Frames: {}", frames.len());

    if let Some(frame) = frames.first() {
        println!(
            "   Frame #{} Data Size: {} bytes",
            frame.index,
            frame.data.len()
        );
        // Truncate output for readability
        let display_len = std::cmp::min(50, frame.data.len());
        println!("   Frame Snippet: {}...", &frame.data[..display_len]);
    } else {
        println!("   ⚠️ No frames captured.");
    }

    Ok(())
}
