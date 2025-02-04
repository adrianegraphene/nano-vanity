use derivation::GenerateKeyType;
use pubkey_matcher::PubkeyMatcher;
use ocl;
use ocl::builders::DeviceSpecifier;
use ocl::builders::ProgramBuilder;
use ocl::flags::MemFlags;
use ocl::Buffer;
use ocl::Platform;
use ocl::ProQue;
use ocl::Result;


#[cfg(feature = "gpu")]
pub use gpu_impl::Gpu;

#[derive(Clone, Copy)]
pub struct GpuOptions<'a> {
    pub platform_idx: usize,
    pub device_idx: usize,
    pub threads: usize,
    pub local_work_size: Option<usize>,
    pub matcher: &'a PubkeyMatcher,
    pub generate_key_type: GenerateKeyType,
}

#[cfg(not(feature = "gpu"))]
pub struct Gpu {
    kernel: ocl::Kernel,
    attempt: Buffer<u8>,
    result: Buffer<u8>,
    root: Buffer<u8>,
}

#[cfg(not(feature = "gpu"))]
impl Gpu {
    pub fn new(_: GpuOptions) -> Result<Gpu, String> {
        eprintln!("GPU support has been disabled at compile time.");
        eprintln!("Rebuild with \"--features gpu\" to enable GPU support.");
        ::std::process::exit(1)
    }

    pub fn compute(&mut self, _: &mut [u8], _: &[u8]) -> Result<bool, String> {
        unreachable!()
    }
}
