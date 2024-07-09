
## remove any prior traces of cuda and cudnn

sudo apt-get remove --purge -y '*nvidia*' '*cuda*' 'libcudnn*' 'libnccl*' '*cudnn*' '*nccl*'
sudo apt-get autoremove --purge -y
sudo apt-get clean
dpkg -l | grep -E 'nvidia|cuda|cudnn|nccl'

nvidia-smi
uname -m

# Official Nvidia instructions type deb (local)

<!-- wget https://developer.download.nvidia.com/compute/cuda/repos/wsl-ubuntu/x86_64/cuda-wsl-ubuntu.pin
sudo mv cuda-wsl-ubuntu.pin /etc/apt/preferences.d/cuda-repository-pin-600
wget https://developer.download.nvidia.com/compute/cuda/12.5.1/local_installers/cuda-repo-wsl-ubuntu-12-5-local_12.5.1-1_amd64.deb
sudo dpkg -i cuda-repo-wsl-ubuntu-12-5-local_12.5.1-1_amd64.deb
sudo cp /var/cuda-repo-wsl-ubuntu-12-5-local/cuda-*-keyring.gpg /usr/share/keyrings/
sudo apt-get update
sudo apt-get -y install cuda-toolkit-12-5 -->

Those official instructions don't work as expected...

Trying these other ones

# Official Nvidia instructions type runfile (installer)

<!-- sudo apt-key del 7fa2af80 -->
wget https://developer.download.nvidia.com/compute/cuda/12.5.1/local_installers/cuda_12.5.1_555.42.06_linux.run
sudo sh cuda_12.5.1_555.42.06_linux.run
<!-- In case the driver is not properly installed, then -->
sudo sh cuda_12.5.1_555.42.06_linux.run --silent --driver
export PATH=/usr/local/cuda-12.5/bin:$PATH
export LD_LIBRARY_PATH=/usr/local/cuda-12.5/lib64:$LD_LIBRARY_PATH
source ~/.bashrc

# Hit the example for a working cuda script
cargo clean
PATH=/usr/local/cuda-12.5/bin:$PATH LD_LIBRARY_PATH=/usr/local/cuda-12.5/lib64:$LD_LIBRARY_PATH cargo build
PATH=/usr/local/cuda-12.5/bin:$PATH LD_LIBRARY_PATH=/usr/local/cuda-12.5/lib64:$LD_LIBRARY_PATH cargo run

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

Meaing that nvcc compiled cuda code to cuda machine code through rust and then it called nvcc to catually run that compiled code also through rust.








