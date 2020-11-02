# Anodize: Language-Level Guarantees for Mixed Criticality Systems

## Running example mixed criticality system

The visual odometry portion of the system is largely unchanged code from [this github repository](https://github.com/mpizenberg/visual-odometry-rs). If any of the below doesn't make sense, there are more links/instructions in that github repository -- although, you should be able to get everything running just following these instructions.

### Dataset
The visual odometry portion relies on an RGBD SLAM dataset. You can download the [TUM datasets here](https://vision.in.tum.de/data/datasets/rgbd-dataset/download) or use your own. Regardless of what dataset you choose, you'll need to provide an *associations file*. The TUM datasets do not provide their own associations file, so you will need to convert their dataset.

For TUM datasets, you can use [their script for associating color and depth images](https://vision.in.tum.de/data/datasets/rgbd-dataset/tools) to convert their dataset to an associations file. An example command for that script looks like:

``
python associate.py $DATASET/depth.txt $DATASET/rgb.txt > $DATASET/associations.txt
``

### Configuration file
You should include a file ``config.json`` located in the ``system`` directory that looks similar to this:

``
{
    "freiberg_type": "fr1",
    "path_to_associations": "/full/path/to/dataset/associations.txt"
}
``

If you are using the TUM dataset, ``freiberg_type`` should be ``fr1``, ``fr2``, ``fr3``, or ``icl``. ``path_to_associations`` should be the *full* filepath to your associations file that you generated.

### Running
To run the example MC system:

```
cd system/
cargo run --release
```
