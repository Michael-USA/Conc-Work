#include <cuda_runtime.h>
#include <stdio.h>

__global__ void helloWorldKernel() {
    int blockId = blockIdx.x + blockIdx.y * gridDim.x;
    int threadId = blockId * (blockDim.x * blockDim.y) + (threadIdx.y * blockDim.x) + threadIdx.x;

}

int main() {
    // Configure the number of blocks and threads per block
    dim3 blocks(2, 1);   // Use 2 blocks
    dim3 threads(2, 1);  // Each block has 2 threads

    // Launch the kernel
    helloWorldKernel<<<blocks, threads>>>();

    // Wait for GPU to finish before accessing on host
    cudaDeviceSynchronize();

    // Check for any errors launching the kernel
    cudaError_t cudaStatus = cudaGetLastError();
    if (cudaStatus != cudaSuccess) {
        fprintf(stderr, Kernel launch failed: %sn, cudaGetErrorString(cudaStatus));
        return 1;
    }

    return 0;
}
