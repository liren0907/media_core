# Solution Pattern: Unified Media Pipeline

This document defines the architectural vision for the **Unified Media Pipeline** in `media_core`. Moving beyond simple utility scripts, this architecture adopts a "Blackboard Pattern" to integrate Metadata, Analysis, Annotation, and Processing modules into a cohesive, intelligent workflow.

## 1. The Core Philosophy

Instead of siloed pipelines (one for metadata, one for analysis), we use a **Single Unified Pipeline** driven by a shared context.

*   **From**: "Run Metadata tool -> Get JSON -> Run Analysis tool -> Get JSON -> Run Processing tool"
*   **To**: "Load Media -> Pipeline executes steps sequentially -> Steps share knowledge via Context"

## 2. Architecture Overview

### 2.1. The Unified Context (`MediaContext`)

The `MediaContext` is the "Brain" or "Blackboard" of the system. It accumulates knowledge as the pipeline progresses.

```rust
pub struct MediaContext {
    // 1. The Subject (Input)
    pub source: MediaSource,           // File path or RTSP URL
    
    // 2. The Knowledge (Accumulated Data)
    pub metadata: Option<MediaMetadata>,   // Filled by Metadata Module
    pub analysis: Option<AnalysisReport>,  // Filled by Analysis Module (Motion, Objects)
    pub stream_info: Option<StreamInfo>,   // Filled by RTSP Module
    
    // 3. The Plan (Execution Instructions)
    // Analysis steps can write "Tasks" here for the Processor to execute
    pub processing_tasks: Vec<Task>,       // e.g., "Clip 00:10-00:20", "Draw Box at x,y"
    
    // 4. Shared Resources (Cache)
    // Optimized sharing of heavy objects (OpenCV VideoCapture, FFmpeg Context)
    resources: ResourceCache, 
}
```

### 2.2. The Modular Adapters

We do not rewrite existing modules. Instead, we create **Pipeline Adapters** that wrap their logic into standard steps.

| Module | Adapter Step | Input (Read from Context) | Output (Write to Context) |
| :--- | :--- | :--- | :--- |
| **Metadata** | `ExtractBasicInfo` | `source` | `context.metadata` |
| **Analysis** | `DetectMotion` | `source`, `metadata` (FPS) | `context.analysis` (Events) |
| **Annotation** | `DrawOverlays` | `source`, `analysis` (Events) | `context.processing_tasks` (Render ops) |
| **Process** | `SmartTranscode` | `source`, `processing_tasks` | File Output |

## 3. Implementation Roadmap

### Phase 1: The Infrastructure (Foundation)
*   **Goal**: Establish the `src/pipeline` core and the `MediaContext`.
*   **Tasks**:
    1.  Define the generic `Pipeline<T>` and `Step<T>` traits (Done).
    2.  Implement `MediaContext` with resource caching (Lazy Loading `VideoCapture`).
    3.  Create the `MediaSource` enum (File vs Stream).

### Phase 2: Knowledge Integration (Metadata & Analysis)
*   **Goal**: Pipeline can "Understand" the media.
*   **Tasks**:
    1.  **Metadata Adapter**: Implement `PipelineStep` for `src/metadata`. It should populate `context.metadata`.
    2.  **Analysis Adapter**: Implement `PipelineStep` for `src/analysis`. It should read `context.metadata` to configure itself (e.g., set thresholds based on resolution) and populate `context.analysis`.

### Phase 3: Action & Output (Annotation & Process)
*   **Goal**: Pipeline can "Act" on the media based on understanding.
*   **Tasks**:
    1.  **Smart Processing**: Create steps that generate processing tasks. Example: "If motion detected > 50%, generate a clip task".
    2.  **Processor Adapter**: Implement a step that executes the `processing_tasks` using `src/process`.

## 4. Usage Example: The "Smart Security Camera" Workflow

This example demonstrates the power of the Unified Pipeline. It combines metadata extraction, motion analysis, annotation, and conditional processing in one cohesive flow.

```rust
// A "Smart Security" Workflow
let pipeline = MediaPipeline::builder()
    // Step 1: Ingest & Understand
    .add(metadata::steps::ExtractCoreInfo::new())      // Get Duration, Resolution
    .add(metadata::steps::ExtractVideoInfo::new())     // Get FPS, Codec
    
    // Step 2: Analyze (Dependent on Step 1)
    .add(analysis::steps::DetectMotion::new()          // Use FPS from Step 1
        .sensitivity(0.8)
        .save_events_to_context())                     // Write to context.analysis
    
    // Step 3: Visualizer (Dependent on Step 2)
    .add(annotation::steps::PrepareOverlays::new()     // Read motion events
        .draw_bounding_boxes()                         // Generate "Draw Task"
        .add_timestamp())
    
    // Step 4: Act (Dependent on Step 2 & 3)
    .add(process::steps::SmartClipper::new()           // Read motion events
        .padding_seconds(5.0)                          // Clip only motion segments
        .apply_overlays()                              // Apply visualizer tasks
        .output_format("mp4"))
    .build();

// Execute
let context = pipeline.execute(MediaSource::File("security_feed.mp4"))?;

// Review Results
println!("Processed {} motion events.", context.analysis.unwrap().events.len());
```

## 5. Benefits

1.  **Data Synergy**: Analysis algorithms can use Metadata (like Bitrate or Color Space) to auto-tune their parameters without user intervention.
2.  **Resource Efficiency**: The input video is opened **once** and shared across Metadata, Analysis, and Thumbnail generation.
3.  **Declarative Workflows**: Complex business logic ("Only record when motion is detected") becomes a simple configuration of pipeline steps.
4.  **Extensibility**: Adding a new AI model (e.g., "Face Detection") is just adding one more Step struct.
