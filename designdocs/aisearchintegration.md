# AI Search Integration Design Document

## ML hardware interface
### ONNX standardization
multiple backends 
- ios
- android
- gpu
- cpu

## Models
### OCR
[PaddleOCRv5](https://paddlepaddle.github.io/PaddleOCR/latest/en/version3.x/deployment/obtaining_onnx_models.html
)
#### Needs:
- Download
- export to ONNX
- find smallest bit size we can use
- Quantize as small as it will go
  - standard: cpu 2.4Gb
  - goal: cpu 1Gb
  - Input quantization
    - dynamic range identification
    - Int4?
  - Param quantization
    - [ONNX quant](https://onnxruntime.ai/docs/performance/model-optimizations/quantization.html#quantization-overview)
    - Int4 depending on if the operators support it.
- test that cpu is working on all platforms
  - windows gpu/cpu
  - macos gpu/npu/cpu
  - android npu/cpu
  - ios npu/cpu
- figure of merit / speed test
  - CI for maintinaing minimum speed of ingest

### text-embedding
Model2Vec mistral-potion-multilingual-124M
