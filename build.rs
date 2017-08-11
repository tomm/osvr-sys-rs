extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    // Tell cargo to tell rustc to link the system bzip2
    // shared library.
    println!("cargo:rustc-link-lib=osvrRenderManager");
    println!("cargo:rustc-link-lib=osvrCommon");
    println!("cargo:rustc-link-lib=osvrClient");
    println!("cargo:rustc-link-lib=osvrClientKit");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.hpp")
        .clang_arg("-std=c++11")
        .whitelisted_type("OSVR_ClientContext")
        .whitelisted_type("OSVR_ClientInterface")
        .whitelisted_type("OSVR_GraphicsLibraryOpenGL")
        .whitelisted_type("OSVR_RenderManager")
        .whitelisted_type("OSVR_OpenResultsOpenGL")
        .whitelisted_type("OSVR_RenderBufferOpenGL")
        .whitelisted_type("OSVR_RenderManagerRegisterBufferState")
        .whitelisted_type("OSVR_RenderInfoOpenGL")
        .whitelisted_type("OSVR_ProjectionMatrix")
        .whitelisted_type("OSVR_TimeValue")
        .whitelisted_function("osvrClientInit")
        .whitelisted_function("osvrClientGetInterface")
        .whitelisted_function("osvrRegisterButtonCallback")
        .whitelisted_function("osvrCreateRenderManagerOpenGL")
        .whitelisted_function("osvrRenderManagerOpenDisplayOpenGL")
        .whitelisted_function("osvrDestroyRenderManager")
        .whitelisted_function("osvrClientUpdate")
        .whitelisted_function("osvrRenderManagerGetDefaultRenderParams")
        .whitelisted_function("osvrRenderManagerGetRenderInfoCollection")
        .whitelisted_function("osvrRenderManagerGetNumRenderInfoInCollection")
        .whitelisted_function("osvrRenderManagerStartRegisterRenderBuffers")
        .whitelisted_function("osvrRenderManagerGetRenderInfoFromCollectionOpenGL")
        .whitelisted_function("osvrRenderManagerCreateColorBufferOpenGL")
        .whitelisted_function("osvrRenderManagerCreateDepthBufferOpenGL")
        .whitelisted_function("osvrRenderManagerRegisterRenderBufferOpenGL")
        .whitelisted_function("osvrRenderManagerReleaseRenderInfoCollection")
        .whitelisted_function("osvrRenderManagerFinishRegisterRenderBuffers")
        .whitelisted_function("osvrRenderManagerStartPresentRenderBuffers")
        .whitelisted_function("osvrRenderManagerPresentRenderBufferOpenGL")
        .whitelisted_function("osvrRenderManagerFinishPresentRenderBuffers")
        .whitelisted_function("osvrClientFreeInterface")
        .whitelisted_function("osvrClientShutdown")
        .whitelisted_function("osvrDestroyRenderManager")
        .whitelisted_function("OSVR_Projection_to_OpenGL")
        //.opaque_type("std::basic_string")
        //.hide_type("std::.*")
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
