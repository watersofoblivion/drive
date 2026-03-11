# Geometry Notes

* Vector Math
  * Tangents
    * Using previous and next points to approximate tangent at a single point is a good algorithm
    * 2-argument arctangent with previous and next points will give an angle of the tangent: `atan2(y[i + 1] - y[i - 1], x[i + 1] - x[i - 1])`
  * Normals can be computed from the tangents
  * Basis vectors can be computed from normalizing the tangent and normal vectors to for a basis as that point.
* For rollups,
  * There's three important notions
    * Centroid, which is the geometric average.  It is easy to compute and can be parallelized on the GPU
    * Geometric median, which is difficult to compute exactly but has iterative approximations (that likely can be GPU accelerated).  This is not guaranteed to be a point in the dataset, and the algorithm actually has to be adapted to handle when it *is* a point in the dataset.
    * Medoid, which is like Geometric median but is guaranteed to be an element in the set
  * Together, it would look like:
    * Geometric median is the "typical" instance from a set of points
    * Medoid is the instance that is closest to typical
    * A circle with center at the centroid and a radius equal to the largest distance from the centroid to any element in the set contains all the points in the set.
* GPU
  * All (or nearly all) of the data processing can be done on the GPU using Compute shaders
    * Data can be smoothed on a GPU
    * Similarly, derivatives can be computed as above on a GPU
    * Cubic bezier curves for interpolation can be computed on the GPU
    * Centroid and Medoid have efficient parallel implementations on the GPU
    * Geometric mean has an iterative algorithm that should be able to be implemented using Compute shaders
    * From there, almost all the math is vector math which the GPU is built for
  * There are a couple of things that don't translate well to GPUs or are easier to do on the CPU.
    * R* trees don't play nice
    * State machine to recognize laps from zones doesn't play nice
    * Fortunately, the first of these directly decorates raw data, the second decorates the output of the first, and the decoration can be done in a single pass.  This means we can even do it on the fly while building the data to send to the GPU.
    * Things like computing camera keyframes, minimum 
  * Data Size
    * For telemetry calculations
      * Assume 10 MB of data per session
        * From Dragy Lap, the uncompressed VBO (text) file for each session is 1.0 - 1.5 MB.
          * The in-memory version of this should be smaller, but with all the derived data it may grow in size.
          * Will measure this once some code is written
        * Assuming an order-of-magnitude increase in data size from raw text form to decorated in-memory is likely absurd, but it gives us a good worst-case lower bound
      * Assume 8GB of RAM available to the GPU
        * Low-end dedicated GPUs have 8GB
        * Integrated GPUs use system RAM, typically up to 50% of system RAM.  It's safe to assume 16GB RAM for a typical laptop (especially if owned by someone who does motorsport)
        * Like the data size assumptions, this is just to compute a worst-case lower bound
      * That means we can hold 8192 MB / 10 MB = 819 fully decorated sessions in RAM on the GPU at once.
      * In real-world terms, this is *plenty*
        * If the user is doing an insane 100 track days a year, that's ~8 years worth of data
        * If the user is doing a more reasonable 8 track days a year, that's ~100 years worth of data
  * Design goal:
    * When adding data:
      * Load all raw data (either from original source files or from some internal database)
      * Perform pre-processing on raw data
        * Geospatial functions like determining track, corners/zones/sectors, laps, etc.
        * Might be doable in one pass
          * If it is, it can be part of the import process
          * Should be part of the core data model, not implemented ad-hoc in each importer
          * Will likely require data points to be added time-ordered.  This typically should not be an issue.
      * Copy *all* raw lap data to the GPU *once*
      * Perform all the math locally on the GPU using Compute Shaders (Vulkano)
      * Copy *all* computed data back to the CPU *once*
      * Merge the raw and computed data
      * Perform post-processing on merged data
        * Camera keyframes, etc.
      * Persist *all* data
        * Likely into an internal database
        * Persist raw and computed data (including things like camera keyframes)
        * Persist input files, including checksums so we don't have to recompute from files that didn't change
    * When visualizing data
      * Load *all* data to the GPU *once*:
        * Merged raw + computed data
        * Track maps and satellite imagery
        * Misc (car, vectors, highlights, text, etc.)
      * Perform all visualization on data already on the GPU
    * This is just a conceptual goal.
      * May break things down further based on implementation details discovered
      * May break things down further so they can be re-usable components
* Import design
  * `data::model*`: The internal data model
  * `data::store::*`: An abstract data store
    * Provides traits for storing the data
    * Supports multiple backends (local files, local persistent BTree, PostgreSQL, DynamoDB, etc.)
    * `data::store::<backend>::*`: Individual backends for the data store
  * `data::format::*`: Parsers (and generators) for supported data formats
    * These just parse the files into ASTs that have pointers into the raw source
    * They don't do any data manipulation/conversion themselves
    * They present APIs for extracting the data in a uniform way (possibly a trait)
    * For example
      * VBO is text, so numbers are strings.
      * In the AST, these are just pointers to the strings in the file
  * `device::*` Adapters to interpret the data from a particular format
    * These only convert data lazily on-demand.
    * They are primarily for codifying the mapping that data files generated by that device to the internal format.
    * For example,
        * A DragyLap device takes a VBO AST
        * When asked to convert that data into the internal format
          * It parses the individual values stored in the VBO format
          * It converts them into values in the internal data model
          * It returns the data collected into the equivalent (i.e., session) internal model object
        * The DragyLap device stores no actual data, just converts a VBO generated by DragyLap into the internal data model
