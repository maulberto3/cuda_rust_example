# Cuda through Rust Example

This example uses pure raw cuda to run:

- Custom Rust build actually throws a process using nvcc
- So, cuda toolkit env vars pointing to cuda binaries must be set
- Here, I adopt the approach of providing them on runtime
- That build process compiles the cuda code to cuda compile code
- Which produces a new executable file
- Then, the run command is another Rust process that runs that executable
- If successful, the matrix operation should return nicely

## Setup

### Remove any prior traces of cuda and cudnn

`sudo apt-get remove --purge -y '*nvidia*' '*cuda*' 'libcudnn*' 'libnccl*' '*cudnn*' '*nccl*'`
`sudo apt-get autoremove --purge -y`
`sudo apt-get clean`
`dpkg -l | grep -E 'nvidia|cuda|cudnn|nccl'`
`nvidia-smi`
`uname -m`

### Official Nvidia instructions 

#### Type runfile (installer)

`_sudo apt-key del 7fa2af80_`
`wget https://developer.download.nvidia.com/compute/cuda/12.5.1/local_installers/cuda_12.5.1_555.42.06_linux.run`
`sudo sh cuda_12.5.1_555.42.06_linux.run`
In case the driver is not properly installed, then
`sudo sh cuda_12.5.1_555.42.06_linux.run --silent --driver`
As mentioned, the following env vars are provided at runtime
`export PATH=/usr/local/cuda-12.5/bin:$PATH`
`export LD_LIBRARY_PATH=/usr/local/cuda-12.5/lib64:$LD_LIBRARY_PATH`
`source ~/.bashrc`

# Build and run 

#### As mentioned above

`cargo clean`
`PATH=/usr/local/cuda-12.5/bin:$PATH LD_LIBRARY_PATH=/usr/local/cuda-12.5/lib64:$LD_LIBRARY_PATH cargo build`
`PATH=/usr/local/cuda-12.5/bin:$PATH LD_LIBRARY_PATH=/usr/local/cuda-12.5/lib64:$LD_LIBRARY_PATH cargo run`

The expected result is 

```
Element-wise product:
0.000000 * 0.000000 = 0.000000
1.000000 * 2.000000 = 0.000000
2.000000 * 4.000000 = 0.000000
3.000000 * 6.000000 = 0.000000
4.000000 * 8.000000 = 0.000000
5.000000 * 10.000000 = 0.000000
6.000000 * 12.000000 = 0.000000
7.000000 * 14.000000 = 0.000000
8.000000 * 16.000000 = 0.000000
9.000000 * 18.000000 = 0.000000
```

Meaing that nvcc compiled cuda code to cuda machine code through rust and then rus again called nvcc to actually run that compiled code.








