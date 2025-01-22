wit_bindgen::generate!({
    path: "/home/christian/matrl/componentsv2/wit",
    world: "hellostones",
});

use std::convert::TryInto;

use cm::wasmquiche::boringssl::hellostone;

use super::*;

// NOTE: This structure is copied from <openssl/aead.h> in order to be able to
// statically allocate it. While it is not often modified upstream, it needs to
// be kept in sync.
#[repr(C)]
struct EVP_AEAD_CTX {
    aead: libc::uintptr_t,
    opaque: [u8; 580],
    alignment: u64,
    tag_len: u8,
}

#[allow(unused_variables)]
impl Open {
    pub fn open_with_u64_counter(
        &self, counter: u64, ad: &[u8], buf: &mut [u8],
    ) -> Result<usize> {
        unimplemented!()
    }
}

#[allow(unused_variables)]
impl Seal {
    pub fn seal_with_u64_counter(
        &self, counter: u64, ad: &[u8], buf: &mut [u8], in_len: usize,
        extra_in: Option<&[u8]>,
    ) -> Result<usize> {
        unimplemented!()
    }
}

#[allow(unused_variables)]
pub(crate) struct PacketKey {
    ctx: EVP_AEAD_CTX,
    nonce: Vec<u8>,
}

#[allow(unused_variables)]
impl PacketKey {
    pub fn new(
        alg: Algorithm, key: Vec<u8>, iv: Vec<u8>, _enc: u32,
    ) -> Result<Self> {
        let alg = rust_alg_to_wit(alg);

        let intermediary_res =
            cm::wasmquiche::boringssl::packet_key_new(alg, &key, &iv, _enc);

        match intermediary_res {
            Ok(pk) => Ok(wit_packet_key_to_rust(pk)),
            Err(e) => Err(wit_quic_error_to_rust(e)),
        }
    }

    pub fn from_secret_prk(
        aead: Algorithm, secret_prk: &hkdf::Prk, enc: u32,
    ) -> Result<Self> {
        let aead = rust_alg_to_wit(aead);

        let prk = rust_prk_to_wit(secret_prk);

        let intermediary_res = cm::wasmquiche::boringssl::packet_key_from_secret_pkr(aead, &prk, enc);

        match intermediary_res {
            Ok(pk) => Ok(wit_packet_key_to_rust(pk)),
            Err(e) => Err(wit_quic_error_to_rust(e)),
        }
    }
}

// Helper functions

fn wit_alg_to_rust(alg: cm::wasmquiche::boringssl::Algorithm) -> Algorithm {
    match alg {
        cm::wasmquiche::boringssl::Algorithm::Aes128Gcm => Algorithm::AES128_GCM,
        cm::wasmquiche::boringssl::Algorithm::Aes256Gcm => Algorithm::AES256_GCM,
        cm::wasmquiche::boringssl::Algorithm::Chacha20Ploy1305 => Algorithm::ChaCha20_Poly1305,
    }
}

fn rust_alg_to_wit(alg: Algorithm) -> cm::wasmquiche::boringssl::Algorithm {
    match alg {
        Algorithm::AES128_GCM =>
            cm::wasmquiche::boringssl::Algorithm::Aes128Gcm,
        Algorithm::AES256_GCM =>
            cm::wasmquiche::boringssl::Algorithm::Aes256Gcm,
        Algorithm::ChaCha20_Poly1305 =>
            cm::wasmquiche::boringssl::Algorithm::Chacha20Ploy1305,
    }
}

fn wit_packet_key_to_rust(pk: cm::wasmquiche::boringssl::PacketKey) -> PacketKey {
    PacketKey {
        ctx: wit_evp_aead_ctx_to_rust(pk.ctx),
        nonce: pk.nonce,
    }
}

fn wit_evp_aead_ctx_to_rust(ctx: cm::wasmquiche::boringssl::EvpAeadCtx) -> EVP_AEAD_CTX {
    EVP_AEAD_CTX {
        aead: ctx.aead as usize,
        opaque: ctx.opaque.try_into().unwrap(),
        alignment: ctx.alignment,
        tag_len: ctx.tag_len,
    }
}

fn wit_quic_error_to_rust(err: cm::wasmquiche::boringssl::QuicError) -> Error {
    match err {
        cm::wasmquiche::boringssl::QuicError::Done => Error::Done,
        cm::wasmquiche::boringssl::QuicError::Buffertooshort => Error::BufferTooShort,
        cm::wasmquiche::boringssl::QuicError::Unknownversion => Error::UnknownVersion,
        cm::wasmquiche::boringssl::QuicError::Invalidframe => Error::InvalidFrame,
        cm::wasmquiche::boringssl::QuicError::Invalidpacket => Error::InvalidPacket,
        cm::wasmquiche::boringssl::QuicError::Invalidstate => Error::InvalidState,
        cm::wasmquiche::boringssl::QuicError::Invalidstreamstate(x) => Error::InvalidStreamState(x),
        cm::wasmquiche::boringssl::QuicError::Invalidtransportparam => Error::InvalidTransportParam,
        cm::wasmquiche::boringssl::QuicError::Cryptofail => Error::CryptoFail,
        cm::wasmquiche::boringssl::QuicError::Tlsfail => Error::TlsFail,
        cm::wasmquiche::boringssl::QuicError::Flowcontrol => Error::FlowControl,
        cm::wasmquiche::boringssl::QuicError::Streamlimit => Error::StreamLimit,
        cm::wasmquiche::boringssl::QuicError::Streamstopped(x) => Error::StreamStopped(x),
        cm::wasmquiche::boringssl::QuicError::Streamreset(x) => Error::StreamReset(x),
        cm::wasmquiche::boringssl::QuicError::Finalsize => Error::FinalSize,
        cm::wasmquiche::boringssl::QuicError::Congestioncontrol => Error::CongestionControl,
        cm::wasmquiche::boringssl::QuicError::Idlimit => Error::IdLimit,
        cm::wasmquiche::boringssl::QuicError::Outofidentifiers => Error::OutOfIdentifiers,
        cm::wasmquiche::boringssl::QuicError::Keyupdate => Error::KeyUpdate,
        cm::wasmquiche::boringssl::QuicError::Cryptobufferexceeded => Error::CryptoBufferExceeded,
    }
}

fn rust_prk_to_wit(secret_prk: &hkdf::Prk) -> cm::wasmquiche::boringssl::PseudorandomKey {
    cm::wasmquiche::boringssl::PseudorandomKey {
        key: rust_hmac_key_to_wit(secret_prk.0)
    }
}